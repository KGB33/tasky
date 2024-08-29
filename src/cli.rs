use clap::Parser;

/// A natural language front end for Taskwarrior, powered by AI.
#[derive(Parser, Debug)]
#[command(version, about, long_about=None)]
pub struct Cmd {
    pub prompt: String,

    #[arg(short = 'y', long, default_value_t = false)]
    pub auto_confirm: bool,
}
