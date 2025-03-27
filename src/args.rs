use clap::Parser;

#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct AppArgs {
    /// add dir to check all .c coding style
    /// ____________________
    #[arg(short, long, verbatim_doc_comment)]
    pub dirs: Vec<String>,

    /// select your binay to check banned function
    /// please place all authorized functoin in allowlist.txt
    /// separated by an endline '\n'
    /// ____________________
    #[arg(short, long, verbatim_doc_comment)]
    pub binary: Option<String>,
}
