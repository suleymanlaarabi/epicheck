use clap::Parser;

#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct AppArgs {
    #[arg(short, long)]
    pub dirs: Vec<String>,
}
