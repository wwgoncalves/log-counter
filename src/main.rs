use regex::Regex;
use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader, Error, ErrorKind};

fn main() -> Result<(), Error> {
    if env::args().count() != 3 {
        return Err(Error::new(
            ErrorKind::Other,
            "File path and pattern must be provided.",
        ));
    }

    let program_arguments: Vec<String> = env::args().collect::<Vec<String>>();
    let file_path = &program_arguments[1];
    let log_pattern = &program_arguments[2];

    let pattern = format!("(?P<log>{log_pattern})");
    let re = Regex::new(pattern.as_str()).unwrap();

    let file = File::open(file_path)?;
    let buffer = BufReader::new(file);

    let mut logs_map = HashMap::<String, u32>::new();

    for line in buffer.lines() {
        let line: String = line.unwrap();
        for pattern_captures in re.captures_iter(&line) {
            let log_piece = &pattern_captures["log"];

            let counter = logs_map.entry(log_piece.to_owned()).or_insert(0);
            *counter += 1;
        }
    }

    let mut logs_list: Vec<(&String, &u32)> = logs_map.iter().collect();
    logs_list.sort_by(|a, b| b.1.cmp(a.1));

    for log in logs_list {
        println!("{}\t\t{}", log.0, log.1);
    }

    Ok(())
}
