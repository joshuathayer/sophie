extern crate num_derive;
use num::{ToPrimitive};

use std::mem;
use std::str::FromStr;
use std::convert::TryFrom;

pub struct Parser {
    pub current: Option<crate::scanner::Token>,
    pub previous: Option<crate::scanner::Token>,
    pub had_error: bool,
    pub panic_mode: bool,
}

macro_rules! opcode {
    ($op:tt) => {
        crate::chunk::Opcode::to_u8(&crate::chunk::Opcode::$op).unwrap()
    };
}

fn init_parser() -> Parser {
    Parser {
        current: None,
        previous: None,
        had_error: false,
        panic_mode: false,
    }
}

pub fn compile(source: &str,
               mut chunk: &mut crate::chunk::Chunk) -> bool {

    let mut scanner = crate::scanner::init_scanner();
    let mut parser = init_parser();

    advance(&mut parser, &mut scanner, source);
    expression(&mut parser, &mut scanner, &mut chunk, source);
    consume(&mut parser, &mut scanner, source,
            crate::scanner::TokenType::EOF,
            "Expect end of expression".to_string());

    end_compiler(&mut chunk, &mut parser);
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

fn emit_byte(mut chunk: &mut crate::chunk::Chunk,
             mut parser: &mut Parser,
             byte: u8) {
    crate::chunk::write_chunk(&mut chunk, byte, parser.previous.as_ref().unwrap().line);
}

fn emit_bytes(mut chunk: &mut crate::chunk::Chunk,
              mut parser: &mut Parser,
              byte0: u8,
              byte1: u8) {
    emit_byte(&mut chunk, &mut parser, byte0);
    emit_byte(&mut chunk, &mut parser, byte1)
}

fn end_compiler(mut chunk: &mut crate::chunk::Chunk,
                mut parser: &mut Parser) {
    emit_return(&mut chunk, &mut parser)
}

fn emit_return(mut chunk: &mut crate::chunk::Chunk,
               mut parser: &mut Parser) {
    emit_byte(&mut chunk, &mut parser,
              opcode!(OPRETURN))
}

// call
// we've entered an ex-expression
// we eval everything until close-paren
// then emit the cdr
// (+ 1 (+ 2 3))
// 1 2 3 + +
// 1 2 3 + +  ->
// 1 5 +  ->
// 6
fn apply(mut parser: &mut Parser,
         mut scanner: &mut crate::scanner::Scanner,
         mut chunk: &mut crate::chunk::Chunk,
         source: &str) {

    let car = &parser.current;
    advance(&mut parser, &mut scanner, source);

    loop {
        if (parser.current.unwrap().typ == crate::scanner::TokenType::RIGHTPAREN) {
            advance(&mut parser, &mut scanner, source);
            break;
        }

        expression(&mut parser, &mut scanner, &mut chunk, source);
    }

    // we need to actually eval the car to get the fn to apply...
    //((flip + -) 1 2) ->
    // 1 2 + - flip ->
    // 1 2 +  // eg
    // 3
    // so hmm we want to call expression on `car` now but we can't
    // since the whole thing has changed...
    expression()

// this is eval
fn expression(mut parser: &mut Parser,
              mut scanner: &mut crate::scanner::Scanner,
              mut chunk: &mut crate::chunk::Chunk,
              source: &str) {

    match parser.current.unwrap().typ {
        crate::scanner::TokenType::LEFTPAREN => apply(&mut chunk,
                                                      &mut parser,
                                                      source),
        crate::scanner::TokenType::NUMBER => number(&mut chunk,
                                                    &mut parser,
                                                    source),

    }

    // we want to dispatch based on the current token.
    // we can make a table based on the u8 of each token, for constant
    // time lookup.
    // - if it's a number, we call number()
    // - if it's a callable, we return it (+,- etc): functions have
    //   valueso
    // - if it's an open paren, we advance and recursively call
    //   expression? no, because we need first element to be callable.
    //   so that suggests we want to call `apply` or something, which
    //   expects the token to be a callable (how? a table?), then
    //   calls `eval` on its operands, pushing the result of each.
    //   then eval's its first element, then pushes that result.
    //   we can know the expected number of operands for the operator
    //   from the table, too, and confirm that we get a close paren
    //   when we expect to. variafdi

}

fn number(mut chunk: &mut crate::chunk::Chunk,
          mut parser: &mut Parser,
          source: &str) {
    let start = parser.previous.as_ref().unwrap().start;
    let len = parser.previous.as_ref().unwrap().length;
    let d = f64::from_str(&source[start..start+len]).unwrap();
    emit_constant(&mut chunk, &mut parser, d)
}


fn emit_constant(mut chunk: &mut crate::chunk::Chunk,
                 mut parser: &mut Parser,
                 val: crate::value::Value) {

    let constant_ix = make_constant(
        &mut chunk,
        &mut parser,
        val);

    emit_bytes(&mut chunk,
               &mut parser,
               opcode!(OPCONSTANT),
               constant_ix)
}

fn make_constant(mut chunk: &mut crate::chunk::Chunk,
                 mut parser: &mut Parser,
                 val: crate::value::Value) -> u8 {
    let id = crate::chunk::add_constant(&mut chunk,
                                        val);
    u8::try_from(id).unwrap()
}

pub fn advance(mut parser: &mut Parser,
               scanner: &mut crate::scanner::Scanner,
               source: &str) {
    loop {
        // "move" current to previous.
        mem::swap(&mut parser.previous, &mut parser.current);
        parser.current = Some(crate::scanner::scan_token(scanner,
                                                         source));

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
