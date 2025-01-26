use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long, default_value_t = String::new())]
    title: String,

    #[arg(short, long, default_value_t = String::new())]
    icon: String,
}

fn main() {
    let args = Args::parse();

    if !args.title.is_empty() {
        println!("Title: {}", args.title);
    }

    if !args.icon.is_empty() {
        println!("Icon: {}", args.icon);
    }
}
