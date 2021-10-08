use byteorder::{ByteOrder, LE};
use clap::Clap;
use std::{io::{Read, Write}, path::PathBuf};

#[derive(Clap)]
#[clap(version = "0.0.1", author = "Rose Hudson <thomhuds@protonmail.com>")]
struct Args {
    #[clap(short, long)]
    overwrite: bool,
    #[clap(parse(from_os_str))]
    input: PathBuf,
    #[clap(parse(from_os_str))]
    output: PathBuf,
    #[clap(short, long)]
    entry_point: Option<String>,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    let mut input_file = std::fs::File::open(args.input)?;
    let input_len = input_file.metadata()?.len() as usize;
    let output_len = if args.overwrite { input_len } else { input_len + 64 };
    let mut data = vec![0; output_len];

    if args.overwrite {
        input_file.read_exact(&mut data[..])?;
    } else {
        input_file.read_exact(&mut data[64..])?;
    }

    // code0
    let branch_64_bytes_ahead = 0x14000010;
    LE::write_u32(&mut data[0..4], branch_64_bytes_ahead);

    // text_offset
    let mut entry_point = args.entry_point
        .map(|s| u64::from_str_radix(s.trim_start_matches("0x"), 16).ok())
        .flatten()
        .unwrap_or(0);
    if !args.overwrite {
        entry_point += 64;
    }
    LE::write_u64(&mut data[8..16], entry_point);

    // image_size
    LE::write_u64(&mut data[16..24], output_len as u64);

    // flags
    // botch the flags for now. who cares
    LE::write_u64(&mut data[24..32], 2);

    // magic
    LE::write_u32(&mut data[56..60], 0x644d5241);

    let mut output_file = std::fs::File::create(args.output)?;
    output_file.write_all(&data)?;
    Ok(())
}
