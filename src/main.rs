use std::io::Read;

use structopt::StructOpt;

use colored::*;
extern crate colored;
#[derive(StructOpt)]
struct Options {
    #[structopt(default_value = "...")]
    /// What does the crab say?
    message: String,
    #[structopt(short = "d", long = "dead")]
    /// Make the crab appear dead
    dead: bool,
    #[structopt(short = "i", long = "stdin")]
    /// Read the message from STDIN instead of the argument
    stdin: bool,
}

fn main() -> Result<std::process::ExitCode, std::io::Error> {
    let options = Options::from_args();

    let mut message = String::new();
    if options.stdin {
        std::io::stdin().read_to_string(&mut message)?; // [2]
    } else {
        message = options.message;
    };

    if message.to_lowercase() == "woof" {
        eprintln!("A crab shouldn't bark like a dog.")
    };
    if message.to_lowercase() == "meow" {
        eprintln!("A crab shouldn't sound like a cat.")
    };

    // clean message, assuming it does not contain multiple lines 
    let message = message.replace('\n', "");

    let eye = if options.dead {
        "x".red().bold()
    } else {
        "o".green()
    };

    println!("{}", message.bright_blue().on_blue());

    println!("{}", r"    \".truecolor(255, 165, 0));
    println!("{}", "     ,__,".truecolor(255, 165, 0));

    print!("{}", r"(/__/\".truecolor(255, 165, 0));
    print!("{eye}{eye}");
    println!("{}", r"/\__(/".truecolor(255, 165, 0));

    println!("{}", r"  _/\/__\/\_".truecolor(255, 165, 0));
    println!("{}", r"   _/    \_ ".truecolor(255, 165, 0));

    Ok(std::process::ExitCode::from(0))
}
