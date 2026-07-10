use std::{io::{self, Write}, str::FromStr};

use clap::Parser;
use thiserror::Error;
use spatial_annotation_sync::crdt::*;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// ID of the Peer, either 1 or 2
    #[arg(short, long)]
    peer_id: u8
}

enum Command {
    Exit,
    List,
    Add{id: u128, text: String, coord: Point},
    // Edit{id: u128, text: String},
    // Move{id: u128, coord: Point},
    // Delete{id: u128},
    // Sync{peer: String}
}

impl FromStr for Command {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut tokens = s.split_whitespace();
        let cmd = tokens.next().ok_or(ParseError::EmptyInput)?;
        let rest = tokens;

        match cmd {
            "exit" => {
                Ok(Command::Exit)
            }
            "list" => {
                Ok(Command::List)
            }
            // "add" => {
            //     let id = rest.next().ok_or(ParseError::IncorrectArgumentCount)?;
            //     let text = rest.next().ok_or(ParseError::IncorrectArgumentCount)?;
            //     let coord = rest.next().ok_or(ParseError::IncorrectArgumentCount)?;

            //     // TODO: replace
            //     Ok(Command::Exit)
            // }
            _ => {
                Err(ParseError::UnknownCommand)
            }
        }
    }
}

#[derive(Error, Debug)]
enum ParseError {

    #[error("Empty input")]
    EmptyInput,

    #[error("Unknown command")]
    UnknownCommand,

    #[error("Incorrect argument count")]
    IncorrectArgumentCount,

    #[error("Unknown error")]
    Unknown
}

fn list_command(env: &SpatialEnvironment) {
    let annotations = env.list_annotation();
    for ann in annotations {
        let id = ann.get_id().map_or(String::from(""), |i| format!("{:?}", i));
        let text = ann.get_text().map_or("", |s| s.as_str());
        let coord = ann.get_coord().map_or(String::from(""), |i| format!("{:?}", i));

        println!(
            "{} {} {}",
            id, text, coord
        )
    }
}

fn handle_command(env: &mut SpatialEnvironment, cmd: Command) -> Result<(), ()>{
    match cmd {
        Command::Exit => {
            Err(())
        }
        Command::List => {
            list_command(env);
            Ok(())
        }
    }
}

fn main() {
    // let args = Args::parse();
    // let port = match args.peer_id {
    //     1 => 3000,
    //     _ => 3001
    // };

    // let peer_port = match args.peer_id {
    //     1 => 3001,
    //     _ => 3000
    // };

    let mut spatial_env = SpatialEnvironment::new();
    spatial_env.create_annotation(SpatialAnnotation::new(
        Some(AnnotationId::new(1)),
        Point(0, 0),
        String::from("Front Door")
    ));
    spatial_env.create_annotation(SpatialAnnotation::new(
        Some(AnnotationId::new(2)),
        Point(1, 0),
        String::from("Bookshelf")
    ));
    spatial_env.create_annotation(SpatialAnnotation::new(
        Some(AnnotationId::new(3)),
        Point(2, 1),
        String::from("Bed")
    ));

    loop {
        let mut input = String::new();
        print!("> "); io::stdout().flush().unwrap();
        io::stdin().read_line(&mut input).unwrap();
        let cmd = Command::from_str(input.as_str());
        let res = match cmd {
            Err(error) => { println!("{}", error); Ok(())},
            Ok(cmd) => { handle_command(&mut spatial_env, cmd) }
        };
        match res {
            Ok(_) => {},
            Err(_) => {break}
        }
    }
}