use std::{env,process};
use commandline::{Config,run};

fn main() {
    //getting the arguments
    let args: Vec<String> =env::args().collect();

    let config=Config::build(&args).unwrap_or_else(|err|{
        println!("problem parsing arguments:{err}");
        process::exit(1);
    });
    println!("Searching for {}", config.query);
    println!("In file {}", config.file_path);

    if let Err(e)=run(config){
        println!("Application error: {e}");
        process::exit(1);
    }
}



