// const
use num_derive::FromPrimitive;
use num_traits::FromPrimitive;
use std::ops::Index;

struct DNSRequest {
    header: DNSHeader,
    question: Question,
}

// impl DNSRequest {
//     fn new(packet: Option<&[u8]>) -> Self {
//         if let Some(packet) = packet {
//
//         } else {
//
//         }
//     }
// }

struct DNSResponse {
    header: DNSHeader,
    response: Response,
}

struct Question {
    qname: [u8; 2],
    qtype: [u8; 2],
    // q
}

struct Response {}

struct DNSHeader {
    id: [u8; 2],
    // might make this another struct
    flags: Flags,
    qdcount: [u8; 2],
    ancount: [u8; 2],
    arcount: [u8; 2],
}

enum QRBit {
    Query,
    Response,
}

struct Flags {
    dns_type: QRBit,
    opcode: Opcode,
    truncated: bool,
    recursion: bool,
    // reserved <- bit
    ad_bit: bool,
    non_auth: bool,
}

#[derive(FromPrimitive)]
enum Opcode {
    // 0
    Query = 0,
    // 1
    IQuery = 1,
    // 2
    Status = 2,
    // bits 3, 6-15, we do not want to use this anyways
    // TODO: assign this? probably wont use complex types. This means that the addition  of
    //   another crate, which means longer compile times.
    // NotAssigned,
    // 4
    Notify = 4,
    // 5
    Update = 5,
}

impl Flags {
    // TODO: Custom Error (possibly use anyhow?), or leave as string?
    fn new(flags: &u8) -> Result<Self, String> {
        let up_opcode = Flags::format_opcode(flags);
        //Iterates through possible bit values that Opcode can be,
        let opcode = (0..15)
            .into_iter()
            .rev()
            .find(|&val| bitflags_check(val, &up_opcode))
            .and_then(Opcode::from_u8)
            // TODO: Decide if multiple conditions are necessary, or if an optional is better.
            .ok_or_else(|| format!("DNS_TYPE: INVALID TYPE: {up_opcode}, AS BITS: {up_opcode:08b}"))?;
        let dns_type = if bitflags_check(0, flags) {
            QRBit::Response
        } else {
            QRBit::Query
        };
        Ok(Self {
            dns_type,
            opcode,
            truncated: false,
            recursion: false,
            ad_bit: false,
            non_auth: false,
        })
    }
    // truncates the first bit and the last 3 bits, this way we only have the opcode bits
    fn format_opcode(flag_bit: &u8) -> u8 {
        let flag_bit = flag_bit >> 1;
        let flag_bit = flag_bit << 3;
        flag_bit >> 3
    }

}
fn bitflags_check(bit: u8, bits: &u8) -> bool {
    (bits & (1 << bit)) == 1
}

#[cfg(test)]
mod tests{
    use super::*;
    #[test]
    fn bits_test(){
        let bits = 1;
        let bit = 0;
        let x: Option<&str> = None;
        assert!(bitflags_check(bit, &bits), "Function does not work");
    }
}