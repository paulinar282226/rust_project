pub struct Question { 
    pub text: String,
    pub options: [String; 4],
    pub correct_answer: char,
}

pub fn rust_questions() -> Vec<Question> {
    vec![
        Question {
            text: String::from("Co oznacza pojęcie 'ownership' w Rust?"),
            options: [String::from("Kontrola nad błędami w czasie kompilacji"), String::from("System zarządzania pamięcią"), String::from("Polimorfizm"), String::from("Inkapsulacja")],
            correct_answer: 'B',
        },
        Question {
            text: String::from("Jakie słowo kluczowe w Rust tworzy zmienną niemutowalną?"),
            options: [String::from("var"), String::from("const"), String::from("let"), String::from("static")],
            correct_answer: 'C',
        },
        Question {
            text: String::from("Jaka biblioteka jest domyślnie używana do testowania w Rust?"),
            options: [String::from("RSpec"), String::from("Cargo"), String::from("Rustc"), String::from("Test")],
            correct_answer: 'D',
        },
        Question {
            text: String::from("Który typ danych w Rust jest używany do reprezentowania pojedynczego znaku?"),
            options: [String::from("str"), String::from("char"), String::from("string"), String::from("word")],
            correct_answer: 'B',
        },
        Question {
            text: String::from("Jakie słowo kluczowe w Rust umożliwia wykorzystanie kodu z innych modułów?"),
            options: [String::from("package"), String::from("module"), String::from("import"), String::from("use")],
            correct_answer: 'D',
        }
    ]
}
pub fn python_questions() -> Vec<Question> {
    vec![
        Question {
            text: String::from("Jaki typ danych w Pythonie jest niemutowalny?"),
            options: [String::from("List"), String::from("Set"), String::from("Dictionary"), String::from("Tuple")],
            correct_answer: 'D',
        },
        Question {
            text: String::from("Jak nazywa się narzędzie do zarządzania środowiskami i pakietami w Pythonie?"),
            options: [String::from("npm"), String::from("pip"), String::from("virtualenv"), String::from("conda")],
            correct_answer: 'D',
        },
        Question {
            text: String::from("Co robi słowo kluczowe 'yield' w Pythonie?"),
            options: [String::from("Zatrzymuje pętlę"), String::from("Tworzy iterator"), String::from("Wyrzuca błąd"), String::from("Przypisuje wartość")],
            correct_answer: 'B',
        },
        Question {
            text: String::from("Jakie są dwie główne wersje Pythona?"),
            options: [String::from("1.x i 2.x"), String::from("2.x i 3.x"), String::from("3.x i 4.x"), String::from("3.x i 5.x")],
            correct_answer: 'B',
        },
        Question {
            text: String::from("Która z poniższych nie jest wbudowaną funkcją w Pythonie?"),
            options: [String::from("print()"), String::from("input()"), String::from("help()"), String::from("import()")],
            correct_answer: 'D',
        }
    ]
}
pub fn c_questions() -> Vec<Question> {
    vec![
        Question {
            text: String::from("Który operator w C służy do dereferencji wskaźników?"),
            options: [String::from("&"), String::from("*"), String::from("#"), String::from("%")],
            correct_answer: 'B',
        },
        Question {
            text: String::from("Jak zadeklarować wskaźnik w języku C?"),
            options: [String::from("int p;"), String::from("int &p;"), String::from("int *p;"), String::from("int #p;")],
            correct_answer: 'C',
        },
        Question {
            text: String::from("Która funkcja w języku C jest używana do dynamicznego przydzielania pamięci?"),
            options: [String::from("malloc()"), String::from("alloc()"), String::from("new()"), String::from("mem()")],
            correct_answer: 'A',
        },
        Question {
            text: String::from("Jaki jest domyślny typ zwracany przez funkcję w C, jeśli nie zadeklarowano inaczej?"),
            options: [String::from("int"), String::from("void"), String::from("double"), String::from("char")],
            correct_answer: 'A',
        },
        Question {
            text: String::from("Co robi słowo kluczowe 'volatile' w C?"),
            options: [String::from("Zapobiega optymalizacji zmiennej przez kompilator"), String::from("Deklaruje zmienną jako stałą"), String::from("Tworzy zmienną tymczasową"), String::from("Wskazuje, że zmienna jest prywatna")],
            correct_answer: 'A',
        }
    ]
}
