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
//           0     Query                              [RFC1035] ?
//           1     IQuery (Inverse Query, Obsolete)   [RFC3425] Do not need to implement
//           2     Status                             [RFC1035] ?
//           3     available for assignment                     Do not need to implement
//           4     Notify                             [RFC1996] ?
//           5     Update                             [RFC2136] ?
//          6-15   available for assignment
// RAW [
// DA, C5,
// 01, 20,
// 00, 01,
// 00, 00,
// 00, 00,
// 00, 01,
// 06, 67, 6F, 6F, 67, 6C, 65, 03, 63, 6F, 6D, 00,
// 00, 01,
// 00, 01,
// 00, 00, 29, 04, D0, 00, 00, 00, 00, 00, 0C, 00, 0A, 00, 08, 2F, E5, B0, EA, 29, E3, D3, 62]
// slice is a dns packet
// two bytes = transaction ID                                       a3 bd
// two bytes = flags (first bit query/response, next 4 opcode, 1 bit truncated, 1 bit recursion, 1 bit reserved, 1 bit AD bit, 1 bit Non-auth data
//                                                                  01 20
// two bytes = questions (amount of queries asked)                  00 01
// two bytes = awnser RR's                                          00 01
// two bytes = authority RR's                                       00 00
// two bytes = additional RR's                                      00 01
// Queries 13 bytes can be variable
//   name: variable, null terminated (ends in 00)
//   type: two bytes, 00 01 type A                                  00 01
//   class: IN                                                      00 01

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