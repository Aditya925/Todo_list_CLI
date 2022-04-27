use std::path::PathBuf;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]

pub enum Action{
    Add{
        #[structopt()]
        text: String,
    },
    Done{
        #[structopt()]
        position: usize,
    },
    List,
}

#[derive(Debug, StructOpt)]
#[structopt(
    name = "todolist",
    about = "A command line to_do app in rust"
)]
pub struct CommandLineArgs{
    #[structopt(subcommand)]
    pub action:Action,
    #[structopt(parse(from_os_str), short, long)]
    pub journal_file: Option<PathBuf>,
}
