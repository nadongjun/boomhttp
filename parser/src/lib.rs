use structopt::StructOpt;
#[derive(StructOpt)]
struct Cli {
    #[structopt(parse(from_os_str))]
    path: std::path::PathBuf,
}

pub fn read_configfile() {
    let _args = Cli::from_args();
    let result = std::fs::read_to_string(&_args.path).expect("could not read file");
    for line in result.lines() {
            println!("{}", line);
    }
    println!("END READ");
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
       // assert_eq!(1,1);
    }
}