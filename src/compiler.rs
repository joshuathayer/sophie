extern crate num_derive;
use num::{ToPrimitive};

use std::str::FromStr;
use std::convert::TryFrom;
use std::rc::Rc;
use indextree::Arena;
use indextree::Node;
use indextree::NodeId;

macro_rules! opcode {
    ($op:tt) => {
        crate::chunk::Opcode::to_u8(&crate::chunk::Opcode::$op).unwrap()
    };
}

#[derive(Debug)]
pub struct Generator {
    pub current: Rc<Option<crate::scanner::Token>>,
    pub had_error: bool,
    pub panic_mode: bool,
}

fn init_generator() -> Generator {
    Generator {
        current: Rc::new(None),
        had_error: false,
        panic_mode: false,
    }
}

#[derive(Debug)]
pub struct ASTParser<'a> {
    pub current: Rc<Option<crate::scanner::Token>>,
    pub had_error: bool,
    pub panic_mode: bool,
    pub scanner: &'a mut crate::scanner::Scanner<'a>,
    pub source: &'a str,
}

fn build_ast(mut parser: ASTParser,
             ast: &mut Arena<Rc<Option<crate::scanner::Token>>>) -> NodeId {

    let root = ast.new_node(Rc::new(Some(crate::scanner::Token{
        typ: crate::scanner::TokenType::NOOP,
        line: 0,
        start: 0,
        length: 0,
        error: None})));

    ast_advance(&mut parser);

    // because at the top level of a file there may be many expressions,
    // we loop through them here until we hit the EOF. this is
    // effectively a `do`
    loop {
        if &parser.current.as_ref().as_ref().unwrap().typ == &crate::scanner::TokenType::EOF {
            break
        }

        // this does one s-expression
        ast_expression(&mut parser, ast, root);
        ast_advance(&mut parser);
    }

    root
}

fn ast_expression(parser: &mut ASTParser,
                  ast: &mut indextree::Arena<Rc<Option<crate::scanner::Token>>>,
                  parent: indextree::NodeId) {

    match parser.current.as_ref().as_ref().unwrap().typ {

        crate::scanner::TokenType::LEFTPAREN => {
            let current = Rc::clone(&parser.current);
            let subtree = ast.new_node(current);
            parent.append(subtree, ast);

            loop {
                ast_advance(parser);
                if parser.current.as_ref().as_ref().unwrap().typ ==
                    crate::scanner::TokenType::RIGHTPAREN {
                        return;
                    }

                ast_expression(parser, ast, subtree);
            }

        },

        _ => {
            let current = Rc::clone(&parser.current);
            let elem = ast.new_node(current);
            parent.append(elem, ast);
        }
    }
}

pub fn ast_advance(parser: &mut ASTParser) {
    loop {
        print!("Advance!\n");

        parser.current = Rc::new(
            Some(crate::scanner::scan_token(parser.scanner,
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

fn ast_error_at_current(parser: &mut ASTParser,
                        message: String,
                        source: &str) {
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
        let token = parser.current.as_ref().as_ref().unwrap();

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
    let ast_parser =  ASTParser{current: Rc::new(None),
                                had_error: false,
                                panic_mode: false,
                                scanner: &mut scanner,
                                source: source};

    let mut ast = Arena::<Rc<Option<crate::scanner::Token>>>::new();
    let root_id = build_ast(ast_parser, &mut ast);
    let root = ast.get(root_id);

    let mut generator = init_generator();

    let mut child = root.unwrap().first_child();

    loop {
        print!("CHILD {:?}\n", child);
        match child {
            None => break,
            Some(n) => {
                let node = ast.get(n).unwrap();
                generator.expression(&ast, &node, &mut chunk, source);
                child = node.next_sibling();

                // tif there's more than one expression at the top of
                // the AST, we want to eval each in turn (presumably
                // they side-effect), and throw away each expression's
                // value until the last one. so we tell the VM to pop
                // into nothing.`do` will have to do similar.
                // XXX also it's stupid how we pass a
                // token all the way through to emit_* when all we
                // really care about is the line number. fix this. and
                // in this case (and `end_compiler`), just grab the
                // value of the head of `chunk.lines` and use that
                if child.is_some() {
                    generator.emit_pop(&mut chunk,

                                       // this token is a placeholder,
                                       // see comment above
                                       &crate::scanner::Token {
                                           typ: crate::scanner::TokenType::NOOP,
                                           line: 0,
                                           start: 0,
                                           length: 0,
                                           error: None},
                    )
                }
            }
        }
    }

    generator.end_compiler(&crate::scanner::Token {
                               typ: crate::scanner::TokenType::NOOP,
                               line: 0,
                               start: 0,
                               length: 0,
                               error: None},
                           &mut chunk);

    // !parser.had_error

    true
}

// rename from Action?
type Action = fn(&mut Generator,
                 &mut crate::chunk::Chunk,
                 &crate::scanner::Token,
                 &str);

static token_fn: [Action; 51] = [
    Generator::noop,

    Generator::noop, Generator::noop,
    Generator::noop, Generator::noop,
    Generator::noop, Generator::noop,
    Generator::noop, Generator::noop,
    Generator::noop, Generator::noop, Generator::op, Generator::op,
    Generator::noop, Generator::noop, Generator::op,

    Generator::noop, Generator::op,
    Generator::op, Generator::op,
    Generator::op, Generator::op,
    Generator::op, Generator::op,

    // Literals
    Generator::identifier, Generator::string,
    Generator::float, Generator::int, Generator::noop,
    Generator::literal, Generator::literal,
    Generator::literal,

    // Keywords
    Generator::noop, Generator::noop, Generator::noop,
    Generator::noop, Generator::noop, Generator::noop,
    Generator::noop, Generator::op, Generator::noop, Generator::noop,
    Generator::noop, Generator::noop, Generator::noop,
    Generator::noop, Generator::op, Generator::op,
    Generator::op,

    Generator::noop,
    Generator::noop
];

// works on the AST
// pushes onto the bytecode in Chunk
impl Generator {

    // we are given a node
    fn expression(&mut self,
                  ast: &Arena::<Rc<Option<crate::scanner::Token>>>,
                  node: &Node::<Rc<Option<crate::scanner::Token>>>,
                  mut chunk: &mut crate::chunk::Chunk,
                  source: &str) {

        let t = node.get().as_ref();

        print!("t is {:?}\n", t);
        match t {

            // a single token

            // using match here isn't quire right, since it won't
            // work with `((if true + -) 1 2)`. use an if.
            Some(token)
                if token.typ != crate::scanner::TokenType::LEFTPAREN =>
                { self.emit_token(&mut chunk, token, source)},

            // an S-expression
            _ => {
                let first_child = ast
                    .get(node.first_child().unwrap())
                    .unwrap();

                // handle nonstandard forms
                // match first_child.get().as_ref() {
                //     Some(n) if n.typ == crate::scanner::TokenType::DEF => {
                //         // ok it's a def. so we expect an identifier
                //         // next, which we need in the symbol table...
                //         // this should all be refactored into its own fn
                //         let identifier =
                //             ast
                //             .get(first_child.next_sibling().unwrap())
                //             .unwrap().get().as_ref().unwrap();

                //         // this will stick a constant into the table
                //         // and emit a const into the bytecode
                //         // not sure that's what we want, since bytecode
                //         // will end up like
                //         // [ def, exp, symbol ] rather than
                //         // [ def, symbol, exp ]
                //         let ix = self.string(&mut chunk,
                //                              &identifier,
                //                              source);


                //     }

                // }

                // otherwise assume we're in a prefix expression
                // we generate operands, then operator
                let mut next_child = first_child.next_sibling();

                // catch semantic problems (arity, types?)
                // example: `(def n)` vs `(def n 1)`
                // should the first form default to nil?
                // in thast case, this is probably the place to do it
                // we'd need logic here to say "is `first` == DEF? if
                // so, if `rest` is len 0, then emil a nil here. if
                // `resst` is len 1, then emit its element. if `rest`
                // is len > 1, it's an error"
                // ~or~ we go fully variadic and emit the operand count
                // here. or both- we'd have a table of op->arity:
                //   + -> variadic
                //   def -> [0,1]
                //   not -> 1
                // then here we know to check those bounds,
                // and also if we should emit the argument count
                // for now we say `def` will only every have 1 arg
                loop {
                    print!("next {:?}\n", next_child);
                    match next_child {
                        None => break,
                        Some(n) => {
                            let node = ast.get(n).unwrap();
                            self.expression(ast,
                                            &node,
                                            &mut chunk,
                                            source);
                            next_child = node.next_sibling();
                        }
                    }
                }

                // if our VM is to support variadic ops, we'd want
                // to push the count of operands here


                // finally, push on the operator
                self.expression(ast, first_child, &mut chunk, source);
            }
        }

        ()
    }

    fn emit_token(&mut self,
                  mut chunk: &mut crate::chunk::Chunk,
                  token: &crate::scanner::Token,
                  source: &str) {

        let f = token_fn[crate::scanner::TokenType::to_u8(&token.typ).unwrap() as usize];

        f(self, &mut chunk, &token, source);

    }

    fn literal(&mut self,
               chunk: &mut crate::chunk::Chunk,
               token: &crate::scanner::Token,
               _source: &str) {
        let byte = match token.typ {
            crate::scanner::TokenType::NIL =>
                crate::chunk::Opcode::OPNIL,
            crate::scanner::TokenType::TRUE =>
                crate::chunk::Opcode::OPTRUE,
            crate::scanner::TokenType::FALSE =>
                crate::chunk::Opcode::OPFALSE,
            _ =>
                return
        };
        self.emit_byte(chunk,
                       token,
                       crate::chunk::Opcode::to_u8(&byte).unwrap())
    }

    fn op(&mut self,
          chunk: &mut crate::chunk::Chunk,
          token: &crate::scanner::Token,
          _source: &str) {

        let byte = match token.typ {
            crate::scanner::TokenType::PLUS =>
                crate::chunk::Opcode::OPADD,
            crate::scanner::TokenType::MINUS =>
                crate::chunk::Opcode::OPSUBTRACT,
            crate::scanner::TokenType::STAR =>
                crate::chunk::Opcode::OPMULTIPLY,
            crate::scanner::TokenType::SLASH =>
                crate::chunk::Opcode::OPDIVIDE,
            crate::scanner::TokenType::NOT =>
                crate::chunk::Opcode::OPNOT,
            crate::scanner::TokenType::EQUAL =>
                crate::chunk::Opcode::OPEQUAL,
            crate::scanner::TokenType::LESS =>
                crate::chunk::Opcode::OPLT,
            crate::scanner::TokenType::GREATER =>
                crate::chunk::Opcode::OPGT,
            crate::scanner::TokenType::LESSEQUAL =>
                crate::chunk::Opcode::OPLTE,
            crate::scanner::TokenType::GREATEREQUAL =>
                crate::chunk::Opcode::OPGTE,
            crate::scanner::TokenType::LEN =>
                crate::chunk::Opcode::OPLEN,
            crate::scanner::TokenType::PRINT =>
                crate::chunk::Opcode::OPPRINT,
            crate::scanner::TokenType::DEF =>
                crate::chunk::Opcode::OPDEF,

            _ =>
                return
        };

        self.emit_byte(chunk,
                       token,
                       crate::chunk::Opcode::to_u8(&byte).unwrap())
    }


    fn emit_byte(&mut self,
                 chunk: &mut crate::chunk::Chunk,
                 token: &crate::scanner::Token,
                 byte: u8) {
        chunk.write_chunk(byte, token.line);
    }

    fn emit_bytes(&mut self,
                  mut chunk: &mut crate::chunk::Chunk,
                  token: &crate::scanner::Token,
                  byte0: u8,
                  byte1: u8) {
        self.emit_byte(&mut chunk, token, byte0);
        self.emit_byte(&mut chunk, token, byte1)
    }

    fn end_compiler(&mut self,
                    token: &crate::scanner::Token,
                    mut chunk: &mut crate::chunk::Chunk) {
        self.emit_return(&mut chunk, token)
    }

    fn emit_return(&mut self,
                   mut chunk: &mut crate::chunk::Chunk,
                   token: &crate::scanner::Token) {
        self.emit_byte(&mut chunk,
                       token,
                       opcode!(OPRETURN))
    }

    fn emit_pop(&mut self,
                mut chunk: &mut crate::chunk::Chunk,
                token: &crate::scanner::Token) {
        self.emit_byte(&mut chunk,
                       token,
                       opcode!(OPPOP))
    }


    fn noop(&mut self,
            mut _chunk: &mut crate::chunk::Chunk,
            token: &crate::scanner::Token,
            _source: &str) {
        println!("Strangely, I am in NOOP with token {:?}", token);
        ()
    }

    fn float(&mut self,
              mut chunk: &mut crate::chunk::Chunk,
              token: &crate::scanner::Token,
              source: &str) {
        let start = token.start;
        let len = token.length;
        let d = f64::from_str(&source[start..start+len]).unwrap();
        let ct = crate::value::ConstantType::FLOAT(d);

        self.emit_constant(&mut chunk,
                           token,
                           ct)
    }

    fn int(&mut self,
             mut chunk: &mut crate::chunk::Chunk,
             token: &crate::scanner::Token,
             source: &str) {
        let start = token.start;
        let len = token.length;
        let d = i64::from_str(&source[start..start+len]).unwrap();
        let ct = crate::value::ConstantType::INT(d);

        self.emit_constant(&mut chunk,
                           token,
                           ct)
    }

    fn string(&mut self,
              mut chunk: &mut crate::chunk::Chunk,
              token: &crate::scanner::Token,
              source: &str) {

        let start = token.start;
        let len = token.length;
        let s = source[start..start+len-1].to_owned();

        // let sv = string_val!(&ct);
        let ct = crate::value::ConstantType::STRING(s);

        self.emit_constant(&mut chunk,
                           token,
                           ct)
    }

    fn identifier(&mut self,
              mut chunk: &mut crate::chunk::Chunk,
              token: &crate::scanner::Token,
              source: &str) {

        let start = token.start;
        let len = token.length;
        let s = source[start..start+len-1].to_owned();

        // let sv = string_val!(&ct);
        let ct = crate::value::ConstantType::STRING(s);

        self.emit_constant(&mut chunk,
                           token,
                           ct)
    }


    fn emit_constant(&mut self,
                     mut chunk: &mut crate::chunk::Chunk,
                     token: &crate::scanner::Token,
                     val: crate::value::ConstantType) {

        // moves val to chunk
        let constant_ix = self.make_constant(&mut chunk,
                                             val);

        self.emit_bytes(&mut chunk,
                        token,
                        opcode!(OPCONSTANT),
                        constant_ix)

    }

    fn make_constant(&self,
                     chunk: &mut crate::chunk::Chunk,
                     val: crate::value::ConstantType) -> u8 {

        let id = chunk.add_constant(val);
        u8::try_from(id).unwrap()
    }


    fn error(&mut self,
             token: &crate::scanner::Token,
             message: String,
             source: &str) {
        if self.panic_mode {
            return;
        }
        self.had_error = true;
        self.panic_mode = true;
        self.error_at(token, message, source);
    }

    fn error_at(&mut self,
                token: &crate::scanner::Token,
                message: String,
                source: &str) {

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
