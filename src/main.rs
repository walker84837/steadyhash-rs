use std::{
    process::ExitCode,
    fs::File,
    io::Read,
    path::PathBuf,
};
use structopt::StructOpt;

mod hashing;

/// A Windows port of the sha256sum command: print or check SHA256 (256-bit) checksums.
#[derive(StructOpt)]
#[structopt(name = "sha256sum-win")]
struct Args {
    /// The file's path.
    #[structopt(parse(from_os_str))]
    path: PathBuf,

    #[structopt(short, long)]
    check: bool,
}

fn main() -> ExitCode {
    let opt = Args::from_args();

    let mut contents = Vec::new();

    match File::open(&opt.path) {
        Ok(mut file) => {
            if let Err(e) = file.read_to_end(&mut contents) {
                eprintln!("Error reading file {}: {}", opt.path.display(), e);
                return ExitCode::FAILURE;
            }
        }
        Err(e) => {
            eprintln!("Error opening file {}: {}", opt.path.display(), e);
            return ExitCode::FAILURE;
        }
    }

    let checking = opt.check;

    if !checking {
        let checksum: String = hashing::Sha256Sum::get_checksum(&contents);
        println!("{}  {}", checksum, opt.path.display());
    } else {
        let contents_str = match std::str::from_utf8(&contents) {
            Ok(s) => s,
            Err(e) => {
                eprintln!("Error converting binary data to string: {}", e);
                return ExitCode::FAILURE;
            }
        };

        let arr: Vec<&str> = contents_str.trim().split_whitespace().collect();
        let checksum_init = arr[0];
        let file_name = PathBuf::from(arr[1]);

        let mut contents_input = Vec::new();

        match File::open(&file_name) {
            Ok(mut file) => {
                if let Err(e) = file.read_to_end(&mut contents_input) {
                    eprintln!("Error reading file {}: {}", file_name.display(), e);
                    return ExitCode::FAILURE;
                }
            }
            Err(e) => {
                eprintln!("Error opening file {}: {}", file_name.display(), e);
                return ExitCode::FAILURE;
            }
        }

        let calculated_checksum = hashing::Sha256Sum::get_checksum(&contents_input);

        if checksum_init == calculated_checksum {
            println!("{}: OK", file_name.display());
        } else {
            eprintln!("{}: FAILED", file_name.display());
            eprintln!("sha256sum-win: WARNING: 1 computed checksum did NOT match");
        }
    }
    ExitCode::SUCCESS
}
