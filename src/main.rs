use std::io::{stdout, Cursor, Read, Write};

use clap::{ArgEnum, Parser};
use ipld::{
    codec::{Decode, Encode},
    IpldCodec,
};
use libipld as ipld;

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct Cli {
    /// What mode to run the program in
    #[clap(arg_enum, short, long, default_value_t = Mode::CBOR)]
    r#in: Mode,
    #[clap(arg_enum)]
    #[clap(arg_enum, short, long, default_value_t = Mode::JSON)]
    out: Mode,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ArgEnum)]
enum Mode {
    JSON,
    CBOR,
}

fn main() {
    let cli = Cli::parse();

    let input_type = match cli.r#in {
        Mode::JSON => IpldCodec::DagJson,
        Mode::CBOR => IpldCodec::DagCbor,
    };
    let output_type = match cli.out {
        Mode::JSON => IpldCodec::DagJson,
        Mode::CBOR => IpldCodec::DagCbor,
    };

    let mut in_data = vec![];
    std::io::stdin()
        .read_to_end(&mut in_data)
        .expect("failed to read stdin");
    let mut c = Cursor::new(in_data);

    let v = ipld::Ipld::decode(input_type, &mut c).expect("Failed to decode");
    let mut out = Vec::new();
    v.encode(output_type, &mut out).expect("Failed to encode");

    stdout().write_all(&out).expect("Failed to write stdout");
}
