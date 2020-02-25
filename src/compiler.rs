pub fn compile(source: &str) {
    let mut scanner = crate::scanner::init_scanner();
    let mut line: u16 = 0;

    loop {
        let token = crate::scanner::scan_token(&mut scanner, source);
        if token.line != line {
            print!("{:4} ", token.line);
            line = token.line;
        } else {
            print!("    | ");
        }

        println!("{:?} {}", token.typ, &source[token.start..token.start+token.length]);

        if token.typ == crate::scanner::TokenType::EOF { break ; }
    }

}
