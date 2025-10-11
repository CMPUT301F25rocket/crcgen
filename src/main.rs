use clap::Parser;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufReader, Write};
use std::path::{Path, PathBuf};

use serde::Deserialize;

#[derive(Deserialize)]
pub struct CardBody {
    responsibility: Vec<String>,
    collaborator: Vec<String>,
}

#[derive(Parser)]
pub struct Args {
    infiles: Vec<PathBuf>,
}

type CrcCards = HashMap<String, CardBody>;

fn read_cards(path: &Path) -> anyhow::Result<CrcCards> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    Ok(serde_yml::from_reader::<_, CrcCards>(reader)?)
}

// I'm going to be transfering this over net pretty often for use
// in the repo. Minimize the code size, under the assumption that 
// a bloated binary costs more workflow time.
fn format_table(output: &mut Box<dyn Write>, name: &str, body: &CardBody) {
    // Write card header.
    output.write(b"# ");
    output.write(name.as_bytes());
    output.write(b"\n<table><tr><th>Responsibilities</th><th>Collaborators</th></tr>");

    output.write(br#"<table><tr colspan="2"><b>"#);
    output.write(br#"</b></tr>"#);
    
    // Finish
    output.write(b"</table>\n");
}

fn main() {
    let args = Args::parse();
    for path in args.infiles {
        let cards = match read_cards(&path) {
            Ok(cards) => cards,
            Err(e) => {
                eprintln!("{:?}", e);
                continue;
            }
        }
        for (name, body) in cards.iter() {
            
        }
    }
}
