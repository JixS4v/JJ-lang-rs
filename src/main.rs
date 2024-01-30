use std::env;
use std::fs;
use std::io;

#[derive(Clone,Debug,Copy)]
enum TokenType {
    // Single character
    LeftParen, RightParen, LeftBrace, RightBrace, Comma, Dot, Minus, Plus, Semicolon, Slash, Star,

    // One or two character
    Bang, BangEqual, Equal, EqualEqual, Greater, GreaterEqual, Less, LessEqual,

    // Literals
    Identifier, String, Number,

    // Keywords
    And, Class, Else, False, Fun, For, If, Nil, Or, Print, Return, Super, This, True, Let, While,

    EOF
}

#[derive(Clone)]
struct Token {
    token_type: TokenType,
    lexeme: String,
    line: usize
}
impl Token {
    fn new(token_type: TokenType, lexeme: String, line: usize) -> Token {
        Token {
            token_type: token_type,
            lexeme: lexeme,
            line: line
        }
    }
    fn to_string(&self) -> String {
        let token = self.token_type;
        let lexeme = self.lexeme.clone();
        format!("{:?} {lexeme}", token).to_string()
    }
}

struct Scanner {
    source: String,
    tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: usize
}

impl Scanner {
    fn new(source: String) -> Scanner {
        Scanner {
            source: source,
            tokens: Vec::new(),
            start: 0,
            current: 0,
            line: 0
        }
    }

    fn check_next(&mut self, c: &str) -> bool {
        if self.current >= self.source.len() {
            return false
        }
        if &self.source[self.current..self.current+1]!= c {
            return false
        }
        self.current+=1;
        return true
    }

    fn peek(&mut self, offset:usize) -> &str {
        if self.current >= self.source.len() {
            return "\0"
        }
        return &self.source[self.current+offset..self.current+offset+1]
    }

    fn string(&mut self) {
        while self.peek(0) != "\"" && self.current<self.source.len() {
            if self.peek(0) == "\n" {self.line+=1;}
            self.current+=1;
        }

        if self.current>=self.source.len() {
            error(self.line, "Unterminated string");
        }

        self.current+=1;

        self.add_token(TokenType::String);
    }

    fn number(&mut self){
        while is_digit(self.peek(0)) {self.current+=1}
        if self.peek(0) == "." && is_digit(self.peek(1)) {
            self.current+=1;

            while is_digit(self.peek(0)) {self.current+=1}
        }
        self.add_token(TokenType::Number)
    }

    fn identifier(&mut self) {
        while is_alphanumeric(self.peek(0)) {self.current+=1;}
        match self.match_symbol() {
            Some(token_type) => self.add_token(token_type),
            None => self.add_token(TokenType::Identifier)
        }

    }

    fn match_symbol(&mut self) -> Option<TokenType> {
        let current_text = self.get_current_string();
        match current_text {
            "and" => Some(TokenType::And),
            "class" => Some(TokenType::Class),
            "else" => Some(TokenType::Else),
            "false" => Some(TokenType::False),
            "for" => Some(TokenType::For),
            "fun" => Some(TokenType::Fun),
            "if" => Some(TokenType::If),
            "nil" => Some(TokenType::Nil),
            "or" => Some(TokenType::Or),
            "print" => Some(TokenType::Print),
            "return" => Some(TokenType::Return),
            "super" => Some(TokenType::Super),
            "this" => Some(TokenType::This),
            "true" => Some(TokenType::True),
            "let" => Some(TokenType::Let),
            "while" => Some(TokenType::While),
            _ => None
        }
    }

    fn scan_token(&mut self) {
        let character = &self.source[self.current..self.current+1];
        self.current += 1;
        match character {
            "(" => self.add_token(TokenType::LeftParen),
            ")" => self.add_token(TokenType::RightParen),
            "{" => self.add_token(TokenType::LeftBrace),
            "}" => self.add_token(TokenType::RightBrace),
            "," => self.add_token(TokenType::Comma),
            "." => self.add_token(TokenType::Dot),
            "-" => self.add_token(TokenType::Minus),
            "+" => self.add_token(TokenType::Plus),
            ";" => self.add_token(TokenType::Semicolon),
            "*" => self.add_token(TokenType::Star),
            "!" =>  if self.check_next("="){self.add_token(TokenType::BangEqual)} else {self.add_token(TokenType::Bang)},
            "=" => if self.check_next("="){self.add_token(TokenType::EqualEqual)} else {self.add_token(TokenType::Equal)},
            "<" => if self.check_next("="){self.add_token(TokenType::LessEqual)} else {self.add_token(TokenType::Less)},
            ">" => if self.check_next("="){self.add_token(TokenType::GreaterEqual)} else {self.add_token(TokenType::Greater)},
            "/" => if self.check_next("/") {while self.peek(0) != "\n" && !self.current >= self.source.len() {self.current+=1;}} else {self.add_token(TokenType::Slash)},
            "\n" => self.line+=1,
            " " | "\r" | "\t" => (), // Ignore whitespace
            "\"" => self.string(),
            _ =>  if is_digit(character) {self.number();} else if is_alpha(character) {self.identifier();} else{error(self.line, "Unexpected character")}
        }
    }

    fn scan_tokens(&mut self) -> Vec<Token> {
        let source_length = self.source.len();
        while self.current < source_length {
            self.start = self.current;
            self.scan_token();
        }
        self.tokens.push(Token::new(TokenType::EOF,"".to_string(),self.line));
        self.tokens.clone()
    }
    fn get_current_string(&mut self) -> &str {
        let text = &self.source[self.start..self.current];
        text
    }
    fn add_token(&mut self, token_type: TokenType) {
        let text = self.get_current_string().to_string();
        self.tokens.push(Token::new(token_type, text,self.line));
    }
}

fn main() {
    //let arguments: Vec<String> = env::args().collect();

    //let argument_number = arguments.len();

    //if argument_number > 1 {
    //    println!("Usage: rsjj [filename]");
    //    return
    //} else if argument_number == 1 {
      //  run_file(&arguments[0]);
    //} else {
        run_prompt();
    //}
}

fn run_file(path: &str) {
    let file = fs::read_to_string(path).expect("Failed to open {path}");
    run(file);
}

fn run_prompt() {
    
    loop {
        let mut current_line = String::new();
        print!("> ");
        io::stdin()
            .read_line(&mut current_line)
            .expect("Failed to read line");
        if current_line.len() == 0 {
            continue;
        }
        run(current_line);
    }
}

fn run(source: String){
    let mut scanner: Scanner = Scanner::new(source);
    let tokens: Vec<Token> = scanner.scan_tokens();

    for token in tokens {
        let print = token.to_string();
        println!("{print}");
    }
}

fn error(line: usize, message: &str) {
    report(line, "", message);
}

fn report(line: usize, location: &str, message: &str) {
    eprintln!("[line {line}]: Error {location}: {message}");
}

fn is_digit(character: &str) -> bool {
    if character.to_string().chars().nth(0).unwrap().is_numeric() {
        return true
    }
    return false
}

fn is_alpha(character: &str) -> bool {
    if character.to_string().chars().nth(0).unwrap().is_alphabetic() {
        return true
    }
    return false
}

fn is_alphanumeric(character: &str) -> bool {
    if character.to_string().chars().nth(0).unwrap().is_alphanumeric() {
        return true
    }
    return false
}