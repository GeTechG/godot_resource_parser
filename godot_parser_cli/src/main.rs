use std::any::Any;
use std::ffi::OsStr;
use std::fs;
use std::path::{PathBuf};
use base64::Engine;
use base64::prelude::BASE64_STANDARD;
use clap::{Parser, Subcommand, ValueEnum, ValueHint};
use godot_data::bincode;
use godot_data::bincode::config;
use godot_data::project_file::ProjectFile;
use godot_data::nanoserde::{DeJson, DeRon, SerJson, SerRon};
use godot_data::tscn_file::TSCNFile;
use godot_parser_library::project_parser::parse_project_file;
use godot_parser_library::tscn_tres_parser::{parse_tres_file, parse_tscn_file};

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
enum Format {
    JSON,
    BIN,
    RON
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

fn main() {
    let cli = Cli::parse();

    let extension: &str;
    let config = config::standard();

    let ser_data: Box<dyn Any> = match cli.command {
        Command::FromGodot => {
            let file_contents = fs::read_to_string(&cli.path)
                .expect("Failed to read the file");
            let _extension = cli.path.extension().and_then(OsStr::to_str).expect("Failed to get the file extension");
            match _extension {
                "godot" => {
                    extension = "bin";
                    let (_, godot_file) = parse_project_file(&file_contents).expect("Failed to parse the Godot file");
                    Box::from(godot_file)
                }
                "tscn" => {
                    extension = "scn";
                    let (_, tscn_file) = parse_tscn_file(&file_contents).expect("Failed to parse the Godot file");
                    Box::from(tscn_file)
                }
                "tres" => {
                    extension = "res";
                    let (_, tscn_file) = parse_tres_file(&file_contents).expect("Failed to parse the Godot file");
                    Box::from(tscn_file)
                }
                _ => {
                    panic!("Unsupported file extension");
                }
            }

        }
        Command::FromFormat { format_in, extension: _extension } => {
            match _extension.as_str() {
                "godot" => {
                    extension = "bin";
                    match format_in {
                        Format::JSON => {
                            let file_contents = fs::read_to_string(&cli.path)
                                .expect("Failed to read the file");
                            let godot_file: ProjectFile = ProjectFile::deserialize_json(&file_contents).expect("Failed to deserialize the JSON file");
                            Box::from(godot_file)
                        }
                        Format::BIN => {
                            let file_contents = fs::read(&cli.path)
                                .expect("Failed to read the file");
                            let (godot_file, _): (ProjectFile, _) = bincode::decode_from_slice(file_contents.as_slice(), config).expect("Failed to deserialize the BIN file");
                            Box::from(godot_file)
                        }
                        Format::RON => {
                            let file_contents = fs::read_to_string(&cli.path)
                                .expect("Failed to read the file");
                            let godot_file: ProjectFile = ProjectFile::deserialize_ron(&file_contents).expect("Failed to deserialize the RON file");
                            Box::from(godot_file)
                        }
                    }
                }
                "tscn" => {
                    extension = "scn";
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
                            let (tscn_file, _): (TSCNFile, _) = bincode::decode_from_slice(file_contents.as_slice(), config).expect("Failed to deserialize the BIN file");
                            Box::from(tscn_file)
                        }
                        Format::RON => {
                            let file_contents = fs::read_to_string(&cli.path)
                                .expect("Failed to read the file");
                            let tscn_file = TSCNFile::deserialize_ron(&file_contents).expect("Failed to deserialize the RON file");
                            Box::from(tscn_file)
                        }
                    }
                }
                "tres" => {
                    extension = "res";
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
                            let (tscn_file, _): (TSCNFile, _) = bincode::decode_from_slice(file_contents.as_slice(), config).expect("Failed to deserialize the BIN file");
                            Box::from(tscn_file)
                        }
                        Format::RON => {
                            let file_contents = fs::read_to_string(&cli.path)
                                .expect("Failed to read the file");
                            let tscn_file = TSCNFile::deserialize_ron(&file_contents).expect("Failed to deserialize the RON file");
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
            let data = if let Some(godot_file) = ser_data.downcast_ref::<ProjectFile>() {
                godot_file.serialize_json()
            } else if let Some(tscn_file) = ser_data.downcast_ref::<TSCNFile>() {
                tscn_file.serialize_json()
            } else {
                panic!("Failed to downcast the data");
            };
            if cli.stdout {
                println!("{}", data);
            } else {
                let output_path = cli.output.unwrap_or(cli.path.with_extension(extension));
                let output_dir = output_path.parent().expect("Failed to get the parent directory");
                fs::create_dir_all(output_dir).expect("Failed to create the output directory");
                fs::write(output_path, data).expect("Failed to write the output file");
            }
        }
        Format::BIN => {
            let data = if let Some(godot_file) = ser_data.downcast_ref::<ProjectFile>() {
                bincode::encode_to_vec(&godot_file, config).expect("Failed to serialize the data")
            } else if let Some(tscn_file) = ser_data.downcast_ref::<TSCNFile>() {
                bincode::encode_to_vec(&tscn_file, config).expect("Failed to serialize the data")
            } else {
                panic!("Failed to downcast the data");
            };
            if cli.stdout {
                println!("{}", BASE64_STANDARD.encode(data.as_slice()));
            } else {
                let output_path = cli.output.unwrap_or(cli.path.with_extension(extension));
                let output_dir = output_path.parent().expect("Failed to get the parent directory");
                fs::create_dir_all(output_dir).expect("Failed to create the output directory");
                fs::write(output_path, data).expect("Failed to write the output file");
            }
        }
        Format::RON => {
            let data = if let Some(godot_file) = ser_data.downcast_ref::<ProjectFile>() {
                godot_file.serialize_ron()
            } else if let Some(tscn_file) = ser_data.downcast_ref::<TSCNFile>() {
                tscn_file.serialize_ron()
            } else {
                panic!("Failed to downcast the data");
            };
            if cli.stdout {
                println!("{}", data);
            } else {
                let output_path = cli.output.unwrap_or(cli.path.with_extension(extension));
                let output_dir = output_path.parent().expect("Failed to get the parent directory");
                fs::create_dir_all(output_dir).expect("Failed to create the output directory");
                fs::write(output_path, data).expect("Failed to write the output file");
            }
        }
    }
}
