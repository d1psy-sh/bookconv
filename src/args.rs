use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    #[arg(short, long)]
    pub input: String,
    #[arg(short, long)]
    pub output: Option<String>,
}

pub struct Config {
    pub input_file: String,
    pub output_file: String,
}

impl Config {
    pub fn new() -> Config {
        let args = Args::parse();
        let input_file = args.input;
        let output_file = match args.output {
            Some(file) => file,
            None => {
                let mut s = input_file.clone();
                s.push_str(".html");
                s
            }
        };
        Config {
            input_file,
            output_file,
        }
    }
}

impl Default for Config {
    fn default() -> Self {
        Self::new()
    }
}
