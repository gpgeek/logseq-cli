
mod logseq_api;
mod clap_cli;

use clap::Parser;
use std::io;


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

            if &journal.text == "-" {
                let mut buffer = String::new();
                let stdin = io::stdin();

                match stdin.read_line(&mut buffer) {
                    Ok(_) => {
                        buffer = str::replace(&buffer, "\n", "\\n");
                        client.add_journal_note(&buffer);
                    },
                    Err(error) => {
                        println!("Error: {:?}", error);
                    },
                }
            } else {
                client.add_journal_note(&journal.text);
            }
        }
    }
}
