use std::mem;

pub struct Parser {
    pub current: Option<crate::scanner::Token>,
    pub previous: Option<crate::scanner::Token>,
    pub had_error: bool,
    pub panic_mode: bool,
}

fn init_parser() -> Parser {
    Parser {
        current: None,
        previous: None,
        had_error: false,
        panic_mode: false,
    }
}

pub fn compile(source: &str, chunk: &mut crate::chunk::Chunk) -> bool {
    let mut scanner = crate::scanner::init_scanner();
    let mut parser = init_parser();

    advance(&mut parser, &mut scanner, source);
    consume(&mut parser, &mut scanner, source, crate::scanner::TokenType::EOF, "Expect end of expression".to_string());

    !parser.had_error
}

pub fn consume(parser: &mut Parser,
               scanner: &mut crate::scanner::Scanner,
               source: &str,
               token_type: crate::scanner::TokenType,
               message: String) {

    if parser.current.as_ref().unwrap().typ == token_type {
        advance(parser, scanner, source);
        return;
    }

    error_at_current(parser, message, source)
}

pub fn advance(mut parser: &mut Parser,
               scanner: &mut crate::scanner::Scanner,
               source: &str) {
    loop {
        // "move" current to previous.
        mem::swap(&mut parser.previous, &mut parser.current);
        parser.current = Some(crate::scanner::scan_token(scanner, source));

        match &parser.current.as_ref().unwrap().error {
            Some(err) => {
                let message = err.to_string();
                error_at_current(&mut parser, message, source)
            },
            None => break
        }
    }
}

fn error(parser: &mut Parser, message: String, source: &str) {
    if parser.panic_mode {
        return;
    }
    parser.had_error = true;
    parser.panic_mode = true;
    error_at(&parser.previous.as_ref().unwrap(), message, source)
}

fn error_at_current(parser: &mut Parser, message: String, source: &str) {
    if parser.panic_mode {
        return;
    }
    parser.had_error = true;
    parser.panic_mode = true;
    error_at(&parser.current.as_ref().unwrap(), message, source)
}

fn error_at(token: &crate::scanner::Token, message: String, source: &str) {

    print!("[line {}] Error", token.line);
    if token.typ == crate::scanner::TokenType::EOF {
        print!(" at end");
    } else if token.typ == crate::scanner::TokenType::ERROR {
        print!("");
    } else {
        print!(" at '{}'",
               &source[token.start..token.start+token.length]);
    }
    println!(": {}", message);
}
