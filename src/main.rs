use anyhow::{anyhow, bail, Result};
use clap::Parser;
use std::{
    fs::File,
    io::{self, BufReader, Read},
    path::{Path, PathBuf},
};

mod errors;
mod hashing;
use hashing::{blake2b::Blake2b, shasum::ShaSum};

#[derive(Parser)]
#[clap(
    version,
    about = "Pure Rust utility which handles various checksum types"
)]
struct Args {
    #[clap(short = 'l', long = "length", help = "the bit length of the checksum")]
    bit_length: i32,

    #[clap(
        short = 't',
        long = "type",
        help = "the type of checksum (sha or blake)"
    )]
    checksum_type: String,

    #[clap(name = "FILEs", help = "the files to process")]
    file_path: Vec<PathBuf>,

    #[clap(short, long, help = "read checksums from the FILEs and check them")]
    check: bool,

    #[clap(long = "tag", help = "create a BSD-style checksum")]
    bsd: bool,

    #[clap(long, help = "read in binary mode")]
    binary: bool,

    #[clap(short, long, help = "read data from stdin")]
    stdin: bool,
}

/// Get the file name in a path, like the `basename` Linux command
fn basename(file: &Path) -> Result<String> {
    Ok(file
        .file_name()
        .ok_or_else(|| anyhow!("File doesn't exist."))?
        .to_string_lossy()
        .into_owned()
        .chars()
        .filter(|&x| x != '\u{FFFD}')
        .collect())
}

fn main() -> Result<()> {
    let args = Args::parse();

    for file in &args.file_path {
        let mut contents = Vec::new();

        if !args.stdin {
            let mut reader = BufReader::new(File::open(file)?);
            if args.binary {
                reader.read_to_end(&mut contents)?;
            } else {
                reader.read_to_end(&mut contents)?;
            }
        } else {
            io::stdin().read_to_end(&mut contents)?;
        }

        match args.checksum_type.as_str() {
            "sha" => {
                let hasher = ShaSum::new(args.bit_length, &contents)?;
                let checksum = hasher.get_checksum()?;

                let r#type = if args.bit_length == 160 {
                    1
                } else {
                    args.bit_length
                };

                if args.bsd {
                    println!(
                        "SHA{} ({}) = {}",
                        r#type,
                        basename(&file)?,
                        checksum
                    );
                } else {
                    println!("{} {}", checksum, basename(&file)?);
                }
            }
            "blake" | "b2" | "blake2" => {
                let hasher = Blake2b::new(args.bit_length, &contents)?;
                let checksum = hasher.get_checksum()?;
                if args.bsd {
                    println!(
                        "BLAKE2b-{} ({}) = {}",
                        args.bit_length,
                        basename(&file)?,
                        checksum
                    );
                } else {
                    println!("{}  {}", checksum, basename(&file)?);
                }
            }
            _ => bail!("Invalid checksum type. Possible values are `sha` or `blake`."),
        }
    }

    Ok(())
}
