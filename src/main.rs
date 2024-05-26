use std::io::{self, BufRead, BufReader, Write};
use std::fs::{File, OpenOptions};
use crate::questions::{rust_questions, python_questions, c_questions, Question};

mod questions;

fn main() {
    game();
}

fn game() {
    println!("Wybierz tryb gry: 1) Jeden gracz 2) Pojedynek 1v1");
    let mode = get_input();

    match mode.trim() {
        "1" => single_mode(),
        "2" => coop(),
        _ => {
            println!("Niepoprawny wybór trybu gry.");
            game();
        }
    }
}

fn single_mode() {
    println!("Podaj swoje imię:");
    let username = get_input();
    let questions = choose_quiz_category();
    let score = run_quiz(&questions);
    println!("{} - Twój wynik to: {}/{}", username,score, questions.len());

    if let Err(err) = save_score(&username, score, questions.len()) {
        println!("Błąd podczas zapisywania wyniku: {}", err);
    }

    if let Err(err) = display_top_scores() {
        println!("Błąd podczas wyświetlania najlepszych wyników: {}", err);
    }

    println!("Pytania i poprawne odpowiedzi:");
    for q in &questions {
        println!("Pytanie: {}", q.text);
        println!("Poprawna odpowiedź: {}",q.correct_answer);
        println!();
    }

    println!("Chcesz zagrać jeszcze raz? Y/N");
    let again = get_input();
    match again.trim().to_uppercase().as_str() {
        "Y" => game(),
        _ => println!("Dzięki za grę!"),
    }
}

fn coop() {
    println!("Gracz 1, podaj swoje imię:");
    let username1 = get_input();
    println!("Gracz 2, podaj swoje imię:");
    let username2 = get_input();
    let questions = choose_quiz_category();
    let scores = run_coop_quiz(&questions,&username1, &username2);

    if scores.0 > scores.1 {
        println!("{} wygrał!", username1)
    } else if scores.1 > scores.0 {
        println!("{} wygrał!",username2)
    } else {
        println!("Remis!")
    }

    println!("Wynik: {} - {}/{} punktów\n{} - {}/{} punktów",
             username1, scores.0, questions.len(),
             username2, scores.1, questions.len());

    println!("Poprawne odpowiedzi:");
    for q in &questions {
        println!("Pytanie: {}", q.text);
        println!("Poprawna odpowiedź: {}", q.correct_answer);
        println!();
    }

    println!("Chcesz zagrać jeszcze raz? Y/N");
    let again = get_input();
    match again.trim().to_uppercase().as_str() {
        "Y" => game(),
        _ => println!("Dzięki za grę!"),
    }
}

fn get_input() -> String {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Nie można odczytać");
    input.trim().to_string()
}

fn choose_quiz_category() -> Vec<Question> {
    println!("Wybierz kategorię quizu: 1) Rust 2) Python 3) C");
    let category = get_input();

    match category.trim() {
        "1" => rust_questions(),
        "2" => python_questions(),
        "3" => c_questions(),
        _ => {
            println!("Wpisz liczbę odpowiadającą danej kategorii.");
            choose_quiz_category()
        }
    }
}

fn run_quiz(questions: &[Question]) -> usize {
    let mut score = 0;

    for q in questions {
        println!("{}", q.text);
        println!("A) {}", q.options[0]);
        println!("B) {}", q.options[1]);
        println!("C) {}", q.options[2]);
        println!("D) {}", q.options[3]);

        let mut user_input: String;
        loop {
            user_input = get_input().to_uppercase();
            if let Some(answer) =user_input.chars().next() {
                if answer == 'A'|| answer == 'B' || answer == 'C' || answer == 'D' {
                    break;
                } else {
                    println!("Niepoprawna odpowiedź. Spróbuj ponownie.");
                }
            } else {
                println!("Niepoprawna odpowiedź. Spróbuj ponownie.");
            }
        }

        if user_input.chars().next() == Some(q.correct_answer) {
            println!("Super!");
            score += 1;
        } else {
            println!("Niestety, to zła odpowiedź!");
        }
    }

    score
}

fn run_coop_quiz(questions: &[Question], username1: &str, username2: &str) -> (usize, usize) {
    let mut scores = (0, 0);

    for  q in questions.iter(){
        println!("{}", q.text);
        println!("A) {}", q.options[0]);
        println!("B) {}", q.options[1]);
        println!("C) {}", q.options[2]);
        println!("D) {}", q.options[3]);

        println!("Tura {}", username1);
        let mut user_input1: String;
        loop {
            user_input1 = get_input().to_uppercase();
            if let Some(answer) =user_input1.chars().next() {
                if answer == 'A' || answer == 'B' || answer == 'C' || answer == 'D' {
                    break;
                } else {
                    println!("Niepoprawna odpowiedź. Spróbuj ponownie.");
                }
            } else {
                println!("Niepoprawna odpowiedź. Spróbuj ponownie.");
            }
        }
        if user_input1.chars().next() == Some(q.correct_answer) {
            scores.0 += 1;
        }

        // Gracz 2 odpowiada
        println!("Tura {}", username2);
        let mut user_input2: String;
        loop {
            user_input2 = get_input().to_uppercase();
            if let Some(answer) =user_input2.chars().next() {
                if answer == 'A' || answer == 'B' || answer == 'C' || answer == 'D' {
                    break;
                } else {
                    println!("Niepoprawna odpowiedź. Spróbuj ponownie.");
                }
            } else {
                println!("Niepoprawna odpowiedź. Spróbuj ponownie.");
            }
        }
        if user_input2.chars().next() == Some(q.correct_answer) {
            scores.1 += 1;
        }
    }

    scores
}

fn save_score(username: &str, score: usize,total: usize) -> io::Result<()> {
    let mut file = OpenOptions::new()
        .write(true)
        .append(true)
        .create(true)
        .open("scores.txt")?;
    writeln!(file, "{} {} {}", username, score, total)?;
    Ok(())
}

fn display_top_scores() -> io::Result<()> {
    let file = File::open("scores.txt")?;
    let reader = BufReader::new(file);

    let mut scores: Vec<(String, usize, usize)> = reader.lines()
        .filter_map(|line| line.ok())
        .filter_map(|line| {
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() == 3 {
                let username = parts[0].to_string();
                let score = parts[1].parse::<usize>().unwrap_or(0);
                let total = parts[2].parse::<usize>().unwrap_or(0);
                Some((username, score, total))
            } else {
                None
            }
        })
        .collect();

    scores.sort_by(|a, b| b.1.cmp(&a.1));

    println!("Top 3 wyniki:");
    for (index, (username, score, total)) in scores.iter().take(3).enumerate() {
        println!("{}. {} - {}/{}", index + 1, username, score, total);
    }

    Ok(())
}
