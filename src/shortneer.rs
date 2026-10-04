use std::env;
use std::io::{self, BufRead};

/// Берёт первую букву каждой части (учитывает двойные имена через дефис):
/// "Анна-Мария" -> "А.-М."
fn initial(part: &str) -> String {
    part.split('-')
        .filter_map(|p| p.chars().next())
        .map(|c| format!("{}.", c.to_uppercase()))
        .collect::<Vec<_>>()
        .join("-")
}

/// "Иванов Иван Иванович" -> "Иванов И. И."
/// Первое слово (фамилия) остаётся полностью, остальные сокращаются до инициалов.
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
        // Имя передано аргументами: cargo run -- Иванов Иван Иванович
        println!("{}", shorten(&args.join(" ")));
    } else {
        // Иначе читаем строки из stdin
        for line in io::stdin().lock().lines().map_while(Result::ok) {
            println!("{}", shorten(&line));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn full_name() {
        assert_eq!(shorten("Иванов Иван Иванович"), "Иванов И. И.");
    }

    #[test]
    fn no_patronymic() {
        assert_eq!(shorten("Петров Пётр"), "Петров П.");
    }

    #[test]
    fn lowercase_and_extra_spaces() {
        assert_eq!(shorten("  сидоров   алексей  "), "сидоров А.");
    }

    #[test]
    fn double_name() {
        assert_eq!(shorten("Смирнова Анна-Мария Олеговна"), "Смирнова А.-М. О.");
    }

    #[test]
    fn single_word_and_empty() {
        assert_eq!(shorten("Иванов"), "Иванов");
        assert_eq!(shorten(""), "");
    }
}
