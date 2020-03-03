extern crate num_derive;
use num::{ToPrimitive};

use std::mem;
use std::str::FromStr;
use std::convert::TryFrom;
use std::rc::Rc;
use indextree::Arena;

macro_rules! opcode {
    ($op:tt) => {
        crate::chunk::Opcode::to_u8(&crate::chunk::Opcode::$op).unwrap()
    };
}

#[derive(Debug)]
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

#[derive(Debug)]
pub struct ASTParser<'a> {
    pub current: Rc<Option<crate::scanner::Token>>,
    pub previous: Rc<Option<crate::scanner::Token>>,
    pub had_error: bool,
    pub panic_mode: bool,
    pub scanner: &'a mut crate::scanner::Scanner<'a>,
    pub source: &'a str,
}

fn build_ast(mut parser: ASTParser,
             mut ast: indextree::Arena<Rc<Option<crate::scanner::Token>>>) {

    let root = ast.new_node(Rc::new(Some(crate::scanner::Token{
        typ: crate::scanner::TokenType::NOOP,
        line: 0,
        start: 0,
        length: 0,
        error: None
    })));

    ast_advance(&mut parser);
    ast_expression(&mut parser, &mut ast, root);
}

fn ast_expression(parser: &mut ASTParser,
                  ast: &mut indextree::Arena<Rc<Option<crate::scanner::Token>>>,
                  parent: indextree::NodeId) {

    match parser.current.as_ref().as_ref().unwrap().typ {

        crate::scanner::TokenType::LEFTPAREN => {
            println!("LEFT PAREN!!");
            let current = Rc::clone(&parser.current);
            let subtree = ast.new_node(current);
            parent.append(subtree, ast);

            loop {
                ast_advance(parser);
                if parser.current.as_ref().as_ref().unwrap().typ ==
                    crate::scanner::TokenType::RIGHTPAREN {
                        println!("CLOSE PAREN!");
                        return;
                    }

                ast_expression(parser, ast, subtree);
            }

        },

        _ => {
            println!("SOMETHING ELSEE; {:?}", parser.current);
            let current = Rc::clone(&parser.current);
            let elem = ast.new_node(current);
            parent.append(elem, ast);
        }
    }
}

pub fn ast_advance(parser: &mut ASTParser) {
    loop {
        // "move" current to previous.
        mem::swap(&mut parser.previous, &mut parser.current);

        parser.current = Rc::new(Some(crate::scanner::scan_token(parser.scanner,
                                                                 parser.source)));

        match &parser.current.as_ref().as_ref().unwrap().error {
            Some(err) => {
                let message = err.to_string();
                ast_error_at_current(parser, message, parser.source)
            },
            None => break
        }
    }
}

fn ast_error_at_current(parser: &mut ASTParser, message: String, source: &str) {
    if parser.panic_mode {
            return;
        }
        parser.had_error = true;
        parser.panic_mode = true;
        ast_error_at(parser, message, source)
    }

fn ast_error_at(parser: &mut ASTParser,
                message: String,
                source: &str) {
        let token = parser.previous.as_ref().as_ref().unwrap();

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



// take source, build a Chunk
pub fn compile(source: &str,
               mut chunk: &mut crate::chunk::Chunk) -> bool {

    // code -> AST
    let mut scanner = crate::scanner::init_scanner();
    let mut ast_parser =  ASTParser{current: Rc::new(None),
                                    previous: Rc::new(None),
                                    had_error: false,
                                    panic_mode: false,
                                    scanner: &mut scanner,
                                    source: source};
    let mut ast = Arena::<Rc<Option<crate::scanner::Token>>>::new();
    build_ast(ast_parser, ast);

    let mut parser = init_parser();

    // parser.advance(&mut scanner, source);
    // parser.expression(&mut scanner, &mut chunk, source);
    // parser.consume(&mut scanner, source,
    //                crate::scanner::TokenType::EOF,
    //                "Expect end of expression".to_string());

    // parser.end_compiler(&mut chunk);
    !parser.had_error
}

impl Parser {
    // XXX this will use AST, not scanner
    pub fn advance(&mut self,
                   scanner: &mut crate::scanner::Scanner,
                   source: &str) {
        loop {
            // "move" current to previous.
            mem::swap(&mut self.previous, &mut self.current);
            self.current = Some(crate::scanner::scan_token(scanner,
                                                           source));

            match &self.current.as_ref().unwrap().error {
                Some(err) => {
                    let message = err.to_string();
                    self.error_at_current(message, source)
                },
                None => break
            }
        }
    }

    // given a token, advance one to exactly that token or throw
    pub fn consume(&mut self,
                   scanner: &mut crate::scanner::Scanner,
                   source: &str,
                   token_type: crate::scanner::TokenType,
                   message: String) {

        if self.current.as_ref().unwrap().typ == token_type {
            self.advance(scanner, source);
            return;
        }

        self.error_at_current(message, source)
    }

    fn expression(&mut self,
                  mut scanner: &mut crate::scanner::Scanner,
                  mut chunk: &mut crate::chunk::Chunk,
                  source: &str) {

        ()
        // match self.current.as_ref().unwrap().typ {
        //     crate::scanner::TokenType::LEFTPAREN =>
        //         self.apply(
        //             &mut scanner,
        //             &mut chunk,
        //             source),
        //     crate::scanner::TokenType::NUMBER =>
        //         self.number(&mut chunk,
        //                     source)
        // }

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

    fn error_at_current(&mut self, message: String, source: &str) {
        if self.panic_mode {
            return;
        }
        self.had_error = true;
        self.panic_mode = true;
        self.error_at(message, source)
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
    fn apply(&mut self,
             mut scanner: &mut crate::scanner::Scanner,
             mut chunk: &mut crate::chunk::Chunk,
             source: &str) {

        // let car = self.current;
        self.advance(&mut scanner, source);

        loop {
            if self.current.as_ref().unwrap().typ == crate::scanner::TokenType::RIGHTPAREN {
                self.advance(&mut scanner, source);
                break;
            }

            self.expression(&mut scanner, &mut chunk, source);
        }

        // we need to actually eval the car to get the fn to apply...
        //((flip + -) 1 2) ->
        // 1 2 + - flip ->
        // 1 2 +  // eg
        // 3
        // so hmm we want to call expression on `car` now but we can't
        // since the whole thing has changed...
        // self.expression()
    }




    fn emit_byte(&mut self,
                 chunk: &mut crate::chunk::Chunk,
                 byte: u8) {

        chunk.write_chunk(byte, self.previous.as_ref().unwrap().line);
    }

    fn emit_bytes(&mut self,
                  mut chunk: &mut crate::chunk::Chunk,
                  byte0: u8,
                  byte1: u8) {
        self.emit_byte(&mut chunk, byte0);
        self.emit_byte(&mut chunk, byte1)
    }

    fn end_compiler(&mut self,
                    mut chunk: &mut crate::chunk::Chunk) {
        self.emit_return(&mut chunk)
    }

    fn emit_return(&mut self,
                   mut chunk: &mut crate::chunk::Chunk) {
        self.emit_byte(&mut chunk,
                       opcode!(OPRETURN))
    }


    fn number(&mut self,
              mut chunk: &mut crate::chunk::Chunk,
              source: &str) {
        let start = self.previous.as_ref().unwrap().start;
        let len = self.previous.as_ref().unwrap().length;
        let d = f64::from_str(&source[start..start+len]).unwrap();
        self.emit_constant(&mut chunk, d)
    }


    fn emit_constant(&mut self,
                     mut chunk: &mut crate::chunk::Chunk,
                     val: crate::value::Value) {

        let constant_ix = self.make_constant(&mut chunk,
                                             val);

        self.emit_bytes(&mut chunk,
                        opcode!(OPCONSTANT),
                        constant_ix)
    }

    fn make_constant(&self,
                     mut chunk: &mut crate::chunk::Chunk,
                     val: crate::value::Value) -> u8 {
        let id = chunk.add_constant(val);
        u8::try_from(id).unwrap()
    }


    fn error(&mut self, message: String, source: &str) {
        if self.panic_mode {
            return;
        }
        self.had_error = true;
        self.panic_mode = true;
        self.error_at(message, source);
    }

    fn error_at(&mut self,
                message: String,
                source: &str) {
        let token = &self.previous.as_ref().unwrap();

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


}
