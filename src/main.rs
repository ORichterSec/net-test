use std::env;
use service::service;
use dirs;

fn main() -> std::io::Result<()> {
    let homedir = dirs::home_dir().unwrap();
    let file = std::fs::read_dir(homedir)?;
    for f in file {
        println!("{:?}",f?.path());
    }

    let args: Vec<String> = env::args().collect();
    dbg!(&args);
    let config = service::Config::new(&args);
    dbg!(&config);
    let result = service::run(&config);

    match result {
        Ok(output) => {
            println!("Operation finished successfully with following message:\n{}", output);
            Ok(())
        },
        Err(msg) => {
            eprint!("Operation failed with following message:\n{}",msg);
            Err(std::io::ErrorKind::Other.into())
        }
    }

}
