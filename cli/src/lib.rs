use structopt::StructOpt;
use std::thread;
use reedline::{DefaultPrompt, Reedline, Signal};
use colored::Colorize;
use backend;

#[derive(StructOpt)]
struct Cli {
    #[structopt(parse(from_os_str))]
    path: std::path::PathBuf,
}

pub fn read_configfile() {
    let _args = Cli::from_args();
    let result = std::fs::read_to_string(&_args.path).expect("could not read file");
    for line in result.lines() {
            println!("{}", format!("{}",line.bold()));
    }
}
 

pub fn init_cli() {
 
    read_configfile();
    let mut line_editor = Reedline::create();
    let prompt = DefaultPrompt::default();

    thread::spawn(move|| {
        loop {
            let sig = line_editor.read_line(&prompt);
            match sig {
                Ok(Signal::Success(buffer)) => {
                    let s = format!("{:?}", &buffer);
                    backend::read_input_configfile(&s)
                }
                Ok(Signal::CtrlD) | Ok(Signal::CtrlC) => {
                    println!("\nAborted!");
                    break;
                }
                x => {
                    println!("Event: {:?}", x);
                }
            }
        }
    });
}