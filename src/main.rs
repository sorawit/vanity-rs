use ethers::core::k256::ecdsa::SigningKey;
use ethers::core::rand::thread_rng;
use ethers::core::utils::{secret_key_to_address, to_checksum};
use rayon::prelude::*;

use clap::Parser;

/// Simple program to generate a private key for an address with a certain prefix
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// The prefix of the address to search
    #[clap(short, long, value_parser)]
    prefix: String,

    /// Whether to search with checksum address contraint
    #[clap(short, long, value_parser, default_value_t = false)]
    checksum: bool,
}

fn main() {
    let args = Args::parse();
    let mut total = 0;
    let mut start_time = std::time::SystemTime::now();
    loop {
        let res: Vec<_> = (0..100000)
            .into_par_iter()
            .filter_map(|_| {
                let sk = SigningKey::random(thread_rng());
                let addr = secret_key_to_address(&sk);
                let addr_str = if args.checksum {
                    to_checksum(&addr, None)
                } else {
                    format!("0x{:x}", addr)
                };
                if addr_str.starts_with(&args.prefix) {
                    Some((sk, addr))
                } else {
                    None
                }
            })
            .collect();
        if res.len() > 0 {
            let (sk, addr) = &res[0];
            println!("FOUND IT!");
            println!("PRIVATE KEY: {:x}", sk.to_bytes());
            println!("ADDRESS: {}", to_checksum(addr, None));
            break;
        }
        total += 100000;
        println!(
            "total = {}; running at {} iterations per second",
            total,
            100000.0 / start_time.elapsed().unwrap().as_secs_f64(),
        );
        start_time = std::time::SystemTime::now();
    }
}
