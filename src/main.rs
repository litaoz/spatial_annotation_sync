use std::{io::{self, Write}, str::FromStr};

use clap::Parser;
use thiserror::Error;
use colored::Colorize;
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
        let mut tokens = shell_words::split(s.trim()).map_err(|_| ParseError::UnmatchedQuote)?.into_iter();
        let cmd = tokens.next().ok_or(ParseError::EmptyInput)?;
        let mut rest = tokens;

        match cmd.as_str() {
            "exit" => { Ok(Command::Exit) }
            "list" => { Ok(Command::List) }
            "add" => {
                let id = rest.next().ok_or(ParseError::IncorrectArgumentCount)?;
                let coord = rest.next().ok_or(ParseError::IncorrectArgumentCount)?;
                let text = rest.next().ok_or(ParseError::IncorrectArgumentCount)?;

                let id = id.parse().map_err(|_| ParseError::NotU128)?;
                let coord = coord.parse().map_err(|_| ParseError::NotPointFormated)?;
                let text = text.parse().map_err(|_| ParseError::NotString)?;

                Ok(Command::Add{id, text, coord})
            }
            "edit" => {
                let id = rest.next().ok_or(ParseError::IncorrectArgumentCount)?;
                let text = rest.next().ok_or(ParseError::IncorrectArgumentCount)?;

                // let id = id.parse().map_err(|_| ParseError::NotU128)?;
                // let text = text.parse().map_err(|_| ParseError::NotString)?;
                unimplemented!("not written yet")
                // Edit{id: u128, text: String},
            }
            "move" => {
                let id = rest.next().ok_or(ParseError::IncorrectArgumentCount)?;
                let coord = rest.next().ok_or(ParseError::IncorrectArgumentCount)?;
                let text = rest.next().ok_or(ParseError::IncorrectArgumentCount)?;

                // let id = id.parse().map_err(|_| ParseError::NotU128)?;
                // let coord = coord.parse().map_err(|_| ParseError::NotPointFormated)?;
                // let text = text.parse().map_err(|_| ParseError::NotString)?;
                unimplemented!("not written yet")
                // Move{id: u128, coord: Point},
            }
            "delete" => {
                let id = rest.next().ok_or(ParseError::IncorrectArgumentCount)?;
                // let id = id.parse().map_err(|_| ParseError::NotU128)?;
                unimplemented!("not written yet")
                // Delete{id: u128},
            }
            "sync" => {
                unimplemented!("not written yet")
                // Sync{peer: String}
            }
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

    #[error("Unmatched quote")]
    UnmatchedQuote,

    #[error("Argument should be a u128")]
    NotU128,
    #[error("Argument should be a String")]
    NotString,
    #[error("Argument should be a (x, y)")]
    NotPointFormated,
}

fn list_command(env: &SpatialEnvironment) {
    let annotations = env.list_annotation();
    for ann in annotations {
        let id = ann.get_id().map_or(String::from(""), |i| format!("{:}", i));
        let coord = ann.get_coord().map_or(String::from(""), |i| format!("{:?}", i));
        let text = ann.get_text().map_or("", |s| s.as_str());

        println!(
            "{:7}  {:13}  {}",
            id, coord.red(), text.blue()
            // id, coord, text
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
        Command::Add { id, text, coord } => {
            env.create_annotation(
                SpatialAnnotation::new(
                    Some(AnnotationId::new(id)),
                    coord,
                    text
                )
            );
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