use anyhow::{anyhow, Result};
use clap::Parser;
use std::{
    fmt,
    fs::File,
    io::{self, BufRead, BufReader, Read},
    os::unix::ffi::OsStrExt,
    path::PathBuf,
};

mod hashing;
use hashing::ShaSum;

#[derive(Parser)]
#[clap(
    version,
    about = "Pure Rust utility which handles SHA-2 and SHA-1 checksums"
)]
struct Args {
    #[clap(
        short = 't',
        long = "type",
        name = "CHECKSUM",
        help = "the type of SHA checksum (1, 256, 512)"
    )]
    checksum_type: i32,

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

trait ReadToEnd {
    fn read_to_end_slice(&mut self) -> Result<Vec<u8>>;
}

impl<R: Read> ReadToEnd for BufReader<R> {
    fn read_to_end_slice(&mut self) -> Result<Vec<u8>> {
        let mut tmp: Vec<u8> = Vec::new();
        self.read_to_end(&mut tmp)?;
        Ok(tmp)
    }
}

/// Custom formatter for Vec<u8> to print bytes in hexadecimal format
struct HexBytes(Vec<u8>);

impl fmt::Display for HexBytes {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for byte in &self.0 {
            let b = *byte as u8;
            write!(f, "{}", b as char)?;
        }
        Ok(())
    }
}

fn main() -> Result<()> {
    let basename = |file: &PathBuf| -> Result<Vec<u8>> {
        Ok(file
            .to_path_buf()
            .file_name()
            .ok_or_else(|| anyhow!("File doesn't exist."))?
            .to_os_string()
            .as_bytes()
            .to_vec())
    };

    let args = Args::parse();

    for file in &args.file_path {
        let mut contents = Vec::new();

        if args.stdin {
            io::stdin().read_to_end(&mut contents)?;
        } else {
            let mut reader = BufReader::new(File::open(file)?);
            if args.binary {
                contents.extend(reader.bytes().flatten());
            } else {
                reader.read_to_end(&mut contents)?;
            }
        }

        let checksum = match args.checksum_type {
            1 | 256 | 512 => {
                let checksummer = ShaSum::new(args.checksum_type, &contents)?;
                checksummer.get_checksum()
            }
            _ => return Err(anyhow!("Invalid type: {}", args.checksum_type)),
        };

        let file_path_checksum = HexBytes(basename(file)?);

        if args.check {
            let expected_checksum = read_expected_checksum(&checksum, file, args.bsd)?;
            if checksum == expected_checksum {
                println!("{}: OK", file_path_checksum);
            } else {
                println!("{}: MISMATCH", file_path_checksum);
            }
        } else if args.bsd {
            println!(
                "SHA{} ({}) = {}",
                args.checksum_type, file_path_checksum, checksum
            );
        } else {
            println!("{}  {}", checksum, file_path_checksum);
        }
    }

    Ok(())
}

fn read_expected_checksum(
    calculated_checksum: &str,
    file_path: &PathBuf,
    bsd: bool,
) -> Result<String> {
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);

    if bsd {
        for line in reader.lines() {
            let line = line?;
            if line.starts_with(&format!(
                "SHA{} ({} )= ",
                calculated_checksum.len() * 8,
                file_path.display()
            )) {
                return Ok(line
                    .trim_start_matches(&format!(
                        "SHA{} ({}) = ",
                        calculated_checksum.len() * 8,
                        file_path.display()
                    ))
                    .to_string());
            }
        }
    } else {
        for line in reader.lines() {
            let line = line?;
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() == 2 && parts[1] == file_path.to_string_lossy() {
                return Ok(parts[0].to_string());
            }
        }
    }
    Err(anyhow!("Checksum not found for file: {:?}", file_path))
}
