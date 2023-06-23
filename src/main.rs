use bech32::{ToBase32, Variant};
use clap::{Parser, ValueEnum};

#[derive(Parser)]
#[command(name = "lnurl-tool")]
#[command(author = "0xtr. <oxtrr@protonmail.com")]
#[command(version = "0.1")]
struct Cli {
    /// What operation to execute
    #[arg(value_enum)]
    mode: Mode,
    /// The data to be operated on
    data: String,
}

#[derive(ValueEnum, Clone, Debug)]
enum Mode {
    ENCODE,
    DECODE,
}

fn main() {
    let cli = Cli::parse();

    match cli.mode {
        Mode::ENCODE => {
            let encoded = encode(&cli.data);
            println!("{}", encoded);
        }
        Mode::DECODE => {
            let decoded = decode(&cli.data);
            println!("{}", decoded);
        }
    }
}

pub fn encode(url: &str) -> String {
    let base32 = url.as_bytes().to_base32();
    bech32::encode("lnurl", base32, Variant::Bech32).unwrap()
}

fn decode(s: &str) -> String {
    if s.to_lowercase().starts_with("lnurl") {
        let (_, data, _) = bech32::decode(s).expect("Something went wrong");
        let bytes = bech32::FromBase32::from_base32(&data).expect("Something went wrong");
        let url = String::from_utf8(bytes).expect("Something went wrong");
        url
    } else {
        eprintln!("Invalid lnurl");
        String::from("")
    }
}
