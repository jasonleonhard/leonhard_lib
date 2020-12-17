pub mod file_io {
    pub fn some_function() {
        println!("# some_function called");
    }

    pub fn some_function2() {
        println!("# some_function2 called");
    }

    use std::{fs::File, io, io::BufRead, path::Path};
    pub fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
    where
        P: AsRef<Path>,
    {
        let file = File::open(filename)?;
        Ok(io::BufReader::new(file).lines())
    }
    pub fn read_lines_example() {
        println!(
            "{}",
            "
use leonhard_lib::file_io;

let mut rows: Vec<String> = Vec::new();
if let Ok(lines) = file_io::read_lines('./data/day3.txt') {
    for line in lines {
        if let Ok(raw_line) = line {
            rows.push(raw_line.clone());
        }
    }
}
            "
        );
    }
}
