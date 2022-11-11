use std::ops::{Add, AddAssign};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TargetZone {
    pub bottom_left: Point,
    pub top_right: Point,
}

impl TargetZone {
    /// Decides whether or not a point is inside the zone
    pub fn contains(&self, point: &Point) -> bool {
        let (min_x, max_x) = (self.bottom_left.x, self.top_right.x);
        let (min_y, max_y) = (self.bottom_left.y, self.top_right.y);
        min_x <= point.x && point.x <= max_x && min_y <= point.y && point.y <= max_y
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

impl Point {
    #[allow(clippy::result_unit_err)]
    pub fn try_apply_vector(&mut self, vector: &mut Vector) -> Result<(), ()> {
        self.x = self.x.checked_add(vector.x).ok_or(())?;
        self.y = self.y.checked_add(vector.y).ok_or(())?;
        vector.degrade();
        Ok(())
    }

    pub fn apply_vector(&mut self, vector: &mut Vector) {
        self.x += vector.x;
        self.y += vector.y;
        vector.degrade();
    }
}
impl AddAssign<Vector> for Point {
    fn add_assign(&mut self, rhs: Vector) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}
impl Add<Vector> for Point {
    type Output = Self;

    fn add(self, rhs: Vector) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Vector {
    pub x: i32,
    pub y: i32,
}

impl Vector {
    pub fn degrade(&mut self) {
        self.y -= 1;
        if self.x > 0 {
            self.x -= 1;
        }
    }
}

pub fn has_past(point: &Point, vec: &Vector, target_zone: &TargetZone) -> bool {
    match *vec {
        // off the bottom
        Vector { x: _, y } if y <= 0 && point.y < target_zone.bottom_left.y => true,
        // off the left
        Vector { x, y: _ } if x < 0 && point.x < target_zone.bottom_left.x => true,
        // off the righ
        Vector { x, y: _ } if x > 0 && point.x > target_zone.top_right.x => true,
        // not moving horizontally, but not in the zone on the x-axis
        Vector { x: 0, y: _ } => {
            point.x < target_zone.bottom_left.x || point.x > target_zone.top_right.x
        }
        // all other movements might still hit
        _ => false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn has_past() {
        let target_zone = TargetZone {
            bottom_left: Point { x: 20, y: -10 },
            top_right: Point { x: 30, y: -5 },
        };

        let left = Vector { x: -1, y: 0 };
        let up_left = Vector { x: -1, y: 1 };
        let up = Vector { x: 0, y: 1 };
        let up_right = Vector { x: 1, y: 1 };
        let right = Vector { x: 1, y: 0 };
        let down_right = Vector { x: 1, y: -1 };
        let down = Vector { x: 0, y: -1 };
        let down_left = Vector { x: -1, y: -1 };
        let directions = [
            left, up_left, up, up_right, right, down_right, down, down_left,
        ];

        let test_all_directions = move |point: &Point, expected: [bool; 8]| {
            for (vec, expected) in directions.into_iter().zip(expected) {
                assert_eq!(
                    super::has_past(point, &vec, &target_zone),
                    expected,
                    "{:?} moving {:?} should be {} but wasn't",
                    point,
                    vec,
                    expected
                );
            }
        };

        // left
        let point = Point { x: 15, y: -7 };
        test_all_directions(&point, [true, true, true, false, false, false, true, true]);
        // top-left
        let point = Point { x: 15, y: -2 };
        test_all_directions(&point, [true, true, true, false, false, false, true, true]);
        // top
        let point = Point { x: 25, y: -2 };
        test_all_directions(
            &point,
            [false, false, false, false, false, false, false, false],
        );
        // top-right
        let point = Point { x: 35, y: -2 };
        test_all_directions(&point, [false, false, true, true, true, true, true, false]);
        // right
        let point = Point { x: 35, y: -7 };
        test_all_directions(&point, [false, false, true, true, true, true, true, false]);
        // bottom_right
        let point = Point { x: 35, y: -13 };
        test_all_directions(&point, [true, false, true, true, true, true, true, true]);
        // bottom
        let point = Point { x: 25, y: -13 };
        test_all_directions(&point, [true, false, false, false, true, true, true, true]);
        // bottom_left
        let point = Point { x: 15, y: -13 };
        test_all_directions(&point, [true, true, true, false, true, true, true, true]);
    }

    #[test]
    fn target_zone_contains() {
        let target_zone = TargetZone {
            bottom_left: Point { x: 20, y: -10 },
            top_right: Point { x: 30, y: -5 },
        };

        for y in -10..=-5 {
            for x in 20..=30 {
                assert!(target_zone.contains(&Point { x, y }))
            }
        }

        assert!(!target_zone.contains(&Point { x: 19, y: -7 }));
        assert!(!target_zone.contains(&Point { x: 25, y: -4 }));
    }
    #[test]
    fn apply_vector() {
        let mut pos = Point { x: 0, y: 0 };
        let mut vector = Vector { x: 2, y: 2 };
        pos.apply_vector(&mut vector);
        assert_eq!(pos, Point { x: 2, y: 2 });
        pos.apply_vector(&mut vector);
        assert_eq!(pos, Point { x: 3, y: 3 });
        pos.apply_vector(&mut vector);
        assert_eq!(pos, Point { x: 3, y: 3 });
        pos.apply_vector(&mut vector);
        assert_eq!(pos, Point { x: 3, y: 2 });
        pos.apply_vector(&mut vector);
        assert_eq!(pos, Point { x: 3, y: 0 });
        pos.apply_vector(&mut vector);
        assert_eq!(pos, Point { x: 3, y: -3 });
    }
    #[test]
    fn degrade_vector() {
        let mut vector = Vector { x: 3, y: 4 };
        vector.degrade();
        assert_eq!(vector.x, 2);
        assert_eq!(vector.y, 3);
        vector.degrade();
        assert_eq!(vector.x, 1);
        assert_eq!(vector.y, 2);
        vector.degrade();
        assert_eq!(vector.x, 0);
        assert_eq!(vector.y, 1);
        vector.degrade();
        assert_eq!(vector.x, 0);
        assert_eq!(vector.y, 0);
        vector.degrade();
        assert_eq!(vector.x, 0);
        assert_eq!(vector.y, -1);
    }
}
