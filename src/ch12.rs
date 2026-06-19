use std::env;
use std::error::Error;

pub fn minigrep() {
    let args: Vec<String> = env::args().collect();
    let cfg = Cfg::build(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {err}");
        std::process::exit(1);
    });
    println!("Searching for: '{}' in file: '{}'", cfg.query, cfg.fpath);
    if let Err(e) = run(cfg) {
        // We use 'if let' rather than 'unwrap_or_else'
        // because run returns () in the success case
        println!("Application error: {e}");
        std::process::exit(1);
    }
}

fn run(cfg: Cfg) -> Result<(), Box<dyn Error>> {
    let contents = std::fs::read_to_string(cfg.fpath)?;
    for line in search(&cfg.query, &contents) {
        println!("{line}");
    }
    Ok(())
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    println!("query: {query}\ncontents: {contents}");
    vec![]
}

struct Cfg {
    query: String,
    fpath: String,
}

impl Cfg {
    fn build(args: &[String]) -> Result<Self, &'static str> {
        if args.len() < 3 {
            return Err("Not enough arguments");
        }
        Ok(Cfg {
            query: args[1].clone(),
            fpath: args[2].clone(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn one_result() {
        let query = "duct";
        let contents = "Rust:\nsafe, fast, productive.\nPick three.";
        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }
}
