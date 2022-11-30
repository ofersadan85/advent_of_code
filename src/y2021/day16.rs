use crate::common::bin2int;

#[derive(Debug, PartialEq, Eq)]
enum LengthType {
    SubPacketBits = 0,
    SubPacketNumber = 1,
}

impl LengthType {
    fn from_bool(value: bool) -> Self {
        if value {
            Self::SubPacketNumber
        } else {
            Self::SubPacketBits
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
struct LiteralValue {
    content: usize,
    length: usize,
}
impl LiteralValue {
    fn from_binary(raw_binary: &str) -> Self {
        let mut value = String::new();
        let mut count = 6;
        loop {
            let next_chunk = raw_binary.get(count..count + 5).unwrap();
            value += next_chunk.get(1..).unwrap();
            count += 5;
            if next_chunk.starts_with('0') {
                break;
            }
        }
        let length = if count % 4 == 0 {
            count
        } else {
            (count / 4 + 1) * 4
        };
        let content = bin2int(&value).unwrap();
        LiteralValue { content, length }
    }
}

#[derive(Debug, PartialEq, Eq)]
enum OperatorType {
    Unknown,
}

#[derive(Debug, PartialEq, Eq)]
struct OperatorValue {
    length: usize,
    length_type: LengthType,
    operator_type: OperatorType,
    sub_packets: Vec<Packet>,
}

impl OperatorValue {
    fn from_binary(raw_binary: &str) -> Self {
        let length_type = LengthType::from_bool(raw_binary.get(6..7).unwrap() == "1");
        let max_sub_packets: usize = bin2int(raw_binary.get(7..18).unwrap()).unwrap(); // will only be used if LengthType::SubPacketNumber

        // todo // let max_sub_packet_bits: usize = bin2int(raw_binary.get(7..22).unwrap()).unwrap(); // will only be used if LengthType::SubPacketBits

        let (sub_index, sub_packets_raw_binary) = match length_type {
            LengthType::SubPacketBits => (22, raw_binary.get(22..).unwrap()),
            LengthType::SubPacketNumber => (18, raw_binary.get(18..).unwrap()),
        };

        // Start parsing sub-packets
        let mut sub_packets = vec![];
        let mut next_packet_index = 0;
        while next_packet_index < sub_packets_raw_binary.len() {
            let current_binary = sub_packets_raw_binary.get(next_packet_index..).unwrap();

            if current_binary.len() > 8 {
                let packet = Packet::from_binary(current_binary);
                next_packet_index += packet.len() - 1;
                sub_packets.push(packet);
            } else {
                break;
            }

            if length_type == LengthType::SubPacketNumber && sub_packets.len() == max_sub_packets {
                break;
            }
        }

        let raw_length = next_packet_index + sub_index + 5;
        let length = if raw_length % 4 == 0 {
            raw_length
        } else {
            (raw_length / 4 + 1) * 4
        };

        OperatorValue {
            length,
            length_type,
            operator_type: OperatorType::Unknown,
            sub_packets,
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
enum PacketType {
    Operator(OperatorValue),
    Literal(LiteralValue),
}

impl PacketType {
    fn from_binary(raw_binary: &str) -> Self {
        match bin2int(raw_binary.get(3..6).unwrap()).unwrap() {
            4 => Self::Literal(LiteralValue::from_binary(raw_binary)),
            _ => Self::Operator(OperatorValue::from_binary(raw_binary)),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Packet {
    version: u8,
    packet_type: PacketType,
}

impl Packet {
    fn from_hex(s: &str) -> Self {
        let mut raw_binary = String::new();
        let hex_vec: Vec<char> = s.chars().collect();
        for i in (0..hex_vec.len()).step_by(2) {
            let small_hex = s.get(i..i + 2).unwrap();
            let chunk = u8::from_str_radix(small_hex, 16).unwrap();
            let chunk = format!("{chunk:0>8b}");
            raw_binary += &chunk;
        }
        Self::from_binary(&raw_binary)
    }

    fn from_binary(raw_binary: &str) -> Self {
        Packet {
            version: bin2int(raw_binary.get(0..3).unwrap()).unwrap(),
            packet_type: PacketType::from_binary(raw_binary),
        }
    }

    fn len(&self) -> usize {
        match &self.packet_type {
            PacketType::Literal(value) => value.length,
            PacketType::Operator(value) => value.length,
        }
    }
}

fn nested_version_sum(packet: &Packet, sum: &mut usize) {
    *sum += packet.version as usize;
    if let PacketType::Operator(value) = &packet.packet_type {
        for p in &value.sub_packets {
            nested_version_sum(p, sum);
        }
    }
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
    fn example_1_nested_version_sum() {
        for (packet, expected) in (0..4).map(|i| (Packet::from_hex(EXAMPLES[i]), EXPECTED[i])) {
            let mut sum = 0;
            nested_version_sum(&packet, &mut sum);
            assert_eq!(sum, expected);
        }
    }

    #[test]
    fn task_1_nested_version_sum() {
        let data = std::fs::read_to_string(PATH).unwrap();
        let p = Packet::from_hex(&data);
        let mut sum = 0;
        nested_version_sum(&p, &mut sum);
        assert_ne!(sum, 790);
    }

    #[test]
    fn example_literal() {
        let p = Packet::from_hex("D2FE28");
        let expected = Packet {
            version: 6,
            packet_type: PacketType::Literal(LiteralValue {
                content: 2021,
                length: 24,
            }),
        };
        assert_eq!(p, expected);
    }

    #[test]
    fn example_operator_sub_packet_bits() {
        let p = Packet::from_hex("38006F45291200");
        let expected = Packet {
            version: 1,
            packet_type: PacketType::Operator(OperatorValue {
                length: 56,
                length_type: LengthType::SubPacketBits,
                operator_type: OperatorType::Unknown,
                sub_packets: vec![
                    Packet {
                        version: 6,
                        packet_type: PacketType::Literal(LiteralValue {
                            content: 10,
                            length: 12,
                        }),
                    },
                    Packet {
                        version: 2,
                        packet_type: PacketType::Literal(LiteralValue {
                            content: 20,
                            length: 16,
                        }),
                    },
                ],
            }),
        };
        assert_eq!(p, expected);
    }

    #[test]
    fn example_operator_sub_packet_num() {
        let p = Packet::from_hex("EE00D40C823060");
        let expected = Packet {
            version: 7,

            packet_type: PacketType::Operator(OperatorValue {
                length: 56,
                length_type: LengthType::SubPacketNumber,
                operator_type: OperatorType::Unknown,
                sub_packets: vec![
                    Packet {
                        version: 2,
                        packet_type: PacketType::Literal(LiteralValue {
                            content: 1,
                            length: 12,
                        }),
                    },
                    Packet {
                        version: 4,
                        packet_type: PacketType::Literal(LiteralValue {
                            content: 2,
                            length: 12,
                        }),
                    },
                    Packet {
                        version: 1,
                        packet_type: PacketType::Literal(LiteralValue {
                            content: 3,
                            length: 12,
                        }),
                    },
                ],
            }),
        };
        assert_eq!(p, expected);
    }
}
