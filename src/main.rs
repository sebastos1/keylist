mod app;
mod config;
mod keys;

use clap::Parser;
use std::path::PathBuf;

#[derive(Parser, Debug)]
struct Args {
    #[arg(short, long)]
    config: Option<PathBuf>,
    #[arg(short, long)]
    theme: Option<String>,
    #[arg(short, long)]
    font: Option<String>,
    #[arg(long)]
    font_size: Option<f32>,
    #[arg(long)]
    columns: Option<u32>,
    #[arg(short, long)]
    mod_key: Option<String>,
}

fn main() {
    let args = Args::parse();

    // get config
    let path = args.config.map(PathBuf::from);
    let mut doc = config::load(path, args.mod_key);

    // overwrite with cli args
    if let Some(theme) = args.theme {
        doc.config.theme = match theme.to_lowercase().as_str() {
            "light" => config::Theme::Light,
            _ => config::Theme::Dark,
        };
    }
    if let Some(font) = args.font {
        doc.config.font = font;
    }
    if let Some(font_size) = args.font_size {
        doc.config.font_size = font_size;
    }
    if let Some(columns) = args.columns {
        doc.config.columns = columns;
    }

    app::run(doc);
}
