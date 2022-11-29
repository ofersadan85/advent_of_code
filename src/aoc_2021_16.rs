use crate::common::*;

#[derive(Debug, PartialEq, Eq)]
enum LengthType {
    SubPacketBits = 0,
    SubPacketNumber = 1,
    Literal,
}

impl LengthType {
    fn from_bool(value: bool) -> Self {
        match value {
            false => Self::SubPacketBits,
            true => Self::SubPacketNumber,
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
struct LiteralValue {
    content: usize,
    length: usize,
}
impl LiteralValue {
    fn from_binary(raw_binary: &str) -> Option<Self> {
        let mut value = "".to_string();
        let mut count = 6;
        loop {
            let next_chunk = raw_binary.get(count..count + 5)?;
            value += next_chunk.get(1..)?;
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
        let content = bin2int(&value)?;
        Some(LiteralValue { content, length })
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
}

impl OperatorValue {
    fn from_binary(raw_binary: &str) -> Option<Self> {
        let length_type = LengthType::from_bool(raw_binary.get(6..7)? == "1");
        let length_indexes = match length_type {
            LengthType::SubPacketBits => 7..22,
            LengthType::SubPacketNumber => 7..18,
            _ => return None,
        };
        let length = bin2int(raw_binary.get(length_indexes)?)?;
        Some(OperatorValue {
            length,
            length_type,
            operator_type: OperatorType::Unknown,
        })
    }

    fn sub_packets(&self) -> Option<Vec<Self>> {
        todo!()
    }
}

#[derive(Debug, PartialEq, Eq)]
enum PacketType {
    Operator(OperatorValue),
    Literal(LiteralValue),
}

impl PacketType {
    fn from_binary(raw_binary: &str) -> Option<Self> {
        match bin2int(raw_binary.get(3..6)?)? {
            4 => Some(Self::Literal(LiteralValue::from_binary(raw_binary)?)),
            _ => Some(Self::Operator(OperatorValue::from_binary(raw_binary)?)),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Packet {
    version: u8,
    packet_type: PacketType,
    raw_binary: String,
}

impl Packet {
    fn from_hex(s: &str) -> Option<Self> {
        let mut raw_binary = "".to_string();
        let hex_vec: Vec<char> = s.chars().collect();
        for i in (0..hex_vec.len()).step_by(2) {
            let small_hex = s.get(i..i + 2)?;
            let chunk = u8::from_str_radix(small_hex, 16).ok()?;
            let chunk = format!("{chunk:0>8b}");
            raw_binary += &chunk;
        }
        Self::from_binary(&raw_binary)
    }

    fn from_binary(raw_binary: &str) -> Option<Self> {
        Some(Packet {
            version: bin2int(raw_binary.get(0..3)?)?,
            packet_type: PacketType::from_binary(raw_binary)?,
            raw_binary: raw_binary.to_string(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    const PATH: &str = "inputs/aoc_2021_16.txt";
    // const DATA: &str = &std::fs::read_to_string(PATH).unwrap();

    #[test]
    fn packet_literal() {
        let p = Packet::from_hex("D2FE28").unwrap();
        let expected = Packet {
            raw_binary: "110100101111111000101000".to_string(),
            version: 6,
            packet_type: PacketType::Literal(LiteralValue {
                content: 2021,
                length: 24,
            }),
        };
        assert_eq!(p, expected);
    }

    #[test]
    fn packet_operator_sub_packet_bits() {
        let p = Packet::from_hex("38006F45291200").unwrap();
        let expected = Packet {
            raw_binary: "00111000000000000110111101000101001010010001001000000000".to_string(),
            version: 1,
            packet_type: PacketType::Operator(OperatorValue {
                length: 27,
                length_type: LengthType::SubPacketBits,
                operator_type: OperatorType::Unknown,
            }),
        };
        assert_eq!(p, expected);

        // let sub_packets = p.sub_packets().unwrap();
        // assert_eq!(sub_packets.len(), 2);
        // assert_eq!(sub_packets[0].literal_content().unwrap().0, 10);
        // assert_eq!(sub_packets[1].literal_content().unwrap().0, 20);
    }

    #[test]
    fn packet_operator_sub_packet_num() {
        let p = Packet::from_hex("EE00D40C823060").unwrap();
        let expected = Packet {
            raw_binary: "11101110000000001101010000001100100000100011000001100000".to_string(),
            version: 7,

            packet_type: PacketType::Operator(OperatorValue {
                length: 3,
                length_type: LengthType::SubPacketNumber,
                operator_type: OperatorType::Unknown,
            }),
        };
        assert_eq!(p, expected);

        // let sub_packets = p.sub_packets().unwrap();
        // assert_eq!(sub_packets.len(), 3);
        // assert_eq!(sub_packets[0].literal_content().unwrap().0, 1);
        // assert_eq!(sub_packets[1].literal_content().unwrap().0, 2);
        // assert_eq!(sub_packets[2].literal_content().unwrap().0, 3);
    }
}
