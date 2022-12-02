use bitstream_io::{BigEndian, BitRead, BitReader};
use std::io::Cursor;

use crate::traits::*;

pub struct S;

struct Reader {
    inner: BitReader<Cursor<Vec<u8>>, BigEndian>,
    bits_read: usize,
}

impl Reader {
    fn read_bit(&mut self) -> std::io::Result<bool> {
        self.bits_read += 1;
        self.inner.read_bit()
    }

    fn read<U>(&mut self, bits: u32) -> std::io::Result<U>
    where
        U: bitstream_io::Numeric,
    {
        self.bits_read += bits as usize;
        self.inner.read(bits)
    }

    fn align_4_bit(&mut self) -> std::io::Result<()> {
        let one_bigger = ((self.bits_read - 1) / 4 + 1) * 4;
        let to_read = one_bigger - self.bits_read;
        if to_read != 0 {
            println!("     aligning: {} to {}", to_read, one_bigger);
            let _: u8 = self.inner.read(to_read as u32)?;
        }
        Ok(())
    }
}

const INDENT: &str = "                                                                                                                                                                                                                                                        ";
fn indent(i: usize) -> &'static str {
    &INDENT[0..i * 2]
}

fn sum_packet_versions(reader: &mut Reader, i: usize) -> usize {
    let mut total_version = 0;
    let version: u8 = match reader.read(3) {
        Ok(v) => v,
        Err(_) => return total_version,
    };

    let id: u8 = match reader.read(3) {
        Ok(i) => i,
        Err(_) => return total_version,
    };
    println!("{}version: {}", indent(i), version);
    println!("{}id: {}", indent(i), id);
    total_version += version as usize;
    if id == 4 {
        println!("{}lit", indent(i));
        loop {
            let more = reader.read_bit().unwrap();
            let data: u8 = reader.read(4).unwrap();
            println!("{}part {}", indent(i), data);
            if !more {
                break;
            }
        }
    } else {
        let let_type = reader.read_bit().unwrap();
        let subpacket_count: u16 = if let_type {
            reader.read(11).unwrap()
        } else {
            reader.read(15).unwrap()
        };
        println!("{}op packet {} subpackets", indent(i), subpacket_count);
        for ii in 0..subpacket_count {
            println!("{} parsing {}", indent(i), ii);
            total_version += sum_packet_versions(reader, i + 1);
        }
    }

    total_version
}

fn process_packets(reader: &mut Reader, i: usize) -> std::io::Result<usize> {
    let version: u8 = reader.read(3)?;

    let id: u8 = reader.read(3)?;

    println!("{}version: {}", indent(i), version);
    println!("{}id: {}", indent(i), id);
    if id == 4 {
        println!("{}lit", indent(i));
        let mut lit_val: usize = 0;
        loop {
            let more = reader.read_bit()?;
            let data: u8 = reader.read(4)?;
            lit_val = lit_val << 4 | data as usize;
            println!("{}part {} tot: {}", indent(i), data, lit_val);
            if !more {
                println!("{} returning {} - {}", indent(i), lit_val, data);
                break Ok(lit_val);
            }
        }
    } else {
        let let_type = reader.read_bit()?;
        let mut bits_left = None;
        let mut packets_left = None;
        if let_type {
            packets_left = Some(reader.read::<u16>(11)?);
        } else {
            bits_left = Some(reader.read::<u16>(15)?);
        };
        println!("{}op packet ", indent(i));

        let mut children = Vec::new();
        loop {
            println!("{} bits left {:?} packets left {:?}", indent(i), bits_left, packets_left);
            let before_bits = reader.bits_read;
            let child = process_packets(reader, i + 1).unwrap();
            let bits_read = reader.bits_read - before_bits;
            children.push(child);

            if let Some(bits_left) = &mut bits_left {
                *bits_left -= bits_read as u16;
                if *bits_left == 0 {
                    break;
                }
            }
            if let Some(packets_left) = &mut packets_left {
                *packets_left -= 1;
                if *packets_left == 0 {
                    break;
                }
            }
        }
        println!("{}got children {:?}", indent(i), &children);

        Ok(match id {
            0 => children.iter().sum(),
            1 => children.iter().product(),
            2 => *children.iter().min().unwrap(),
            3 => *children.iter().max().unwrap(),
            5 => if children[0] > children[1] { 1 } else { 0 },
            6 => if children[0] < children[1] { 1 } else { 0 },
            7 => if children[0] == children[1] { 1 } else { 0 },
            _ => unreachable!(),
        })
    }
}

impl AocDay for S {
    fn part1(&self, input: crate::traits::Input) -> Output {
        let bytes = data_encoding::HEXUPPER_PERMISSIVE
            .decode(input.as_str().trim().as_bytes())
            .unwrap();

        let cursor = Cursor::new(bytes);
        let mut reader = Reader {
            inner: BitReader::endian(cursor, BigEndian),
            bits_read: 0,
        };

        let root = sum_packet_versions(&mut reader, 0);
        root.into()
    }

    fn part2(&self, input: crate::traits::Input) -> Output {
        let bytes = data_encoding::HEXUPPER_PERMISSIVE
            .decode(input.as_str().trim().as_bytes())
            .unwrap();

        let cursor = Cursor::new(bytes);
        let mut reader = Reader {
            inner: BitReader::endian(cursor, BigEndian),
            bits_read: 0,
        };

        process_packets(&mut reader, 0).unwrap().into()
    }
}
