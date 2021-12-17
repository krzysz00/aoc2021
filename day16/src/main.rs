#[derive(Clone, Debug)]
struct Bitstream {
    bits: Vec<u8>,
    idx: usize,
    consumed: usize,
    valid_bits: u8,
}

impl Bitstream {
    pub fn new(bits: Vec<u8>) -> Self {
        Self { bits, idx: 0, consumed: 0, valid_bits: 8 }
    }

    pub fn get(&mut self, n: u8) -> u8 {
        use std::cmp::Ordering;
        assert!(n <= 8);
        match n.cmp(&self.valid_bits) {
            Ordering::Less => {
                let new_valid = self.valid_bits - n;
                let ret = self.bits.get(self.idx).copied().unwrap_or(0) >> new_valid;
                self.bits.get_mut(self.idx).map(|b| *b &= (1 << new_valid) - 1);
                self.valid_bits = new_valid;
                self.consumed += n as usize;
                ret
            },
            Ordering::Equal => {
                self.valid_bits = 8;
                self.consumed += n as usize;
                let ret = self.bits.get(self.idx).copied().unwrap_or(0);
                self.bits.get_mut(self.idx).map(|v| *v = 0);
                self.idx += 1;
                ret
            },
            Ordering::Greater => {
                let in_this_bit = self.valid_bits;
                let remaining = n - in_this_bit;
                let this_bit = self.bits.get(self.idx).copied()
                    .unwrap_or(0) << remaining;
                self.consumed += in_this_bit as usize;
                self.valid_bits = 8;
                self.bits.get_mut(self.idx).map(|v| *v = 0);
                self.idx += 1;
                let sequel = self.get(remaining);
                this_bit | sequel
            }
        }
    }

    pub fn consumed(&self) -> usize { self.consumed }
}

fn to_bitstream(string: &str) -> Bitstream {
    let char_vec: Vec<char> = string.trim().chars().collect();
    let bytes = char_vec.chunks(2).map(|c| {
        let high = c[0].to_digit(16).unwrap() as u8;
        let low = c.get(1).unwrap_or(&'0').to_digit(16).unwrap() as u8;
        low | (high << 4)
    }).collect();
    Bitstream::new(bytes)
}

#[derive(Clone, Debug)]
enum PacketType {
    Literal(u64),
    Operator { typ: u8, ops: Vec<Packet> },
}

#[derive(Clone, Debug)]
struct Packet {
    pub version: u8,
    pub body: PacketType,
}

fn parse_packet(bits: &mut Bitstream) -> Packet {
    let version = bits.get(3);
    let typ = bits.get(3);
    if typ == 4 {
        let mut ret: u64 = 0;
        loop {
            let next = bits.get(5);
            let cont = next >> 4;
            let value = (next & 0xf) as u64;
            ret = (ret << 4) | value;
            if cont == 0 { break; }
        }
        Packet { version, body: PacketType::Literal(ret) }
    } else {
        let length_type = bits.get(1);
        match length_type {
            0 => {
                let to_consume_hi = bits.get(7) as usize;
                let to_consume_lo = bits.get(8) as usize;
                let to_consume = (to_consume_hi << 8) | to_consume_lo;
                let consume_start = bits.consumed();
                let mut ops = Vec::new();
                while bits.consumed() - consume_start < to_consume {
                    ops.push(parse_packet(bits));
                }
                Packet { version, body: PacketType::Operator { typ, ops }}
            },
            1 => {
                let n_packets_hi = bits.get(3) as usize;
                let n_packets_lo = bits.get(8) as usize;
                let n_packets = (n_packets_hi << 8) | n_packets_lo;
                let ops = (0..n_packets).map(|_| parse_packet(bits)).collect();
                Packet { version, body: PacketType::Operator { typ, ops }}
            },
            _ => unreachable!(),
        }
    }
}

fn versions_sum(packet: &Packet) -> u64 {
    let mut ret = packet.version as u64;
    match &packet.body {
        PacketType::Literal(_) => (),
        PacketType::Operator {typ: _typ, ops} => {
            for packet in ops.iter() {
                ret += versions_sum(packet);
            }
        }
    }
    ret
}

fn execute(packet: &Packet) -> u64 {
    match &packet.body {
        PacketType::Literal(v) => *v,
        PacketType::Operator { typ, ops } => {
            match *typ {
                0 => { ops.iter().map(execute).sum() },
                1 => { ops.iter().map(execute).product() },
                2 => { ops.iter().map(execute).min().unwrap_or(0) },
                3 => { ops.iter().map(execute).max().unwrap_or(0) },
                4 => panic!("Literal here"),
                5 => {
                    let v0 = execute(&ops[0]);
                    let v1 = execute(&ops[1]);
                    if v0 > v1 { 1 } else { 0 }
                },
                6 => {
                    let v0 = execute(&ops[0]);
                    let v1 = execute(&ops[1]);
                    if v0 < v1 { 1 } else { 0 }
                },
                7 => {
                    let v0 = execute(&ops[0]);
                    let v1 = execute(&ops[1]);
                    if v0 == v1 { 1 } else { 0 }
                },
                _ => unreachable!(),
            }
        }
    }
}

fn main() {
    let mut input_buffer = String::new();
    std::io::stdin().read_line(&mut input_buffer).expect("Successful I/O");
    let mut bitstream = to_bitstream(&input_buffer);
    let packet = parse_packet(&mut bitstream);
    let soln_a = versions_sum(&packet);
    println!("Part a: {}", soln_a);
    let soln_b = execute(&packet);
    println!("Part b: {}", soln_b);
}
