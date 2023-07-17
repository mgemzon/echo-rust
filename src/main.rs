use clap::{App, Arg};

fn main() {
    let matches = App::new("echo-rust")
        .version("0.1.0")
        .author("Popher Gemzon <gemzon.markchristopher@gmail.com>")
        .about("Rust implemenation of echo command")
        .arg(
            Arg::with_name("text")
                .value_name("TEXT")
                .help("Input text")
                .required(true)
                .min_values(1),
        )
        .arg(
            Arg::with_name("no_newline")
                .short("n")
                .long("no-newline")
                .help("Do not print newline")
                .takes_value(false),
        )
        .get_matches();

    let text: Vec<String>;
    match matches.values_of_lossy("text") {
        Some(value) => {
            text = value;
        }
        None => {
            println!("{}", matches.usage());
            std::process::exit(1);
        }
    }

    let no_newline = matches.is_present("no_newline");

    print!("{}{}", text.join(" "), if no_newline { "" } else { "\n" });
}
