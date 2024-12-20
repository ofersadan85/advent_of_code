use advent_of_code_common::iterators::BitsIterator;
use advent_of_code_macros::aoc_tests;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum OpType {
    Literal(usize),
    Sum,
    Product,
    Min,
    Max,
    Greater,
    Less,
    Equal,
}

impl From<OpType> for u8 {
    fn from(op_type: OpType) -> Self {
        match op_type {
            OpType::Sum => 0,
            OpType::Product => 1,
            OpType::Min => 2,
            OpType::Max => 3,
            OpType::Literal(_) => 4,
            OpType::Greater => 5,
            OpType::Less => 6,
            OpType::Equal => 7,
        }
    }
}

impl TryFrom<usize> for OpType {
    type Error = &'static str;

    fn try_from(value: usize) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Sum),
            1 => Ok(Self::Product),
            2 => Ok(Self::Min),
            3 => Ok(Self::Max),
            4 => Ok(Self::Literal(0)),
            5 => Ok(Self::Greater),
            6 => Ok(Self::Less),
            7 => Ok(Self::Equal),
            _ => Err("Invalid packet type"),
        }
    }
}

impl OpType {
    fn id(&self) -> u8 {
        u8::from(*self)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum LengthType {
    Bits(u16),
    Operands(u16),
    Unknown,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Packet {
    version: u8,
    op_type: OpType,
    length: LengthType,
    operands: Vec<Packet>,
}

impl Packet {
    fn value(&self) -> usize {
        use std::cmp::Ordering;
        let iter_values = || self.operands.iter().map(Self::value);
        let cmp_operands = || {
            let a = self.operands.first().expect("No first operand").value();
            let b = self.operands.get(1).expect("No second operand").value();
            a.cmp(&b)
        };
        match &self.op_type {
            OpType::Sum => iter_values().sum(),
            OpType::Product => iter_values().product(),
            OpType::Min => iter_values().min().expect("No min"),
            OpType::Max => iter_values().max().expect("No max"),
            OpType::Literal(v) => *v,
            OpType::Greater => usize::from(cmp_operands() == Ordering::Greater),
            OpType::Less => usize::from(cmp_operands() == Ordering::Less),
            OpType::Equal => usize::from(cmp_operands() == Ordering::Equal),
        }
    }

    fn version_sum(&self) -> usize {
        self.version as usize + self.operands.iter().map(Self::version_sum).sum::<usize>()
    }

    fn from_bits_literal(bits: &mut BitsIterator) -> Result<(usize, usize), &'static str> {
        let mut has_more = true;
        let mut value = 0;
        let mut consumed = 0;
        while has_more {
            has_more = bits.consume(1).ok_or("Expecting more bit")? == 1;
            let new_value = bits.consume(4).ok_or("Expecting value")?;
            value = (value << 4) | new_value;
            consumed += 5;
        }
        Ok((value, consumed))
    }

    fn from_bits(bits: &mut BitsIterator) -> Result<(Self, usize), &'static str> {
        let mut consumed = 0;
        let version = bits
            .consume(3)
            .and_then(|v| u8::try_from(v).ok())
            .ok_or("Invalid version")?;
        consumed += 3;
        let type_bits = bits.consume(3);
        consumed += 3;
        let (op_type, length, operands) = match type_bits {
            None => return Err("Missing packet type"),
            Some(4) => {
                let (value, consumed_literal) = Self::from_bits_literal(bits)?;
                consumed += consumed_literal;
                (OpType::Literal(value), LengthType::Unknown, vec![])
            }
            Some(op_type_id) => {
                let op_type = OpType::try_from(op_type_id)?;
                let length_bit = bits.consume(1).ok_or("Missing length type")?;
                consumed += 1;
                let length = match length_bit {
                    0 => {
                        let length = LengthType::Bits(
                            bits.consume(15)
                                .and_then(|v| u16::try_from(v).ok())
                                .ok_or("Invalid length")?,
                        );
                        consumed += 15;
                        length
                    }
                    1 => {
                        let length = LengthType::Operands(
                            bits.consume(11)
                                .and_then(|v| u16::try_from(v).ok())
                                .ok_or("Invalid length")?,
                        );
                        consumed += 11;
                        length
                    }
                    _ => return Err("Invalid length type"),
                };
                let mut operands = vec![];
                let mut consumed_for_operands = 0;
                loop {
                    match length {
                        LengthType::Unknown => {}
                        LengthType::Bits(count) => {
                            let (operand, consumed_operand) = Self::from_bits(bits)?;
                            consumed_for_operands += consumed_operand;
                            operands.push(operand);
                            if consumed_for_operands > usize::from(count) {
                                return Err("operands exceed length");
                            }
                            if consumed_for_operands == usize::from(count) {
                                break;
                            }
                        }
                        LengthType::Operands(count) => {
                            if operands.len() < usize::from(count) {
                                let (operand, consumed_operand) = Self::from_bits(bits)?;
                                consumed_for_operands += consumed_operand;
                                operands.push(operand);
                            } else {
                                break;
                            }
                        }
                    }
                }
                consumed += consumed_for_operands;
                (op_type, length, operands)
            }
        };
        Ok((
            Self {
                version,
                op_type,
                length,
                operands,
            },
            consumed,
        ))
    }
}

impl std::str::FromStr for Packet {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut bits: BitsIterator = s.parse()?;
        let (packet, _consumed) = Self::from_bits(&mut bits)?;
        if bits.remaining() > 0 {
            let padding = bits.consume_until_end();
            debug_assert_eq!(padding, 0, "Padding bits");
        }
        Ok(packet)
    }
}

#[aoc_tests]
mod tests {
    #[test]
    fn literal() {
        let packet: Packet = "D2FE28".parse().unwrap();
        assert_eq!(packet.version, 6);
        assert_eq!(packet.op_type.id(), 4);
        assert_eq!(packet.value(), 2021);
    }

    #[test]
    fn op_bit_length() {
        let packet: Packet = "38006F45291200".parse().unwrap();
        assert_eq!(packet.version, 1);
        assert_eq!(packet.op_type.id(), 6);
        assert_eq!(packet.operands.len(), 2);
        assert_eq!(packet.operands[0].value(), 10);
        assert_eq!(packet.operands[1].value(), 20);
    }

    #[test]
    fn op_operands_length() {
        let packet: Packet = "EE00D40C823060".parse().unwrap();
        assert_eq!(packet.version, 7);
        assert_eq!(packet.op_type.id(), 3);
        assert_eq!(packet.operands.len(), 3);
        assert_eq!(packet.operands[0].value(), 1);
        assert_eq!(packet.operands[1].value(), 2);
        assert_eq!(packet.operands[2].value(), 3);
    }

    #[test]
    fn version_sum() {
        let packet: Packet = "8A004A801A8002F478".parse().unwrap();
        assert_eq!(packet.version_sum(), 16);
        let packet: Packet = "620080001611562C8802118E34".parse().unwrap();
        assert_eq!(packet.version_sum(), 12);
        let packet: Packet = "C0015000016115A2E0802F182340".parse().unwrap();
        assert_eq!(packet.version_sum(), 23);
        let packet: Packet = "A0016C880162017C3686B18A3D4780".parse().unwrap();
        assert_eq!(packet.version_sum(), 31);
    }

    #[test]
    fn part_1() {
        let packet: Packet = read_input().parse().unwrap();
        assert_eq!(packet.version_sum(), 965);
    }

    #[test]
    fn values_ops() {
        let packet: Packet = "C200B40A82".parse().unwrap();
        assert_eq!(packet.value(), 3, "{:?}", packet.op_type);
        let packet: Packet = "04005AC33890".parse().unwrap();
        assert_eq!(packet.value(), 54, "{:?}", packet.op_type);
        let packet: Packet = "880086C3E88112".parse().unwrap();
        assert_eq!(packet.value(), 7, "{:?}", packet.op_type);
        let packet: Packet = "CE00C43D881120".parse().unwrap();
        assert_eq!(packet.value(), 9, "{:?}", packet.op_type);
        let packet: Packet = "D8005AC2A8F0".parse().unwrap();
        assert_eq!(packet.value(), 1, "{:?}", packet.op_type);
        let packet: Packet = "F600BC2D8F".parse().unwrap();
        assert_eq!(packet.value(), 0, "{:?}", packet.op_type);
        let packet: Packet = "9C005AC2F8F0".parse().unwrap();
        assert_eq!(packet.value(), 0, "{:?}", packet.op_type);
        let packet: Packet = "9C0141080250320F1802104A08".parse().unwrap();
        assert_eq!(packet.value(), 1, "{:?}", packet.op_type);
    }

    #[test]
    fn part_2() {
        let packet: Packet = read_input().parse().unwrap();
        assert_eq!(packet.value(), 116672213160);
    }
}
