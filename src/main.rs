use clap::StructOpt;
use wnote::cli::{Cli, Command, NewCommand};

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Command::New { command, name, content } => {
            let default_name = match name {
                Some(name) => name,
                None => {
                    let now = chrono::Local::now();
                    now.format("%Y-%M-%d").to_string()
                }
            };

            // TODO: Remove this
            println!("{}", default_name);

            match command {
                Some(NewCommand::File { path }) => println!("new file path={}", path),
                Some(NewCommand::Url { url }) => println!("new url url={}", url),
                None => println!("new content={}", content.unwrap()),
            }
        }
        Command::List => {
            println!("list");
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
}
