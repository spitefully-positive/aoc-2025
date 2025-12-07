use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    /// An array of days, ranging from 1 to 25, that should be solved
    pub days: Vec<u8>,
}
