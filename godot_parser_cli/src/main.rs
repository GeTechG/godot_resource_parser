use std::ffi::OsStr;
use std::fs;
use std::path::{Path, PathBuf};
use base64::Engine;
use base64::prelude::BASE64_STANDARD;
use clap::{Parser, Subcommand, ValueEnum, ValueHint};
use godot_data::godot_file::GodotFile;
use godot_data::nanoserde::{DeBin, DeJson, SerBin, SerJson};
use godot_data::tscn_file::TSCNFile;
use godot_parser_library::godot_parser::parse_godot_file;
use godot_parser_library::tscn_parser::parse_tscn_file;

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
enum Format {
    JSON,
    BIN,
}

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Command,

    #[arg(value_parser = clap::value_parser!(PathBuf), value_hint = ValueHint::FilePath)]
    path: PathBuf,

    #[arg(value_enum, default_value_t = Format::JSON)]
    format: Format,

    /// Print the output to stdout
    #[arg(short, long)]
    stdout: bool,

    /// The output file
    #[arg(short, long)]
    output: Option<PathBuf>,
}


#[derive(Subcommand)]
enum Command {
    FromGodot,
    FromFormat {
        extension: String,

        #[arg(value_enum, default_value_t = Format::JSON)]
        format_in: Format,
    }
}

trait Serializable: SerJson + SerBin {}
impl<T: SerJson + SerBin> Serializable for T {}

fn main() {
    let cli = Cli::parse();

    let ser_data: Box<dyn Serializable> = match cli.command {
        Command::FromGodot => {
            let file_contents = fs::read_to_string(&cli.path)
                .expect("Failed to read the file");
            let extension = cli.path.extension().and_then(OsStr::to_str).expect("Failed to get the file extension");
            match extension {
                "godot" => {
                    let (_, godot_file) = parse_godot_file(&file_contents).expect("Failed to parse the Godot file");
                    Box::from(godot_file)
                }
                "tscn" => {
                    let (_, tscn_file) = parse_tscn_file(&file_contents).expect("Failed to parse the Godot file");
                    Box::from(tscn_file)
                }
                _ => {
                    panic!("Unsupported file extension");
                }
            }
        }
        Command::FromFormat { format_in, extension } => {
            match extension.as_str() {
                "godot" => {
                    match format_in {
                        Format::JSON => {
                            let file_contents = fs::read_to_string(&cli.path)
                                .expect("Failed to read the file");
                            let godot_file: GodotFile = GodotFile::deserialize_json(&file_contents).expect("Failed to deserialize the JSON file");
                            Box::from(godot_file)
                        }
                        Format::BIN => {
                            let file_contents = fs::read(&cli.path)
                                .expect("Failed to read the file");
                            let godot_file: GodotFile = GodotFile::deserialize_bin(file_contents.as_slice()).expect("Failed to deserialize the BIN file");
                            Box::from(godot_file)
                        }
                    }
                }
                "tscn" => {
                    match format_in {
                        Format::JSON => {
                            let file_contents = fs::read_to_string(&cli.path)
                                .expect("Failed to read the file");
                            let tscn_file = TSCNFile::deserialize_json(&file_contents).expect("Failed to deserialize the JSON file");
                            Box::from(tscn_file)
                        }
                        Format::BIN => {
                            let file_contents = fs::read(&cli.path)
                                .expect("Failed to read the file");
                            let tscn_file = TSCNFile::deserialize_bin(file_contents.as_slice()).expect("Failed to deserialize the BIN file");
                            Box::from(tscn_file)
                        }
                    }
                }
                _ => {
                    panic!("Unsupported file extension");
                }
            }
        }
    };

    match cli.format {
        Format::JSON => {
            if cli.stdout {
                println!("{}", ser_data.serialize_json());
            } else {
                let output_path = cli.output.unwrap_or(cli.path.with_extension("json"));
                fs::write(output_path, ser_data.serialize_json()).expect("Failed to write the output file");
            }
        }
        Format::BIN => {
            if cli.stdout {
                let bin = ser_data.serialize_bin();
                println!("{}", BASE64_STANDARD.encode(bin.as_slice()));
            } else {
                let output_path = cli.output.unwrap_or(cli.path.with_extension("bin"));
                fs::write(output_path, ser_data.serialize_bin()).expect("Failed to write the output file");
            }
        }
    }
}
