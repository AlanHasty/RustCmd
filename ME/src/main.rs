use clap::{Arg, ArgAction, Command};


fn main() {
    let matches = Command::new("ME")
        .version("0.1.0")
        .author("Alan Hasty <alan.hasty@gmail.com")
        .about("Alan's version of `echo`")
        .arg(
            Arg::new("text")
                .value_name("TEXT")
                .help("Input Text")
                .required(true)
                .num_args(1..),
        )
        .arg(
            Arg::new("omit_newline")
                .short('n')
                .action(ArgAction::SetTrue)
                .help("Do not print newline"),
        )
        .get_matches();
    let omit_newline = matches.get_flag("omit_newline");
    // println!("{:#?}", matches);

    let text: Vec<String> = matches.get_many("text").unwrap().cloned().collect();

    println!("{}{}", text.join(" "),
                if omit_newline { "" } else { "\n" } );
}
