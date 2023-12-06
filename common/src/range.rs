use num::{Num, One};
use std::ops::{AddAssign, RangeInclusive, SubAssign};

pub struct MultiRange<T> {
    ranges: Vec<RangeInclusive<T>>,
}

impl<T> AddAssign<&RangeInclusive<T>> for MultiRange<T>
where
    T: Num + Ord + Copy,
{
    fn add_assign(&mut self, other: &RangeInclusive<T>) {
        let index = self
            .ranges
            .iter()
            .position(|r| r.contains(other.start()) || r.contains(other.end()));
        match index {
            Some(index) => {
                let r = &mut self.ranges[index];
                let r_start = *r.start();
                let r_end = *r.end();
                let other_start = *other.start();
                let other_end = *other.end();
                *r = r_start.min(other_start)..=r_end.max(other_end);
            }
            None => {
                self.ranges.push(other.clone());
            }
        }
        self.ranges.sort_unstable_by_key(|r| *r.start());
    }
}

impl<T> SubAssign<&RangeInclusive<T>> for MultiRange<T>
where
    T: Num + Ord + Copy,
{
    fn sub_assign(&mut self, other: &RangeInclusive<T>) {
        let mut index = 0;
        while index < self.ranges.len() {
            let r = &mut self.ranges[index];
            let r_start = *r.start();
            let r_end = *r.end();
            let other_start = *other.start();
            let other_end = *other.end();
            match (r.contains(other.start()), r.contains(other.end())) {
                (true, true) => {
                    // other is contained in r, split r into two ranges
                    // r       => |--------------------|
                    // other   =>     |--------|
                    // result1 => |---|
                    // result2 =>              |-------|
                    let r1 = r_start..=(other_start - One::one());
                    let r2 = (other_end + One::one())..=r_end;
                    self.ranges.remove(index);
                    self.ranges.push(r1);
                    self.ranges.push(r2);
                }
                (true, false) => {
                    // other.start() is contained in r, truncate r
                    // r      => |--------------------|
                    // other  =>     |------------------|
                    // result => |---|
                    *r = r_start..=(other_start - One::one());
                }
                (false, true) => {
                    // other.end() is contained in r, truncate r
                    //     r  =>          |------------|
                    // other  =>       |--------|
                    // result =>                 |-----|
                    *r = (other_end + One::one())..=r_end;
                }
                (false, false) => {
                    if r_start > other_start && r_end < other_end {
                        // r is contained in other, remove r
                        // r      =>     |---|
                        // other  => |-----------|
                        // result => None
                        self.ranges.remove(index);
                        continue; // skip index += 1
                    }
                    // If we got here, other is not contained in r, do nothing
                    // r      => |---|
                    // other  =>         |--------|
                    // result => |---|
                }
            }
            index += 1;
        }
        self.ranges = self
            .ranges
            .iter()
            .filter(|r| r.start() <= r.end())
            .cloned()
            .collect();
        self.ranges.sort_unstable_by_key(|r| *r.start());
    }
}

impl<T> MultiRange<T> {
    pub fn new() -> Self {
        Self { ranges: vec![] }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_inclusive_add() {
        let a = 1..=10;
        let b = 5..=15;
        let mut m = MultiRange::new();
        m += &a;
        m += &b;
        assert_eq!(m.ranges.len(), 1);
        assert_eq!(m.ranges[0], 1..=15);
        let a = 6..=10;
        m += &a;
        assert_eq!(m.ranges.len(), 1);
        assert_eq!(m.ranges[0], 1..=15);
        let a = 100..=102;
        m += &a;
        assert_eq!(m.ranges.len(), 2);
        assert_eq!(m.ranges[0], 1..=15);
        assert_eq!(m.ranges[1], 100..=102);
        let a = 20..=30;
        m += &a;
        assert_eq!(m.ranges.len(), 3);
        assert_eq!(m.ranges[0], 1..=15);
        assert_eq!(m.ranges[1], 20..=30);
        assert_eq!(m.ranges[2], 100..=102);
        let a = 22..=35;
        m += &a;
        assert_eq!(m.ranges.len(), 3);
        assert_eq!(m.ranges[0], 1..=15);
        assert_eq!(m.ranges[1], 20..=35);
        assert_eq!(m.ranges[2], 100..=102);
    }

    #[test]
    fn test_inclusive_sub() {
        let a = 1..=15;
        let mut m = MultiRange::new();
        m += &a;
        let a = 6..=10;
        m -= &a;
        assert_eq!(m.ranges.len(), 2);
        assert_eq!(m.ranges[0], 1..=5);
        assert_eq!(m.ranges[1], 11..=15);
        let a = 100..=102;
        m -= &a;
        assert_eq!(m.ranges.len(), 2);
        assert_eq!(m.ranges[0], 1..=5);
        assert_eq!(m.ranges[1], 11..=15);
        let a = 4..=12;
        m -= &a;
        assert_eq!(m.ranges.len(), 2);
        assert_eq!(m.ranges[0], 1..=3);
        assert_eq!(m.ranges[1], 13..=15);
        let mut m2 = MultiRange::new();
        for r in &m.ranges {
            m2 += r;
        }
        let a = 1..=15;
        let b = 0..=16;
        m -= &a;
        m2 -= &b;
        assert_eq!(m.ranges.len(), 0, "Exactly empty");
        assert_eq!(m2.ranges.len(), 0, "Over empty");
    }
}
