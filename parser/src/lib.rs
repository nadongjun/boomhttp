use structopt::StructOpt;
use std::thread;
use reedline::{DefaultPrompt, Reedline, Signal};
use std::str;
use colored::Colorize;

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

pub fn read_input_configfile(path : &str) {
    println!("{} : {}", format!("INPUT Configuration file path : ").bold().green(), path);
    let path = path.replace('"', "");
    //let mut file = std::fs::File::create(path).expect("create failed");

    let result = std::fs::read_to_string(&path);
    match result {
        Ok(content) => { println!("{}", format!("file content: \n{}", content).bold());},
        Err(error) => { println!("{} {} : {}", format!("Not found : ").bold().red() , format!("{}", error).bold().red(), path);}
    }
     
     
}

pub fn init_parser() {
 
    read_configfile();
    let mut line_editor = Reedline::create();
    let prompt = DefaultPrompt::default();

    thread::spawn(move|| {
        loop {
            let sig = line_editor.read_line(&prompt);
            match sig {
                Ok(Signal::Success(buffer)) => {
                    let s = format!("{:?}", &buffer);
                    read_input_configfile(&s)
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

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
       // assert_eq!(1,1);
    }
}