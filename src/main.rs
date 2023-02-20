use std::net::UdpSocket;
use std::str;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let listener = UdpSocket::bind("localhost:5053")?;
    println!("Hello, world! I am a DNS Server!");
    let mut buf  = [0u8; 1024];
    // initialize threads and channels
    loop {
        if let Ok((size, addr)) = listener.recv_from(&mut buf) {
            let slice = &buf[..size];
            println!("SIZE:  {size}");
            println!("RAW {slice:02X?}");
            // Parse DNS query
            // let packet = if size == 51 {
            //
            // } else {
            //
            // }
            // Additional processing, if query not in the sqlite db, then check other dns servers
            // decide if sqlite is best or if another simpler backend might be better.
            // if the valid entry is there.
            if let Err(e) = listener.connect(addr){
                println!("{}", e.to_string());
            } else {
                if let Err(e) = listener.send(slice) {
                    println!("{}", e.to_string());
                }
            }
            // Ok((size, addr)) => {
            //
            //     if size > buf.len() {
            //         continue
            //     }
            //     let slice = &buf[..size];
            //     println!("RAW {slice:02X?}");
            //     println!("{}", slice.iter().enumerate().filter_map(|(i, &c )| {
            //         if c >= 97 && c <= 122 {
            //             println!("BIT! {i}, {} {} {c:X?}", c, c as char);
            //             Some(c as char)
            //         } else {
            //             None
            //         }
            //     } ).collect::<String>());
            //     //connect back to the client.
            //     println!("{addr:?}");
            // }
            // Drop/Ignore the Datagram
            // Err(_) => {
            //     println!("Not great error handling");
            //     continue
            // }
        } else {
            println!("ERR DATAGRAM, {}", line!());
        }
    }
}
