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
    let res = if cfg.ncase {
        search_case_insensitive(&cfg.query, &contents)
    } else {
        search(&cfg.query, &contents)
    };
    for line in res {
        println!("{line}");
    }
    Ok(())
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut res = Vec::new();
    for line in contents.lines() {
        if line.contains(query) {
            res.push(line);
        }
    }
    res
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    let mut res = Vec::new();
    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            res.push(line);
        }
    }
    res
}

struct Cfg {
    query: String,
    fpath: String,
    ncase: bool,
}

impl Cfg {
    fn build(args: &[String]) -> Result<Self, &'static str> {
        if args.len() < 3 {
            return Err("Not enough arguments");
        }
        Ok(Cfg {
            query: args[1].clone(),
            fpath: args[2].clone(),
            ncase: env::var("IGNORE_CASE").is_ok(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "Rust:\nsafe, fast, productive.\nPick three.\nDuck tape.";
        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }
    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";
        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(query, contents)
        );
    }
}
