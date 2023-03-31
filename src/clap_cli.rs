use clap::{Args, Parser, Subcommand};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
pub struct Cli {
    #[command(subcommand)]
  pub  command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    // #[clap(alias = "t")]
    // Tasks(Tasks),
    #[clap(alias = "j")]
    Journal(Journal),
}

// #[derive(Subcommand)]
// pub enum TaskCommands {
//     Add(Add),
// }

// #[derive(Args)]
// pub struct Tasks {
//     #[command(subcommand)]
//   pub  command: TaskCommands,
// }

#[derive(Args)]
pub struct Journal {
  pub  text: String,
}

// #[derive(Args)]
// pub struct Add {
//   pub text: String,
//     #[arg(short, long)]
//     page: Option<String>,
//     #[arg(short, long)]
//     block_ref: Option<String>,
// }