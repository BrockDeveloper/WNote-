pub use clap::{ArgGroup, Parser, Subcommand};

/**
 * wnote --override-default-folder <folder> <mode> [...<extra>]
 * wnote new file <file> --name <name>
 * wnote new url <download_url> --name <name>
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
#[clap(author = "Korb Industries", version, about = "Sono un gestore di note.", long_about = None)]
#[clap(propagate_version = true)]
pub struct Cli {
    /// Mode of operation
    #[clap(subcommand)]
    pub command: Command,
}

#[derive(Subcommand, Debug)]
pub enum Command {
    New {
        #[clap(subcommand)]
        command: Option<NewCommand>,

        #[clap(long, short)]
        name: Option<String>,

        #[clap(group = "new-content")]
        content: Option<String>,
    },
    List,
    Show {
        /// Copy the note shown to clipboard
        #[clap(long, short)]
        copy: bool,
        name: String,
    },
    Delete {
        /// Copy the deleted note to clipboard
        #[clap(long, short)]
        copy: bool,
        name: String,
    },
    Search {
        query: String,
    },
    Configure,
    Interactive,
}

#[derive(Subcommand, Debug)]
pub enum NewCommand {
    File {
        #[clap(group = "new-content")]
        path: String,
    },
    Url {
        #[clap(group = "new-content")]
        url: String,
    },
}
