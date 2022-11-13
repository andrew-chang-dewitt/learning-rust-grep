use std::{env, fs, io};

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let config =
        Config::parse_args(&args).map_err(|err| io::Error::new(io::ErrorKind::NotFound, err))?;
    let search_space = fs::read_to_string(config.filename)?;
    if let true = search_space.contains(config.pattern) {
        println!("Match found!");
        Ok(())
    } else {
        println!("No match found!");
        Err(io::Error::new(io::ErrorKind::NotFound, "No match found!"))
    }
}

#[derive(Debug)]
struct Config<'a> {
    pattern: &'a String,
    filename: &'a String,
}

impl<'a> Config<'a> {
    /// Parse a vec of args into a struct of config values
    fn parse_args(args: &'a [String]) -> Result<Self, String> {
        let pattern = args.get(1).ok_or("Pattern arg missing!")?;
        let filename = args.get(2).ok_or("Filename arg missing!")?;

        Ok(Config { pattern, filename })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn struct_args_parses_vec_into_pattern_and_filename() {
        let expected = ["bin/name", "find me", "filename.txt"];
        let args: Vec<String> = expected.iter().map(|&v| String::from(v)).collect();

        let parsed = Config::parse_args(&args).unwrap();

        assert_eq!(parsed.pattern, &String::from(expected[1]),);
        assert_eq!(parsed.filename, &String::from(expected[2]),);
    }
}
