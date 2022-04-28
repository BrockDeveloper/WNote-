use clap::StructOpt;
use wnote::{
    cli::{Cli, Command},
    App,
};

fn main() {
    let cli = Cli::parse();
    let mut app = App::new();
    let path = std::path::PathBuf::from("./notes");

    // Creo la cartella se non esiste
    if let Err(err) = std::fs::create_dir_all(&path) {
        eprintln!("{}", err);
        std::process::exit(1);
    }

    if let Err(err) = app.load(&path) {
        eprintln!("{}", err);
        std::process::exit(1);
    }

    match cli.command {
        Command::New { name, content } => {
            let default_name = match name {
                Some(name) => name,
                None => {
                    let now = chrono::Local::now();
                    now.format("%Y-%M-%d").to_string()
                }
            };

            // TODO: Remove this
            println!("{}: {}", default_name, content);
        }
        Command::List => {
            for note in &app.notes {
                println!("{}: {}", note.name, note.content);
            }
        }
        Command::Show { copy, name } => {
            println!("show copy?{} name={}", copy, name);
        }
        Command::Delete { copy, name } => {
            println!("delete copy?{} name={}", copy, name);
        }
        Command::Search { query } => {
            println!("search query={}", query);
        }
        Command::Configure => {
            println!("configure");
        }
        Command::Interactive => {
            println!("interactive");
        }
    }

    if let Err(err) = app.save(&path) {
        eprintln!("{}", err);
        std::process::exit(1);
    }
}
