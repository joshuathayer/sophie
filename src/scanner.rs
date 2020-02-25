#[derive(Debug)]
#[derive(PartialEq)]
#[repr(u8)]
pub enum TokenType {
    // Single-character tokens.
    LEFTPAREN, RIGHTPAREN,
    LEFTBRACE, RIGHTBRACE,
    COMMA, DOT, MINUS, PLUS,
    SEMICOLON, SLASH, STAR,

    // One or two character tokens.
    BANG, BANGEQUAL,
    EQUAL, EQUALEQUAL,
    GREATER, GREATEREQUAL,
    LESS, LESSEQUAL,

    // Literals.
    IDENTIFIER, STRING, NUMBER,

    // Keywords.
    AND, CLASS, ELSE, FALSE,
    FOR, FUN, IF, NIL, OR,
    PRINT, RETURN, SUPER, THIS,
    TRUE, VAR, WHILE,

    ERROR,
    EOF
}

pub struct Scanner {
    pub line: u16,
    pub start: usize,
    pub current: usize
}

pub struct Token {
    pub typ: TokenType,
    pub line: u16,
    pub start: usize,
    pub length: usize,
    pub error: Option<String>,
}

 pub fn init_scanner() -> Scanner {
    Scanner {
        line: 1,
        start: 0,
        current: 0
    }
}

pub fn scan_token(scanner: &mut Scanner, source: &str) -> Token {
    scanner.start = scanner.current;

    skip_whitespace(scanner, source);

    if is_at_end(scanner, source) {
        return make_token(TokenType::EOF, &scanner);
    }

    let c = advance(scanner, source);

    match c {
        '(' => make_token(TokenType::LEFTPAREN, scanner),
        ')' => make_token(TokenType::RIGHTPAREN, scanner),
        '{' => make_token(TokenType::LEFTBRACE, scanner),
        '}' => make_token(TokenType::RIGHTBRACE, scanner),
        ';' => make_token(TokenType::SEMICOLON, scanner),
        ',' => make_token(TokenType::COMMA, scanner),
        '.' => make_token(TokenType::DOT, scanner),
        '-' => make_token(TokenType::MINUS, scanner),
        '+' => make_token(TokenType::PLUS, scanner),
        '*' => make_token(TokenType::STAR, scanner),
        '/' => make_token(TokenType::SLASH, scanner),
        '!' => {
            if char_match('=', scanner, source) {
                make_token(TokenType::BANGEQUAL, scanner)
            } else {
                make_token(TokenType::BANG, scanner)
            }
        },
        '=' => {
            if char_match('=', scanner, source) {
                make_token(TokenType::EQUALEQUAL, scanner)
            } else {
                make_token(TokenType::EQUAL, scanner)
            }
        },
        '>' => {
            if char_match('=', scanner, source) {
                make_token(TokenType::GREATEREQUAL, scanner)
            } else {
                make_token(TokenType::GREATER, scanner)
            }
        },
        '<' => {
            if char_match('=', scanner, source) {
                make_token(TokenType::LESSEQUAL, scanner)
            } else {
                make_token(TokenType::LESS, scanner)
            }
        },
        '"' => string(scanner, source),
        '0'..='9' => number(scanner, source),
        _   => error_token("Unexpected character.".to_string(), scanner)
    }

}

fn number(scanner: &mut Scanner, source: &str) -> Token {
    loop {
        if is_at_end(scanner, source) {
            break;
        }
        let c = peek(scanner, source);

        match c {
            '0'..='9' => (), // continue if it's a digit...
            '.' => match peek_next(scanner, source) {
                '0'..='9' => {
                    advance(scanner, source); // continue if it's a digit after a '.'
                },
                _ => break                             // otherwise we're done
            }
            _ => break // break if it's anything else
        }

        advance(scanner, source);
    }

    make_token(TokenType::NUMBER, scanner)
}

fn string<'a>(scanner: &mut Scanner, source: &'a str) -> Token {
    loop {
        if is_at_end(scanner, source) {
            break;
        }

        let c = peek(scanner, source);

        match c {
            '"' => { advance(scanner, source); break; },
            '\n' => scanner.line += 1,
            _ => ()
        }

        advance(scanner, source);
    }

    make_token(TokenType::STRING, scanner)
}

fn skip_whitespace<'a>(scanner: &mut Scanner, source: &'a str) {
    loop {
        if is_at_end(scanner, source) {
            break;
        }

        let c = peek(scanner, source);

        match c {
            ' ' | '\r' | '\t' => {
                advance(scanner, source);
                scanner.start = scanner.current;
            },
            '\n' => {
                scanner.line += 1;
                advance(scanner, source);
                scanner.start = scanner.current;
            },
            _ => break
        }
    };
}

fn peek(scanner: &Scanner, source: &str) -> char {
    source[scanner.current..].chars().next().unwrap()
}

fn peek_next(scanner: &Scanner, source: &str) -> char {
    source[scanner.current+1..].chars().next().unwrap()
}


fn char_match<'a>(expected: char, scanner: &mut Scanner, source: &'a str) -> bool {
    if is_at_end(scanner, source) {
        return false;
    }

    if &source[scanner.current..scanner.current+1] != expected.to_string() {
        return false;
    }
    scanner.current += 1;
    true
}

fn advance<'a>(scanner: &mut Scanner, source: &'a str) -> char {
    scanner.current += 1;
    source[scanner.current - 1..].chars().next().unwrap()
}

fn is_at_end(scanner: &Scanner, source: &str) -> bool {
    scanner.current == source.len()
}

fn make_token(typ: TokenType, scanner: &Scanner) -> Token {
    Token {
        typ: typ,
        line: scanner.line,
        start: scanner.start,
        length: scanner.current - scanner.start,
        error: None,
    }
}

fn error_token(message: String, scanner: &Scanner) -> Token {
    Token {
        typ: TokenType::ERROR,
        line: scanner.line,
        start: scanner.start,
        length: scanner.current - scanner.start,
        error: Some(message)
    }
}
