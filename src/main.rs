use std::path::PathBuf;
use clap::Parser;
use ruls::{run, model::Options};

#[derive(Parser)]
struct Args {
    #[arg(short = 'a', long = "all", default_value_t = false, help = "Include hidden files")]
    show_hidden: bool,

    #[arg(short = 'r', long = "reverse", default_value_t = false, help = "Reverse sort order")]
    reverse: bool,

    #[arg(short = 'l', long = "long", default_value_t = false, help = "Display long format")]
    long: bool,

    #[arg(short = 'B', long = "bytes", default_value_t = false, help = "Display file size in bytes")]
    bytes: bool,

    #[arg(long = "nocolor", default_value_t = false, help = "Don't use colored output")]
    nocolor: bool,

    paths: Vec<PathBuf>,
}

fn main() -> anyhow::Result<()> {
    // Parse CLI args
    let args = Args::parse();
    let opts = Options {
        show_hidden: args.show_hidden,
        reverse: args.reverse,
        long: args.long,
        bytes: args.bytes,
        nocolor: args.nocolor
    };

    run(&opts, &args.paths)?;
    Ok(())
}
