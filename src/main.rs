#![feature(file_buffered)]

use clap::Parser;
use std::collections::HashMap;
use std::fs::File;
use std::io::{self, stdout};
use std::io::{BufWriter, Write};
use std::path::{Path, PathBuf};
use std::process::exit;

use serde::Deserialize;

#[derive(Parser)]
#[command(
    version,
    about = "Generate html flavoured markdown CRC cards from YAML."
)]
pub struct Args {
    #[arg(required=true, num_args=1..)]
    infiles: Vec<PathBuf>,

    #[arg(short, long)]
    outfile: Option<PathBuf>,
}

#[derive(Deserialize)]
pub struct CardBody {
    role: Vec<String>,
    collab: Vec<String>,
}

type CrcCards = HashMap<String, CardBody>;

fn read_cards(path: &Path) -> anyhow::Result<CrcCards> {
    let reader = File::open_buffered(path)?;
    Ok(serde_yml::from_reader::<_, CrcCards>(reader)?)
}

pub trait CardFormatter {
    fn write_to(writer: &mut BufWriter<dyn Write>, cards: &CrcCards) -> io::Result<usize>;
}

pub struct MarkdownFormatter;

impl CardFormatter for MarkdownFormatter {
    fn write_to(writer: &mut BufWriter<dyn Write>, cards: &CrcCards) -> io::Result<usize> {
        let mut bytes_written = 0;
        for (name, body) in cards.iter() {
            // Write card header
            bytes_written += writer.write(b"\n# ")?;
            bytes_written += writer.write(name.as_bytes())?;

            // Finish the card header line and write the table header row
            bytes_written += writer
                .write(b"\n<table><tr><th>Responsibilities</th><th>Collaborators</th></tr>")?;

            // Write rows
            for i in 0..body.role.len().max(body.collab.len()) {
                // Start row and first table data
                bytes_written += writer.write(b"<tr><td>")?;
                if i < body.role.len() {
                    bytes_written += writer.write(body.role[i].as_bytes())?;
                }

                // End first table data and start second
                bytes_written += writer.write(b"</td><td>")?;

                if i < body.collab.len() {
                    bytes_written += writer.write(body.collab[i].as_bytes())?;
                }
                // End second table data and row
                bytes_written += writer.write(b"</td></tr>")?;
            }

            // End table
            bytes_written += writer.write(b"</table>\n")?;
        }
        Ok(bytes_written)
    }
}

pub struct SvgFormatter;

impl CardFormatter for SvgFormatter {
    fn write_to(writer: &mut BufWriter<dyn Write>, cards: &CrcCards) -> io::Result<usize> {
        let mut bytes_written = 0;

        Ok(bytes_written)
    }
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    let mut writer: Box<BufWriter<dyn Write>> = match args.outfile {
        Some(path) => Box::new(File::create_buffered(path)?),
        // If no output file is specified write to stdout.
        None => Box::new(BufWriter::new(stdout())),
    };

    for path in args.infiles {
        let cards = match read_cards(&path) {
            Ok(cards) => cards,
            Err(e) => {
                eprintln!(
                    "Failed to read cardfile `{:?}`\n{:?}",
                    path.to_str().unwrap_or("<cannot display>"),
                    e
                );
                exit(1);
            }
        };
        // If failing to write theres not much we can do.
        // Return the error to anyhow and stop the program.
        MarkdownFormatter::write_to(&mut writer, &cards)?;
    }

    writer.flush()?;
    Ok(())
}
