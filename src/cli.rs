pub use clap::{ArgGroup, Parser, Subcommand};

/**
 * wnote --override-default-folder <folder> <mode> [...<extra>]
//// wnote new file <file> --name <name>
//// wnote new url <download_url> --name <name>
 * wnote new <content> --name <name>
 * wnote list
 * wnote show --copy <name>
//// wnote edit <name> <new_content>
 * wnote delete --copy <name>
 * wnote search <query>
 * wnote configure ...<key=value>
 * wnote help
 * wnote interactive
 */

#[derive(Parser, Debug)]
#[clap(author = "Korb Industries", version, about = "Sono un gestore di note. Made with 💖", long_about = None)]
#[clap(propagate_version = true)]

pub struct Cli {
    /// Mode of operation
    #[clap(subcommand)]
    pub command: Command,
}

#[derive(Subcommand, Debug)]
pub enum Command {
    /// Create a new note 
    New {
        #[clap(long, short)]
        name: Option<String>,

        content: String,
    },
    /// List all notes
    List,
    /// Show a note
    Show {
        /// Copy the note shown to clipboard
        #[clap(long, short)]
        copy: bool,
        name: String,
    },
    /// Delete a note
    Delete {
        /// Copy the deleted note to clipboard
        #[clap(long, short)]
        copy: bool,
        name: String,
    },
    /// Search for notes
    Search {
        query: String,
    },
    /// Configure wnote
    Configure,
    /// Run wnote in interactive mode
    Interactive,
}
