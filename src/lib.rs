// const

struct DNSRequest {
    header: DNSHeader,
    question:DNSQuestion,
}

// impl DNSRequest {
//     fn new(byte_stream: Option<&[u8]>) -> Self {
//         Sel
//     }
// }

struct DNSResponse {

}

struct DNSQuestion {
    qname: [u8; 2],
    qtype: [u8; 2],
    // q
}


struct DNSHeader {
    id: [u8; 2],
    // might make this another struct
    flags: [u8; 2],
    qdcount: [u8; 2],
    ancount: [u8; 2],
    arcount: [u8; 2],
}
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

