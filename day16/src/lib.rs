use std::iter::Sum;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PacketVersion(u8); // three bits
impl PacketVersion {
    pub fn from_iterator<I: Iterator<Item = char>>(iterator: &mut I) -> Option<Self> {
        let digits = iterator.take(3).collect::<String>();
        Some(PacketVersion(u8::from_str_radix(&digits, 2).ok()?))
    }
}
impl Sum<PacketVersion> for u64 {
    fn sum<I: Iterator<Item = PacketVersion>>(iter: I) -> Self {
        iter.fold(0, |acc, next| acc + next.0 as u64)
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LengthType {
    TotalLengthInBits(usize),
    SubpacketCount(usize),
}
impl LengthType {
    pub fn from_iterator<I: Iterator<Item = char>>(iterator: &mut I) -> Self {
        let type_id = iterator.next().expect("This shouldn't be able to fail");
        match type_id {
            '0' => {
                let num_bytes = usize::from_str_radix(&iterator.take(15).collect::<String>(), 2)
                    .expect("This shouldn't be able to fail");
                Self::TotalLengthInBits(num_bytes)
            }
            '1' => {
                let subpacket_count =
                    usize::from_str_radix(&iterator.take(11).collect::<String>(), 2)
                        .expect("This shouldn't be able to fail");
                Self::SubpacketCount(subpacket_count)
            }
            _ => unreachable!(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OperatorType {
    Sum,
    Product,
    Minimum,
    Maximum,
    GreaterThan,
    LessThan,
    EqualTo,
}
impl OperatorType {
    fn from_type_id(type_id: u8) -> Option<Self> {
        match type_id {
            0 => Some(Self::Sum),
            1 => Some(Self::Product),
            2 => Some(Self::Minimum),
            3 => Some(Self::Maximum),
            5 => Some(Self::GreaterThan),
            6 => Some(Self::LessThan),
            7 => Some(Self::EqualTo),
            _ => None,
        }
    }
}

pub struct TypeId(u8); // three bits
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MessageType {
    Literal(u64),
    Operator(LengthType, OperatorType),
}
impl MessageType {
    pub fn from_iterator<I: Iterator<Item = char>>(iterator: &mut I) -> Option<Self> {
        let digits = iterator.take(3).collect::<String>();
        let type_id = u8::from_str_radix(&digits, 2).ok()?;
        match type_id {
            4 => {
                let mut s = String::new();
                loop {
                    if let Some('1') = iterator.next() {
                        s.extend(iterator.take(4));
                    } else {
                        s.extend(iterator.take(4));
                        break;
                    }
                }
                Some(Self::Literal(u64::from_str_radix(&s, 2).ok()?))
            }
            x => {
                let length_type = LengthType::from_iterator(iterator);
                let operator_type = OperatorType::from_type_id(x)?;
                Some(Self::Operator(length_type, operator_type))
            }
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Packet {
    pub version: PacketVersion,
    pub message_type: MessageType,
    pub body: Vec<Packet>,
}

impl Packet {
    pub fn new(s: &str) -> Self {
        Self::from_iterator(&mut s.chars()).unwrap()
    }

    pub fn value(&self) -> u64 {
        match self.message_type {
            MessageType::Literal(v) => v,
            MessageType::Operator(_, op_type) => match op_type {
                OperatorType::Sum => self.subpackets().fold(0, |acc, next| acc + next.value()),
                OperatorType::Product => self.subpackets().fold(1, |acc, next| acc * next.value()),
                OperatorType::Minimum => self.subpackets().fold(u64::MAX, |acc, next| {
                    let value = next.value();
                    if value < acc {
                        value
                    } else {
                        acc
                    }
                }),
                OperatorType::Maximum => self.subpackets().fold(u64::MIN, |acc, next| {
                    let value = next.value();
                    if value > acc {
                        value
                    } else {
                        acc
                    }
                }),
                OperatorType::GreaterThan => {
                    if let &[a, b] = self.subpackets().collect::<Vec<&Packet>>().as_slice() {
                        (a.value() > b.value()).into()
                    } else {
                        panic!("This should provably not happen");
                    }
                }
                OperatorType::LessThan => {
                    if let &[a, b] = self.subpackets().collect::<Vec<&Packet>>().as_slice() {
                        (a.value() < b.value()).into()
                    } else {
                        panic!("This should provably not happen");
                    }
                }
                OperatorType::EqualTo => {
                    if let &[a, b] = self.subpackets().collect::<Vec<&Packet>>().as_slice() {
                        (a.value() == b.value()).into()
                    } else {
                        panic!("This should provably not happen");
                    }
                }
            },
        }
    }

    /// Consume from an iterator until a valid packet is formed, but no further
    /// ```rust
    /// use day16::Packet;
    /// // Two sets of packets
    /// let packets_string = "110100101111111000101110100101111111000101000";
    /// let mut packets_iter = packets_string.chars();
    /// let packet1 = Packet::from_iterator(&mut packets_iter).unwrap();
    /// let packet2 = Packet::from_iterator(&mut packets_iter).unwrap();
    /// # assert_eq!(packets_iter.next(), Some('0'));
    /// # assert_eq!(packets_iter.next(), Some('0'));
    /// # assert_eq!(packets_iter.next(), Some('0'));
    /// # assert_eq!(packets_iter.next(), None);
    /// assert_eq!(packet1, Packet::new("110100101111111000101"));
    /// assert_eq!(packet2, Packet::new("110100101111111000101000"));
    /// ```
    pub fn from_iterator<I: Iterator<Item = char>>(iterator: &mut I) -> Option<Self> {
        let version = PacketVersion::from_iterator(iterator)?;
        let message_type = MessageType::from_iterator(iterator)?;
        let body = match message_type {
            MessageType::Literal(_) => Vec::new(),
            MessageType::Operator(LengthType::TotalLengthInBits(bits), _) => {
                let bytes = &mut iterator
                    .by_ref()
                    .take(bits)
                    .collect::<Vec<char>>()
                    .into_iter();
                let mut subpackets = Vec::new();
                while let Some(packet) = Packet::from_iterator(bytes) {
                    subpackets.push(packet);
                }
                subpackets
            }

            MessageType::Operator(LengthType::SubpacketCount(count), _) => {
                let mut subpackets = Vec::with_capacity(count);
                for _ in 0..count {
                    if let Some(packet) = Packet::from_iterator(iterator) {
                        subpackets.push(packet);
                    }
                }
                subpackets
            }
        };
        Some(Self {
            version,
            message_type,
            body,
        })
    }

    /// Construct from a hex str
    /// ```rust
    /// use day16::Packet;
    /// let packet = Packet::from_hex_str("D2FE28").unwrap();
    /// let expected = Packet::new("110100101111111000101000");
    /// assert_eq!(packet, expected)
    /// ```
    pub fn from_hex_str(hexstr: &str) -> Option<Self> {
        let mapper = |c: char| -> Option<String> { c.to_digit(16).map(|d| format!("{:04b}", d)) };
        let s = hexstr
            .chars()
            .map(mapper)
            .collect::<Option<Vec<String>>>()?
            .join("");
        // let n = u64::from_str_radix(hexstr, 16).ok()?;
        // let mut s = format!("{:b}", n);
        // while s.len() % 4 > 0 {
        //     s = format!("0{}", s);
        // }
        Some(Packet::new(&s))
    }

    /// An iterator over the subpackets
    /// this will probably be replaced by enumerating the Operators
    /// and having them define their own subpackets.
    /// ```rust
    /// use day16::Packet;
    /// // A packet with two subpackets
    /// let packet = Packet::from_hex_str("38006F45291200").unwrap();
    /// let a = Packet::new("11010001010");
    /// let b = Packet::new("0101001000100100");
    /// let expected = vec![
    ///     // references to the packets above...
    /// #   &a,
    /// #   &b,
    /// ];
    /// for (subpacket, expected) in packet.subpackets().zip(expected) {
    ///     assert_eq!(subpacket, expected)
    /// }
    /// ```
    pub fn subpackets(&self) -> impl Iterator<Item = &Packet> {
        self.body.iter()
    }

    /// An iterator over the subpackets and all THEIR subpackets through the tree
    /// ```rust
    /// use day16::Packet;
    /// // A packet with a number of subpackets with children of their own
    /// // packet ( a ( b ( c )))
    /// let packet = Packet::from_hex_str("8A004A801A8002F478").unwrap();
    /// let a = Packet::new("001010100000000001101010000000000000101111010001111000");
    /// let b = Packet::new("101010000000000000101111010001111000");
    /// let c = Packet::new("11010001111000");
    /// let expected = vec![
    ///     &a,
    ///     &b,
    ///     &c,
    /// ].into_iter();
    /// for (got, expect) in packet.traverse_subpackets().zip(expected) {
    ///     assert_eq!(got, expect);
    /// }
    /// ```
    pub fn traverse_subpackets(&self) -> impl Iterator<Item = &Packet> {
        let mut flattened: Vec<&Packet> = Vec::new();
        for subpacket in self.subpackets() {
            flattened.push(subpacket);
            if !subpacket.body.is_empty() {
                flattened.extend(subpacket.traverse_subpackets())
            }
        }
        flattened.into_iter()
    }
}

#[cfg(test)]
mod solve_tests {
    use super::*;

    const INPUT: [&str; 4] = [
        "8A004A801A8002F478",             // op -> op -> op -> lit
        "620080001611562C8802118E34",     // op -> op op -> lit lit lit lit
        "C0015000016115A2E0802F182340",   // op -> op op -> lit lit lit lit
        "A0016C880162017C3686B18A3D4780", // op -> op -> op -> lit lit lit lit lit
    ];

    #[test]
    fn test_sum_versions() {
        fn sum_versions(packet: Packet) -> u64 {
            vec![&packet]
                .into_iter()
                .chain(packet.traverse_subpackets())
                .map(|packet| packet.version)
                .sum::<u64>()
        }

        assert_eq!(sum_versions(Packet::from_hex_str(INPUT[0]).unwrap()), 16);
        assert_eq!(sum_versions(Packet::from_hex_str(INPUT[1]).unwrap()), 12);
        assert_eq!(sum_versions(Packet::from_hex_str(INPUT[2]).unwrap()), 23);
        assert_eq!(sum_versions(Packet::from_hex_str(INPUT[3]).unwrap()), 31);
    }

    #[test]
    fn test_count_packets() {
        fn count_packets(packet: Packet) -> usize {
            vec![&packet]
                .into_iter()
                .chain(packet.traverse_subpackets())
                .count()
        }
        let packet = Packet::from_hex_str(INPUT[0]).expect("Input must parse");
        assert_eq!(count_packets(packet), 4);
        let packet = Packet::from_hex_str(INPUT[1]).expect("Input must parse");
        assert_eq!(count_packets(packet), 7);
        let packet = Packet::from_hex_str(INPUT[2]).expect("Input must parse");
        assert_eq!(count_packets(packet), 7);
        let packet = Packet::from_hex_str(INPUT[3]).expect("Input must parse");
        assert_eq!(count_packets(packet), 8);
    }
}

#[cfg(test)]
mod packet_tests {
    use super::*;

    #[test]
    fn from_hex_str() {
        let packet = Packet::from_hex_str("D2FE28").unwrap();
        let expected = Packet::new("110100101111111000101000");
        assert_eq!(packet, expected);

        let packet = Packet::from_hex_str("38006F45291200").unwrap();
        let expected = Packet::new("00111000000000000110111101000101001010010001001000000000");
        assert_eq!(packet, expected);

        let packet = Packet::from_hex_str("8A004A801A8002F478").unwrap();
        let expected =
            Packet::new("100010100000000001001010100000000001101010000000000000101111010001111000");
        assert_eq!(packet, expected);
    }
    #[test]
    fn from_iterator() {
        let s = String::from("00111000000000000110111101000101001010010001001000000000");
        let iter = &mut s.chars();

        let packet = Packet::from_iterator(iter).unwrap();
        let expected = Packet::new("00111000000000000110111101000101001010010001001000000000");

        assert_eq!(packet, expected);
    }

    #[test]
    fn version_from_iterator() {
        let s = String::from("1104561");
        let iter = &mut s.chars();

        let version = PacketVersion::from_iterator(iter).unwrap();

        assert_eq!(version, PacketVersion(6));
        assert_eq!(iter.next(), Some('4'));
        assert_eq!(iter.next(), Some('5'));
        assert_eq!(iter.next(), Some('6'));
        assert_eq!(iter.next(), Some('1'));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn message_type_literal_from_iterator() {
        let s = String::from("1001100001000000");
        let iter = &mut s.chars();
        let message_type = MessageType::from_iterator(iter).unwrap();
        assert_eq!(message_type, MessageType::Literal(0b10001000));
        assert_eq!(iter.next(), Some('0'));
        assert_eq!(iter.next(), Some('0'));
        assert_eq!(iter.next(), Some('0'));
        assert_eq!(iter.next(), None);
    }
    #[test]
    fn message_type_operator_length_from_iterator() {
        let s = String::from("11000000000000010111101");
        let iter = &mut s.chars();
        let message_type = MessageType::from_iterator(iter).unwrap();
        assert_eq!(
            message_type,
            MessageType::Operator(LengthType::TotalLengthInBits(11), OperatorType::LessThan)
        );
        assert_eq!(iter.next(), Some('1'));
        assert_eq!(iter.next(), Some('1'));
        assert_eq!(iter.next(), Some('0'));
        assert_eq!(iter.next(), Some('1'));
        assert_eq!(iter.next(), None);
    }
    #[test]
    fn message_type_operator_count_from_iterator() {
        let s = String::from("1101000000010111101");
        let iter = &mut s.chars();
        let message_type = MessageType::from_iterator(iter).unwrap();
        assert_eq!(
            message_type,
            MessageType::Operator(LengthType::SubpacketCount(11), OperatorType::LessThan)
        );
        assert_eq!(iter.next(), Some('1'));
        assert_eq!(iter.next(), Some('1'));
        assert_eq!(iter.next(), Some('0'));
        assert_eq!(iter.next(), Some('1'));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn value_literal() {
        let packet = Packet::new("1101001000001000");
        assert_eq!(packet.value(), 8);
    }

    #[test]
    fn value_sum() {
        let packet = Packet::new("11000010000000001011010010000010001101001000001000");
        //                                    2SUBPACKETS   ^---LIT 8------^^-----LIT 8----^
        assert_eq!(packet.value(), 16);
        let packet = Packet::from_hex_str("C200B40A82").expect("Input must parse");
        // + 1 2
        assert_eq!(packet.value(), 3)
    }
    #[test]
    fn value_product() {
        let packet = Packet::new("11000110000000001011010010000010001101001000001000");
        //                                    2SUBPACKETS   ^---LIT 8------^^-----LIT 8----^
        assert_eq!(packet.value(), 64);
        let packet = Packet::from_hex_str("04005AC33890").expect("Input must parse");
        // * 6 9
        assert_eq!(packet.value(), 54);
    }
    #[test]
    fn value_minimum() {
        let packet = Packet::new("11001010000000001011010010000001001101001000001000");
        //                                    2SUBPACKETS   ^---LIT 4------^^-----LIT 8----^
        assert_eq!(packet.value(), 4);
        let packet = Packet::from_hex_str("880086C3E88112").expect("Input must parse");
        // min 7 8 9
        assert_eq!(packet.value(), 7);
    }
    #[test]
    fn value_maximum() {
        let packet = Packet::new("11001110000000001011010010000001001101001000001000");
        //                                    2SUBPACKETS   ^---LIT 4------^^-----LIT 8----^
        assert_eq!(packet.value(), 8);
        let packet = Packet::from_hex_str("CE00C43D881120").expect("Input must parse");
        // max 7 8 9
        assert_eq!(packet.value(), 9);
    }
    #[test]
    fn value_greater_than() {
        let packet = Packet::new("11010110000000001011010010000001001101001000001000");
        //                                    2SUBPACKETS   ^---LIT 4------^^-----LIT 8----^
        assert_eq!(packet.value(), 0);
        let packet = Packet::from_hex_str("D8005AC2A8F0").expect("Input must parse");
        // < 5 15
        assert_eq!(packet.value(), 1);
    }
    #[test]
    fn value_less_than() {
        let packet = Packet::new("11011010000000001011010010000001001101001000001000");
        //                                    2SUBPACKETS   ^---LIT 4------^^-----LIT 8----^
        assert_eq!(packet.value(), 1);
        let packet = Packet::from_hex_str("F600BC2D8F").expect("Input must parse");
        // > 5 15
        assert_eq!(packet.value(), 0);
    }
    #[test]
    fn value_equal_to() {
        let packet = Packet::new("11011110000000001011010010000001001101001000001000");
        //                                    2SUBPACKETS   ^---LIT 4------^^-----LIT 8----^
        assert_eq!(packet.value(), 0);
        let packet = Packet::from_hex_str("9C005AC2F8F0").expect("Input must parse");
        // = 5 15
        assert_eq!(packet.value(), 0);
        let packet = Packet::from_hex_str("9C0141080250320F1802104A08").expect("Input must parse");
        // = (+ 1 3) (* 2 2)
        assert_eq!(packet.value(), 1);
    }
}
