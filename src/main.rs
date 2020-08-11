use structopt::StructOpt;
use std::{str, fs::File};
use std::io::{Seek, SeekFrom, Read};
use std::str::from_utf8;
use std::convert::Infallible;
use std::str::{FromStr};

enum DisplayType {
    DisplayNewLine,
    DisplayChar,
    DisplayHex,
    DisplayDec,
}

impl FromStr for DisplayType {
    type Err = Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err>{
        match s {
            "char" => Ok(DisplayType::DisplayChar),
            "dec" => Ok(DisplayType::DisplayDec),
            "newline" => Ok(DisplayType::DisplayNewLine),
            _ => Ok(DisplayType::DisplayHex),
        }
    }
}

#[derive(StructOpt)]
struct Cli {
    #[structopt(short="o", long="output-format", default_value="hex")]
    output_fmt: DisplayType,

    #[structopt(parse(from_os_str))]
    path: Vec<std::path::PathBuf>,
}

fn main() {
    // Define a list of "newline" characters:
    let newline_chars: [u8; 4] = [10, 13, 21, 30];

    // Capture the commandline arguments:
    let args = Cli::from_args();

    // A single character buffer to hold the read character:
    let mut last_char = [0];

    // Loop over each input file glob:
    for input_path in args.path {
        // Atrempt to convert the OS path to a string:
        let file_path = input_path.to_str().expect("failed to parse input file");
        // Attempt to open the file handle:
        let mut file_handle = File::open(file_path).expect("failed to open input file");
        // Attempt to seek to the second to last byte in the file:
        file_handle.seek(SeekFrom::End(-1)).expect("failed to seek in input file");
        // Attempt to read the last character from the file:
        file_handle.read_exact(&mut last_char).expect("failed to read final character from file");
        // Print out the data:
        match args.output_fmt {
            DisplayType::DisplayChar => println!("{}\t{}", from_utf8(&last_char).expect("failed to decode character"), file_path),
            DisplayType::DisplayHex => println!("{:#X}\t{}", last_char[0], file_path),
            DisplayType::DisplayDec => println!("{}\t{}", last_char[0], file_path),
            DisplayType::DisplayNewLine => {
                let res = match newline_chars.iter().any(|v| v == &last_char[0]) {
                    true => "newline",
                    false => "other",
                };
                println!("{}\t{}", res, file_path);
            },
        }
    }
}
