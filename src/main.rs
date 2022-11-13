use std::{env, fs, io};

fn main() -> io::Result<()> {
    let config = Config::parse_args(env::args())
        .map_err(|err| io::Error::new(io::ErrorKind::NotFound, err))?;
    let search_space = fs::read_to_string(config.filename)?;
    if let true = search_space.contains(&config.pattern) {
        println!("Match found!");
        Ok(())
    } else {
        println!("No match found!");
        Err(io::Error::new(io::ErrorKind::NotFound, "No match found!"))
    }
}

#[derive(Debug)]
struct Config {
    pattern: String,
    filename: String,
}

impl Config {
    /// Parse a vec of args into a struct of config values
    fn parse_args<'a>(mut args: impl Iterator<Item = String>) -> Result<Self, &'a str> {
        args.next();
        let pattern: String = args.next().ok_or("Arg missing: pattern")?;
        let filename: String = args.next().ok_or("Arg missing: filename")?;

        Ok(Config { pattern, filename })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn struct_args_parses_vec_into_pattern_and_filename() {
        let expected = ["bin/name", "find me", "filename.txt"];
        let args = expected.iter().map(|&v| String::from(v));

        let parsed = Config::parse_args(args).unwrap();

        assert_eq!(parsed.pattern, String::from(expected[1]),);
        assert_eq!(parsed.filename, String::from(expected[2]),);
    }
}
