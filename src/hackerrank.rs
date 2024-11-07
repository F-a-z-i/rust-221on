use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};

fn simpleArraySum(ar: &[i32]) -> i32 {
    ar.iter().sum()
}

#[test]
fn test() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let mut fptr = File::create(env::var("OUTPUT_PATH").expect("OUTPUT_PATH not set")).expect("Unable to create file");

    let _ = stdin_iterator.next().expect("No input line").expect("Failed to read line");

    let ar: Vec<i32> = stdin_iterator.next().expect("No input line").expect("Failed to read line")
        .trim_end()
        .split(' ')
        .map(|s| s.parse::<i32>().expect("Failed to parse number"))
        .collect();

    let result = simpleArraySum(&ar);

    writeln!(&mut fptr, "{}", result).expect("Unable to write to file");
}
