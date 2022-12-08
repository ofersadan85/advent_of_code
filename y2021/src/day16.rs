use std::ops::Add;

#[derive(Debug, PartialEq, Eq)]
enum PacketType {
    Empty,
    Literal,
    OperatorUnknown,
}

#[derive(Debug, PartialEq, Eq)]
struct Packet {
    raw_binary: String,
    version: usize,
    packet_type: PacketType,
    sub_packets: Vec<Packet>,
}

#[derive(Debug, PartialEq, Eq)]
struct PacketSummary {
    values: Vec<usize>,
    length: usize,
    version: usize,
}

impl Add for PacketSummary {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let mut values = self.values.clone();
        values.append(&mut rhs.values.clone());
        Self {
            values,
            length: self.length + rhs.length,
            version: self.version + rhs.version,
        }
    }
}

impl Packet {
    fn from_hex(hex: &str) -> Self {
        let mut raw_binary = String::new();
        let hex_vec: Vec<char> = hex.chars().collect();
        for i in (0..hex_vec.len()).step_by(2) {
            let small_hex = hex.get(i..i + 2).unwrap();
            let chunk = u8::from_str_radix(small_hex, 16).unwrap();
            raw_binary += &format!("{chunk:0>8b}");
        }
        Self::from_binary(&raw_binary)
    }

    fn from_binary(raw_binary: &str) -> Self {
        let raw_binary = raw_binary.to_string();
        let version = usize::from_str_radix(&raw_binary[0..3], 2).unwrap();
        let packet_type = match u8::from_str_radix(&raw_binary[3..6], 2).unwrap() {
            4 => PacketType::Literal,
            _ => PacketType::OperatorUnknown,
        };
        let sub_packets = if packet_type == PacketType::Literal {
            vec![]
        } else if &raw_binary[6..7] == "1" {
            let max_packets = usize::from_str_radix(&raw_binary[7..18], 2).unwrap();
            parse_packets(&raw_binary[18..], max_packets)
        } else {
            let length = usize::from_str_radix(&raw_binary[7..22], 2).unwrap();
            let mut tmp = parse_packets(&raw_binary[22..22 + length], 9999);
            //todo THIS IS THE PROBLEM, THE REST OF THE BINARY EITHER GETS PUSHED OUT OR FEEDS AS GARBAGE
            tmp.push(Packet {
                raw_binary: raw_binary[21 + length..].to_string(),
                version: 0,
                packet_type: PacketType::Empty,
                sub_packets: vec![],
            });
            tmp
        };

        Self {
            raw_binary,
            version,
            packet_type,
            sub_packets,
        }
    }

    fn summary(&self) -> PacketSummary {
        match self.packet_type {
            PacketType::Empty => PacketSummary {
                values: vec![],
                length: self.raw_binary.len(),
                version: 0,
            },
            PacketType::Literal => {
                let mut value_binary = String::new();
                let mut idx = 6;
                loop {
                    let slice = &self.raw_binary[idx..idx + 5];
                    value_binary += &slice[1..];
                    idx += 5;
                    if slice.starts_with('0') {
                        break;
                    }
                }
                PacketSummary {
                    values: vec![usize::from_str_radix(&value_binary, 2).unwrap()],
                    length: idx,
                    version: self.version,
                }
            }
            PacketType::OperatorUnknown => {
                let mut summary = PacketSummary {
                    values: vec![],
                    length: if &self.raw_binary[6..7] == "1" {
                        16
                    } else {
                        20
                    },
                    version: self.version,
                };
                for p in &self.sub_packets {
                    summary = summary + p.summary()
                }
                summary.length = if summary.length % 8 == 0 {
                    summary.length
                } else {
                    (summary.length / 8 + 1) * 8
                };
                summary
            }
        }
    }

    fn apply(&self) -> usize {
        todo!()
    }
}

fn parse_packets(raw_binary: &str, max_packets: usize) -> Vec<Packet> {
    println!("Parse packets called with {raw_binary} max: {max_packets}");
    let mut raw_binary = raw_binary;
    let mut packets = vec![];
    loop {
        let new_packet = Packet::from_binary(&raw_binary);
        packets.push(new_packet);
        let last_packet_length = packets.last().unwrap().summary().length;
        raw_binary = &raw_binary[last_packet_length.max(1) - 1..];

        if max_packets <= packets.len() || raw_binary.len() < 11 {
            break;
        }
    }
    packets
}

#[cfg(test)]
mod tests {
    use super::*;
    const PATH: &str = "inputs/2021/day16.txt";
    const EXAMPLES: [&str; 4] = [
        "8A004A801A8002F478",
        "620080001611562C8802118E34",
        "C0015000016115A2E0802F182340",
        "A0016C880162017C3686B18A3D4780",
    ];
    const EXPECTED: [usize; 4] = [16, 12, 23, 31];

    #[test]
    fn example_1() {
        for (p, expected) in (0..4).map(|i| (Packet::from_hex(EXAMPLES[i]), EXPECTED[i])) {
            assert_eq!(p.summary().version, expected, "{:#?}{:#?}", p, p.summary());
        }
    }

    #[test]
    fn example_literal() {
        let p = Packet::from_hex("D2FE28");
        assert_eq!(p.raw_binary, "110100101111111000101000");
        assert_eq!(p.version, 6);
        assert_eq!(p.packet_type, PacketType::Literal);
        assert_eq!(p.sub_packets, vec![]);
        let expected_summary = PacketSummary {
            values: vec![2021],
            length: 21,
            version: 6,
        };
        assert_eq!(p.summary(), expected_summary, "{:#?}{:#?}", p, p.summary());
    }

    #[test]
    fn example_operator_sub_packet_bits() {
        let p = Packet::from_hex("38006F45291200");
        assert_eq!(p.summary().version, 9, "{:#?}{:#?}", p, p.summary());
        assert_eq!(
            p.summary().values,
            vec![10, 20],
            "{:#?}{:#?}",
            p,
            p.summary()
        );
        assert_eq!(
            p.summary().length,
            p.raw_binary.len(),
            // "{:#?}{:#?}{:#?}",
            // p,
            // p.summary(),
            // p.sub_packets.last().unwrap().summary()
        );
    }
}
