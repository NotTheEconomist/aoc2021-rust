use std::{
    collections::{HashSet, VecDeque},
    fmt::Debug,
    ops::Add,
    str::FromStr,
};

mod parser;

#[derive(Clone, PartialEq, Eq)]
pub enum SnailFish {
    Num(u32),
    Pair(Box<(SnailFish, SnailFish)>),
}

impl Debug for SnailFish {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Num(n) => write!(f, "{}", n),
            Self::Pair(ref bx) => write!(f, "[{:?}, {:?}]", bx.0, bx.1),
        }
    }
}

impl Add<SnailFish> for SnailFish {
    type Output = Self;

    fn add(self, rhs: SnailFish) -> Self::Output {
        Self::Pair(Box::new((self, rhs))).reduce()
    }
}

impl FromStr for SnailFish {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match parser::root(s) {
            Ok((_, snailfish)) => Ok(snailfish),
            Err(e) => Err(e.to_string()),
        }
    }
}

impl SnailFish {
    pub fn pair(a: SnailFish, b: SnailFish) -> Self {
        Self::Pair(Box::new((a, b)))
    }
    pub fn num(n: u32) -> Self {
        Self::Num(n)
    }
    fn try_add_value(&mut self, rhs: u32) -> Option<&Self> {
        match self {
            Self::Num(n) => {
                *n += rhs;
                Some(self)
            }
            Self::Pair(_) => None,
        }
    }
    fn recurse_left_mut(&mut self) -> &mut Self {
        let mut cur = self;
        while let SnailFish::Pair(ref mut bx) = cur {
            cur = &mut bx.0;
        }
        cur
    }
    fn recurse_right_mut(&mut self) -> &mut Self {
        let mut cur = self;
        while let SnailFish::Pair(ref mut bx) = cur {
            cur = &mut bx.1;
        }
        cur
    }

    /// Recursively calculates the magnitude of a SnailFish
    /// ```rust
    /// # use day18::*;
    /// let s = SnailFish::pair(
    ///     SnailFish::pair(
    ///         SnailFish::num(1),
    ///         SnailFish::num(2)
    ///     ),
    ///     SnailFish::pair(
    ///         SnailFish::pair(SnailFish::num(3), SnailFish::num(4)),
    ///         SnailFish::num(5)
    ///     )
    /// );
    /// // ((1, 2), ((3, 4), 5))
    /// assert_eq!(s.magnitude(), 143)
    /// ```
    pub fn magnitude(&self) -> u64 {
        match self {
            SnailFish::Num(x) => *x as u64,
            SnailFish::Pair(ref bx) => {
                let (left, right) = bx.as_ref();
                left.magnitude() * 3 + right.magnitude() * 2
            }
        }
    }

    /// Split a number in two if it's valid to do so.
    /// SnailFish numbers split in two when they are single
    /// numbers >= 10 (pairs never split and nums < 10 never split)
    /// A split number d is replaced by the pair (x, y) where
    /// x and y are halves of d. If d is odd, y = x+1
    /// ```rust
    /// # use day18::*;
    /// let mut s = SnailFish::num(11);
    /// let res = s.split();
    /// assert_eq!(s, SnailFish::pair(SnailFish::num(5), SnailFish::num(6)));
    /// assert_eq!(res, true)
    /// ```
    ///
    /// split returns true if it took an action and false if it did not
    pub fn split(&mut self) -> bool {
        match self {
            Self::Num(d) if *d >= 10 => {
                let (left, mut right) = (*d / 2, *d / 2);
                if left + right < *d {
                    right += 1;
                }
                *self = Self::pair(SnailFish::num(left), SnailFish::num(right));
                true
            }
            _ => false,
        }
    }
    pub fn reduce(mut self) -> Self {
        'outer: loop {
            // if anything can explode, go back to the start
            if self.explode() {
                continue 'outer;
            }

            for snailfish in self.iter_mut() {
                if snailfish.split() {
                    // if anything can split, go back to the start
                    continue 'outer;
                }
            }
            break;
        }
        self
    }

    pub fn explode(&mut self) -> bool {
        // We use raw pointers here because we want to pass around what are (effectively) multiple mutable references
        // to the same object. Since we have eclusive access to self (via the &mut self reference) and never read a
        // value after modifying it, this is safe.
        let mut queue = VecDeque::from([(0, self as *mut SnailFish, Vec::new())]);

        /// Takes a mutable raw pointer and returns the *mut to the rightmost Snailfish
        unsafe fn get_rightmost_value_from(
            this: *mut SnailFish,
            mut parents: Vec<*mut SnailFish>,
        ) -> Option<*mut SnailFish> {
            let mut seen = HashSet::new();
            while let Some(parent) = parents.pop() {
                if let SnailFish::Pair(ref mut bx) = *parent {
                    if this == &mut bx.1 || seen.iter().any(|&p| p == &mut bx.1 as *mut SnailFish) {
                        seen.insert(parent);
                        continue;
                    } else {
                        let target = &mut bx.1;
                        return Some(target.recurse_left_mut() as *mut SnailFish);
                    }
                }
            }
            None
        }
        /// Takes a mutable raw pointer and returns the *mut to the leftmost Snailfish
        unsafe fn get_leftmost_value_from(
            this: *mut SnailFish,
            mut parents: Vec<*mut SnailFish>,
        ) -> Option<*mut SnailFish> {
            let mut seen = HashSet::new();
            while let Some(parent) = parents.pop() {
                if let SnailFish::Pair(ref mut bx) = *parent {
                    if this == &mut bx.0 || seen.iter().any(|&p| p == &mut bx.0 as *mut SnailFish) {
                        seen.insert(parent);
                        continue;
                    } else {
                        let target = &mut bx.0;
                        return Some(target.recurse_right_mut() as *mut SnailFish);
                    }
                }
            }
            None
        }

        while let Some((depth, cur, parents)) = queue.pop_front() {
            if depth >= 4 {
                unsafe {
                    // we only need to explode if we're in a pair
                    if let SnailFish::Num(_) = *cur {
                        continue;
                    }
                    let left_val = if let SnailFish::Pair(ref bx) = *cur {
                        if let SnailFish::Num(n) = bx.0 {
                            Some(n)
                        } else {
                            None
                        }
                    } else {
                        None
                    };
                    let right_val = if let SnailFish::Pair(ref bx) = *cur {
                        if let SnailFish::Num(n) = bx.1 {
                            Some(n)
                        } else {
                            None
                        }
                    } else {
                        None
                    };
                    let leftmost = get_leftmost_value_from(cur, parents.clone());
                    let rightmost = get_rightmost_value_from(cur, parents);
                    if let (Some(left_snail), Some(left_val)) = (leftmost, left_val) {
                        (*left_snail)
                            .try_add_value(left_val)
                            .expect("left_snail should always be a Num, so this shouldn't fail");
                    }
                    if let (Some(right_snail), Some(right_val)) = (rightmost, right_val) {
                        (*right_snail)
                            .try_add_value(right_val)
                            .expect("right_snail should always be a Num, so this shouldn't fail");
                    }
                    *cur = SnailFish::Num(0);
                    return true;
                }
            } else {
                unsafe {
                    if let SnailFish::Pair(ref mut bx) = *cur {
                        let mut new_parents = parents.clone();
                        new_parents.push(cur);
                        queue.push_back((
                            depth + 1,
                            &mut bx.0 as *mut SnailFish,
                            new_parents.clone(),
                        ));
                        queue.push_back((depth + 1, &mut bx.1 as *mut SnailFish, new_parents));
                    }
                }
            }
        }
        false
    }

    /// Produce all the numbers out of a SnailFish
    pub fn iter(&self) -> impl Iterator<Item = &SnailFish> {
        let mut acc = Vec::new(); // We'll return this one
        let mut stack = vec![self]; // All the pairs we haven't iterated through yet

        while let Some(snail) = stack.pop() {
            match snail {
                Self::Num(_) => acc.push(snail),
                Self::Pair(ref bx) => {
                    let (ref a, ref b) = **bx;
                    stack.push(a);
                    stack.push(b);
                }
            }
        }

        acc.into_iter().rev()
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut SnailFish> {
        let mut acc = Vec::new(); // We'll return this one
        let mut stack = vec![self]; // All the pairs we haven't iterated through yet

        while let Some(snail) = stack.pop() {
            match snail {
                Self::Num(_) => acc.push(snail),
                Self::Pair(bx) => {
                    let (ref mut a, ref mut b) = **bx;
                    stack.push(a);
                    stack.push(b);
                }
            }
        }

        acc.into_iter().rev()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn snailfish_iter() {
        let snailfish = SnailFish::pair(
            SnailFish::pair(SnailFish::num(1), SnailFish::num(2)),
            SnailFish::pair(
                SnailFish::pair(SnailFish::num(3), SnailFish::num(4)),
                SnailFish::num(5),
            ),
        );

        let values: Vec<&SnailFish> = snailfish.iter().collect();
        assert_eq!(
            values,
            vec![
                &SnailFish::num(1),
                &SnailFish::num(2),
                &SnailFish::num(3),
                &SnailFish::num(4),
                &SnailFish::num(5)
            ]
        )
    }

    #[test]
    fn test_snailfish_split() {
        let mut s = SnailFish::num(10);
        s.split();
        let expected = "[5, 5]".parse::<SnailFish>().expect("expected must parse");
        assert_eq!(s, expected);

        // odd numbers should be split (small, large) s.t. small + large = total
        let mut s = SnailFish::num(11);
        s.split();
        let expected = "[5, 6]".parse::<SnailFish>().expect("expected must parse");
        assert_eq!(s, expected);

        // split is not expected to operato multiple times
        let mut s = SnailFish::num(21);
        s.split();
        let expected = "[10, 11]"
            .parse::<SnailFish>()
            .expect("expected must parse");
        assert_eq!(s, expected);
    }

    #[test]
    fn test_snailfish_explode() {
        let mut s = "[[[[[9,8],1],2],3],4]"
            .parse::<SnailFish>()
            .expect("given must parse");

        let did_explode = s.explode();
        assert!(did_explode);

        let expected = "[[[[0,9],2],3],4]"
            .parse::<SnailFish>()
            .expect("expected must parse");
        assert_eq!(s, expected);

        let mut s = "[[[[1, [9, 8]], 2], 3], 4]"
            .parse::<SnailFish>()
            .expect("given must parse");
        let did_explode = s.explode();
        assert!(did_explode);
        let expected = "[[[[10, 0], 10], 3], 4]"
            .parse::<SnailFish>()
            .expect("expected must parse");
        assert_eq!(s, expected);
    }

    #[test]
    fn test_magnitude() {
        let s = "[[1,2],[[3,4],5]]"
            .parse::<SnailFish>()
            .expect("given must parse");
        assert_eq!(s.magnitude(), 143);
        let s = "[[[[0,7],4],[[7,8],[6,0]]],[8,1]]"
            .parse::<SnailFish>()
            .expect("given must parse");
        assert_eq!(s.magnitude(), 1384);
        let s = "[[[[1,1],[2,2]],[3,3]],[4,4]]"
            .parse::<SnailFish>()
            .expect("given must parse");
        assert_eq!(s.magnitude(), 445);
        let s = "[[[[3,0],[5,3]],[4,4]],[5,5]]"
            .parse::<SnailFish>()
            .expect("given must parse");
        assert_eq!(s.magnitude(), 791);
        let s = "[[[[5,0],[7,4]],[5,5]],[6,6]]"
            .parse::<SnailFish>()
            .expect("given must parse");
        assert_eq!(s.magnitude(), 1137);
        let s = "[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]"
            .parse::<SnailFish>()
            .expect("given must parse");
        assert_eq!(s.magnitude(), 3488);
    }

    #[test]
    fn test_reduce() {
        let mut acc = "[[[0,[4,5]],[0,0]],[[[4,5],[2,6]],[9,5]]]"
            .parse::<SnailFish>()
            .unwrap();
        let next = "[7,[[[3,7],[4,3]],[[6,3],[8,8]]]]"
            .parse::<SnailFish>()
            .unwrap();
        acc = acc + next;
        // [[[[0, [4, 5]], [0, 0]], [[[4, 5], [2, 6]], [9, 5]]], [7, [[[3, 7], [4, 3]], [[6, 3], [8, 8]]]]]
        assert_eq!(
            acc,
            "[[[[4,0],[5,4]],[[7,7],[6,0]]],[[8,[7,7]],[[7,9],[5,0]]]]"
                .parse::<SnailFish>()
                .unwrap()
        );
    }
}
