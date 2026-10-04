use std::env;
use std::io::{self, BufRead};

fn initial(part: &str) -> String {
    part.split('-')
        .filter_map(|p| p.chars().next())
        .map(|c| format!("{}.", c.to_uppercase()))
        .collect::<Vec<_>>()
        .join("-")
}

fn shorten(full_name: &str) -> String {
    let mut parts = full_name.split_whitespace();
    let Some(surname) = parts.next() else {
        return String::new();
    };

    let mut result = surname.to_string();
    for part in parts {
        result.push(' ');
        result.push_str(&initial(part));
    }
    result
}

fn shortname() {
    let args: Vec<String> = env::args().skip(1).collect();

    if !args.is_empty() {
        println!("{}", shorten(&args.join(" ")));
    } else {
        for line in io::stdin().lock().lines().map_while(Result::ok) {
            println!("{}", shorten(&line));
        }
    }
}
