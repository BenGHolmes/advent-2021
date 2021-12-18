use std::collections::VecDeque;

const INPUT: &'static str = include_str!("../inputs/day16.txt");

pub(crate) fn run() {
    println!("day 15, output 1: {}", parse1(INPUT));
    println!("day 15, output 2: {}", parse2(INPUT));
}

fn parse1(input: &str) -> usize {
    let mut buffer = BitBuffer::from_hex_str(input);
    let packet = Packet::from_buffer(&mut buffer);

    let mut queue = VecDeque::from(vec![packet]);
    let mut sum = 0;

    while !queue.is_empty() {
        match queue.pop_front().unwrap() {
            Packet::Literal { version, value: _ } => sum += version as usize,
            Packet::Operator {
                version,
                type_id: _,
                sub_packets,
            } => {
                sum += version as usize;
                queue.extend(sub_packets);
            }
        }
    }

    sum
}

fn parse2(input: &str) -> usize {
    let mut buffer = BitBuffer::from_hex_str(input);
    let packet = Packet::from_buffer(&mut buffer);

    packet.value()
}

#[derive(Debug, Eq, PartialEq)]
enum Packet {
    Literal {
        version: u8,
        value: usize,
    },
    Operator {
        version: u8,
        type_id: u8,
        sub_packets: Vec<Packet>,
    },
}

impl Packet {
    fn from_buffer(buffer: &mut BitBuffer) -> Self {
        let version = buffer.read_bits(3) as u8;
        let type_id = buffer.read_bits(3) as u8;
        let packet = match type_id {
            4 => Packet::Literal {
                version,
                value: Packet::parse_literal(buffer),
            },
            _ => Packet::Operator {
                version,
                type_id,
                sub_packets: Packet::parse_subpackets(buffer),
            },
        };

        packet
    }

    fn parse_literal(buffer: &mut BitBuffer) -> usize {
        let mut value = 0;
        loop {
            let end = buffer.read_bits(1) == 0;
            value <<= 4;
            value |= buffer.read_bits(4);
            if end {
                break;
            }
        }
        value
    }

    fn parse_subpackets(buffer: &mut BitBuffer) -> Vec<Packet> {
        let (mut n_bits, n_packets) = match buffer.read_bits(1) {
            0 => (buffer.read_bits(15), 0),
            1 => (0, buffer.read_bits(11)),
            _ => unreachable!(),
        };

        let mut sub_packets = Vec::new();

        while n_bits > 0 {
            let initial_pos = 4 * buffer.index + buffer.bit;
            sub_packets.push(Packet::from_buffer(buffer));
            let final_pos = 4 * buffer.index + buffer.bit;
            n_bits -= final_pos - initial_pos;
        }

        for _ in 0..n_packets {
            sub_packets.push(Packet::from_buffer(buffer));
        }

        sub_packets
    }

    fn value(&self) -> usize {
        match self {
            Packet::Literal { version: _, value } => *value,
            Packet::Operator {
                version: _,
                type_id: 0,
                sub_packets,
            } => sub_packets.iter().map(|op| op.value()).sum(),
            Packet::Operator {
                version: _,
                type_id: 1,
                sub_packets,
            } => sub_packets.iter().map(|op| op.value()).product(),
            Packet::Operator {
                version: _,
                type_id: 2,
                sub_packets,
            } => sub_packets.iter().map(|op| op.value()).min().unwrap(),
            Packet::Operator {
                version: _,
                type_id: 3,
                sub_packets,
            } => sub_packets.iter().map(|op| op.value()).max().unwrap(),
            Packet::Operator {
                version: _,
                type_id: 5,
                sub_packets,
            } => (sub_packets[0].value() > sub_packets[1].value()) as usize,
            Packet::Operator {
                version: _,
                type_id: 6,
                sub_packets,
            } => (sub_packets[0].value() < sub_packets[1].value()) as usize,
            Packet::Operator {
                version: _,
                type_id: 7,
                sub_packets,
            } => (sub_packets[0].value() == sub_packets[1].value()) as usize,
            _ => unreachable!(),
        }
    }
}

struct BitBuffer {
    bytes: Vec<u8>,
    index: usize,
    bit: usize,
}

impl BitBuffer {
    fn from_hex_str(hex_str: &str) -> Self {
        BitBuffer::from_bytes(
            hex_str
                .bytes()
                .map(|b| if b < b'A' { b - b'0' } else { b - b'A' + 10 })
                .collect(),
        )
    }

    fn from_bytes(bytes: Vec<u8>) -> Self {
        Self {
            bytes,
            index: 0,
            bit: 0,
        }
    }

    fn read_bits(&mut self, n_bits: usize) -> usize {
        let mut out: usize = 0;
        for _ in 0..n_bits {
            let this_byte = self.bytes[self.index];
            let this_bit = (this_byte >> (3 - self.bit)) & 1;
            out = (out << 1) | this_bit as usize;

            self.bit += 1;
            if self.bit == 4 {
                self.bit = 0;
                self.index += 1;
            }
        }

        out
    }

    fn peek_bit(&self) -> bool {
        let this_byte = self.bytes[self.index];
        let this_bit = (this_byte >> (3 - self.bit)) & 1;
        this_bit == 1
    }

    fn is_empty(&self) -> bool {
        self.index == self.bytes.len()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUTS: &'static str = "1163751742
1381373672
2136511328
3694931569
7463417111
1319128137
1359912421
3125421639
1293138521
2311944581";
    #[test]
    fn parse_literal() {
        let in_str = "D2FE28";
        let mut buffer = BitBuffer::from_hex_str(in_str);

        let packet = Packet::from_buffer(&mut buffer);
        assert_eq!(
            packet,
            Packet::Literal {
                value: 2021,
                version: 6
            }
        )
    }

    #[test]
    fn parse_operator() {
        let in_str = "EE00D40C823060";
        let mut buffer = BitBuffer::from_hex_str(in_str);

        let packet = Packet::from_buffer(&mut buffer);
        let expected = Packet::Operator {
            version: 7,
            type_id: 3,
            sub_packets: vec![
                Packet::Literal {
                    version: 2,
                    value: 1,
                },
                Packet::Literal {
                    version: 4,
                    value: 2,
                },
                Packet::Literal {
                    version: 1,
                    value: 3,
                },
            ],
        };

        assert_eq!(packet, expected);
    }

    #[test]
    fn first() {
        for (input, expected) in [
            ("8A004A801A8002F478", 16),
            ("620080001611562C8802118E34", 12),
            ("C0015000016115A2E0802F182340", 23),
            ("A0016C880162017C3686B18A3D4780", 31),
        ] {
            assert_eq!(parse1(input), expected);
        }
    }

    #[test]
    fn second() {
        for (input, expected) in [
            ("C200B40A82", 3),
            ("04005AC33890", 54),
            ("880086C3E88112", 7),
            ("CE00C43D881120", 9),
            ("D8005AC2A8F0", 1),
            ("F600BC2D8F", 0),
            ("9C005AC2F8F0", 0),
            ("9C0141080250320F1802104A08", 1),
        ] {
            assert_eq!(parse2(input), expected);
        }
    }
}
