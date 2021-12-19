#[derive(Debug)]
struct Packet {
    version: u8,
    type_id: u8,
    literal_value: Option<u64>,
    length_type_id: Option<u8>,
    sub_packets: Option<Vec<Packet>>,
    length_in_bits: usize,
}

fn parse_packet(bits: &[u8]) -> Packet {
    let version: u8 = bits_to_u8(&bits[0..=2]);
    let type_id: u8 = bits_to_u8(&bits[3..=5]);
    let mut length_in_bits = 0;
    let mut literal_value = None;
    if type_id == 4 {
        let literal_value_result = parse_literal_value(&bits[6..]);
        literal_value = Some(literal_value_result.0);
        length_in_bits = literal_value_result.1 + 3 + 3; // version and type ID are each 3 bits
    }
    let length_type_id = match type_id {
        4 => None,
        _ => Some(bits[6]),
    };
    let sub_packets = match (type_id, length_type_id) {
        (4, _) => None,
        (_, Some(0)) => {
            let subpackets_length_in_bits = bits_to_usize(&bits[7..22]);
            let mut parsed_packets: Vec<Packet> = Vec::new();
            let mut running_length_total: usize = 0;
            while running_length_total < subpackets_length_in_bits {
                let new_packet = parse_packet(&bits[(22 + running_length_total)..]);
                running_length_total += new_packet.length_in_bits;
                parsed_packets.push(new_packet);
            }
            Some(parsed_packets)
        }
        (_, Some(1)) => {
            let number_of_subpackets: usize = bits_to_usize(&bits[7..18]);
            let mut parsed_packets: Vec<Packet> = Vec::new();
            let mut running_length_total: usize = 0;
            while parsed_packets.len() < number_of_subpackets {
                let new_packet = parse_packet(&bits[(18 + running_length_total)..]);
                running_length_total += new_packet.length_in_bits;
                parsed_packets.push(new_packet);
            }
            Some(parsed_packets)
        }
        (_, _) => {
            panic!("Invalid combination of packet type ID and length type ID");
        }
    };
    if type_id != 4 {
        length_in_bits = sub_packets
            .as_ref()
            .unwrap()
            .iter()
            .map(|p| p.length_in_bits)
            .sum::<usize>()
            + match length_type_id {
                Some(0) => 3 + 3 + 1 + 15,
                Some(1) => 3 + 3 + 1 + 11,
                _ => panic!("Invalid length type ID"),
            };
    }
    Packet {
        version,
        type_id,
        literal_value,
        length_type_id,
        sub_packets,
        length_in_bits,
    }
}

/// Parses a literal value as found in packet type ID 4.
///
/// Returns (literal_value, length_in_bits) where literal_value is the
/// value encoded by the bit string, and length_in_bits is the total
/// number of bits used to represent the literal value including the
/// "header" bits in every fifth position.
fn parse_literal_value(bits: &[u8]) -> (u64, usize) {
    let mut literal_value: u64 = 0;
    let mut length_in_bits: usize = 0;
    for group in 0..=bits.len() / 5 {
        let group_start = group * 5;
        literal_value <<= 4;
        literal_value += bits_to_u64(&bits[group_start + 1..=group_start + 4]);
        length_in_bits += 5;
        if bits[group_start] == 0 {
            break;
        }
    }
    (literal_value, length_in_bits)
}

pub(crate) fn solve_part_1(puzzle_string: String) -> String {
    let bits = puzzle_string
        .trim()
        .chars()
        .map(|c| {
            let nybble = c.to_digit(16).unwrap() as u8;
            [
                (nybble & 8) >> 3,
                (nybble & 4) >> 2,
                (nybble & 2) >> 1,
                nybble & 1,
            ]
        })
        .flatten()
        .collect::<Vec<u8>>();
    // println!("{:?}", bits);
    let packet = parse_packet(&bits);
    // println!("Packet: {:?}", packet);
    let mut packets: Vec<Packet> = Vec::new();
    packets.push(packet);
    sum_packet_versions(&packets).to_string()
}

fn sum_packet_versions(packets: &Vec<Packet>) -> u64 {
    packets
        .iter()
        .map(|packet| match &packet.sub_packets {
            None => packet.version as u64,
            Some(sub_packets) => packet.version as u64 + sum_packet_versions(sub_packets),
        })
        .sum()
}

fn bits_to_u8(bits: &[u8]) -> u8 {
    if bits.len() > 8 {
        panic!("Data would be lost converting this value to u8");
    }
    let mut result: u8 = 0;
    for bit in bits.iter() {
        result <<= 1;
        result += *bit;
    }
    result
}

fn bits_to_u64(bits: &[u8]) -> u64 {
    if bits.len() > 64 {
        panic!("Data would be lost converting this value to u64");
    }
    let mut result: u64 = 0;
    for bit in bits.iter() {
        result <<= 1;
        result += *bit as u64;
    }
    result
}

fn bits_to_usize(bits: &[u8]) -> usize {
    let mut result: usize = 0;
    for bit in bits.iter() {
        result <<= 1;
        result += *bit as usize;
    }
    result
}

#[cfg(test)]
use super::test_helpers;

#[test]
fn test_part_1() {
    assert_eq!(solve_part_1(test_helpers::load_puzzle_input(16)), "860");
}
