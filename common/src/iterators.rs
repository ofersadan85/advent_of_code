#[derive(Debug, Clone)]
pub struct BitsIterator {
    data: Vec<u8>,
    index: usize,
    used: usize,
}

impl Iterator for BitsIterator {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        self.consume(1)
    }
}

impl FromIterator<u8> for BitsIterator {
    fn from_iter<I: IntoIterator<Item = u8>>(iter: I) -> Self {
        Self::new(iter.into_iter().collect())
    }
}

impl std::str::FromStr for BitsIterator {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // Process two characters at a time as hexadecimals
        let mut data = Vec::new();
        let mut current = 0;
        for (i, c) in s.trim().chars().enumerate() {
            let value = match c {
                '0'..='9' => c as u8 - b'0',
                'a'..='f' => c as u8 - b'a' + 10,
                'A'..='F' => c as u8 - b'A' + 10,
                _ => return Err("Invalid character"),
            };
            let is_last = i == s.len() - 1;
            if i % 2 == 1 || is_last {
                current |= usize::from(value);
                data.push(u8::try_from(current).expect("Invalid byte"));
                current = 0;
            } else {
                current = usize::from(value) << 4;
            }
        }
        Ok(Self::new(data))
    }
}

impl BitsIterator {
    pub const fn new(data: Vec<u8>) -> Self {
        Self {
            data,
            index: 0,
            used: 0,
        }
    }

    pub const fn len(&self) -> usize {
        self.data.len() * 8
    }

    pub const fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    pub const fn remaining(&self) -> usize {
        (self.data.len() - self.index) * 8 - self.used
    }

    pub fn consume(&mut self, n: usize) -> Option<usize> {
        let mut result = 0;
        let mut remaining = n.min(self.remaining());
        if remaining == 0 {
            return None;
        }
        while remaining > 0 {
            if self.index >= self.data.len() {
                return None;
            }
            let will_use = (8 - self.used).min(remaining);
            result <<= will_use;
            // dbg!(n, remaining, will_use, self.used, self.index);
            remaining = self.remaining().min(remaining - will_use);
            let shift = 8 - will_use - self.used;
            let mask = ((1_usize.overflowing_shl(u32::try_from(will_use).ok()?)).0 - 1) << shift;
            let current = self
                .data
                .get(self.index)
                .map(|b| (usize::from(*b) & mask) >> shift)?;
            // dbg!(
            //     "Idx:{} Used:{} Will:{} Rem:{} Mask:{:08b} Bit: {:08b} => {:08b}",
            //     self.index, self.used, will_use, remaining, mask, self.data[self.index], current
            // );
            result |= current;
            self.used += will_use;
            while self.used >= 8 {
                self.index += 1;
                self.used -= 8;
            }
        }
        Some(result)
    }

    pub fn consume_until_end(&mut self) -> usize {
        self.consume(self.remaining()).unwrap_or(0)
    }

    pub const fn reset(&mut self) {
        self.index = 0;
        self.used = 0;
    }

    pub fn as_bytes(&self) -> &[u8] {
        &self.data
    }

    pub fn unconsumed_bytes(&mut self, limit: Option<usize>) -> (Vec<u8>, usize) {
        let mut result = vec![];
        while self.remaining() >= 8 {
            result.extend(self.consume(8).and_then(|b| u8::try_from(b).ok()));
            if result.len() >= limit.unwrap_or(usize::MAX) {
                break;
            }
        }
        (result, self.remaining())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn iter_bits() {
        let mut b = BitsIterator::new(vec![0b1010_1010, 0b0101_0101]);
        let v: Vec<_> = b.clone().collect();
        assert_eq!(v, vec![1, 0, 1, 0, 1, 0, 1, 0, 0, 1, 0, 1, 0, 1, 0, 1]);
        assert_eq!(b.consume(3), Some(5));
        assert_eq!(b.consume(3), Some(2));
        assert_eq!(b.consume_until_end(), 597);
        assert_eq!(b.remaining(), 0, "No more data");
        assert_eq!(b.consume_until_end(), 0, "No more data");
    }

    #[test]
    fn parse_hex() {
        assert!("a5a5aq".parse::<BitsIterator>().is_err(), "Bad character");
        assert!(
            "Abc123"
                .parse::<BitsIterator>()
                .is_ok_and(|b| b.data.len() == 3),
            "Mixed case"
        );
        let mut b: BitsIterator = "abcd1234".parse().unwrap();
        assert_eq!(b.consume(8), Some(0xab));
        assert_eq!(b.consume(16), Some(0xcd12));
        assert_eq!(b.consume_until_end(), 0x34);
        assert_eq!(b.remaining(), 0, "No more data");
        assert_eq!(b.consume_until_end(), 0, "No more data");
    }

    #[test]
    fn parse_hex_odd() {
        let mut b: BitsIterator = "F".parse().unwrap();
        assert_eq!(b.consume(8), Some(0x0F));
        let mut b: BitsIterator = "00F".parse().unwrap();
        assert_eq!(b.consume(8), Some(0));
        assert_eq!(b.consume(8), Some(0x0F));
    }

    #[test]
    fn consume_mid_byte() {
        let mut b: BitsIterator = "FFFF".parse().unwrap();
        assert_eq!(b.consume(7), Some(0b1111111));
        assert_eq!(b.consume(4), Some(0b1111));
        assert_eq!(b.consume_until_end(), 0b11111);
        assert_eq!(b.remaining(), 0, "No more data");
        assert_eq!(b.consume_until_end(), 0, "No more data");
    }

    #[test]
    fn over_consumed() {
        let mut b: BitsIterator = "FF".parse().unwrap();
        assert_eq!(b.consume(9), Some(0b11111111));
        assert_eq!(b.remaining(), 0, "No more data");
        assert_eq!(b.consume_until_end(), 0, "No more data");
        let mut b: BitsIterator = "FFFF".parse().unwrap();
        assert_eq!(b.consume(9), Some(0b111111111));
        assert_eq!(b.consume(9), Some(0b1111111));
        assert_eq!(b.remaining(), 0, "No more data");
        assert_eq!(b.consume_until_end(), 0, "No more data");
    }
}
