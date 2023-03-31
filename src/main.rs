
mod logseq_api;
mod clap_cli;

use clap::Parser;


fn main() {
    let args = clap_cli::Cli::parse();
    let mut client = logseq_api::Client::new();

    match &args.command {
        // clap_cli::Commands::Tasks(task) => match &task.command {
        //     clap_cli::TaskCommands::Add(add) => {
        //         client.add_journal_note(&add.text);
        //         println!("Task added: {:?}", add.text);
        //     }
        // },
        clap_cli::Commands::Journal(journal) => {
            client.add_journal_note(&journal.text);
        }
    }
}
