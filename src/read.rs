use clap::Parser;
use deku::DekuContainerRead;
use std::path::Path;

use crate::request;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Name of the person to greet
    #[clap(short, long)]
    filename: String,
}

pub fn main() {
    let args = Args::parse();
    let path = Path::new(&args.filename);
    // Path::try_from(args.filename);
    // let p = Path::from(args.filename);

    println!("Using path {p}", p = path.display());

    let mut captured = pcap::Capture::from_file(path).expect("Expected file to open");

    loop {
        let pkt = match captured.next() {
            Ok(p) => p,
            Err(pcap::Error::NoMorePackets) => break,
            Err(e) => {
                panic!("Unexpected error reading packet: {e}")
            }
        };
        if pkt.header.len != pkt.header.caplen {
            panic!(
                "Expected packet len {} to equal caplen {}",
                pkt.header.len, pkt.header.caplen
            );
        }
        println!("header: {l}", l = pkt.header.len);

        println!("{:02X?}", pkt.data);
        let cut = &pkt.data[42..];

        println!("{:02X?}", cut);

        let res = request::Header::from_bytes((cut, 0));
        let (rest, req_header) = match res {
            Ok(rh) => rh,
            Err(e) => {
                println!("Failed reading packet: {e}");
                continue;
            }
        };

        println!(
            "Read successfully! Remaining: {}, {} / {}",
            rest.0.len(),
            rest.1,
            pkt.data.len()
        );
        println!("{:?}", req_header);

        // packets.push(pkt);
    }
}
