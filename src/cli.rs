use clap::Parser;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    /// Characters to extract from the input
    #[arg(short, long, action=clap::ArgAction::Append, required=true)]
    pub characters: Vec<usize>,
}
