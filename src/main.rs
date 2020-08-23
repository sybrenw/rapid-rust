use rapid_rust;

mod lexer;
mod parser;

fn main() {
    println!("Hello, world!");
    let tokens = lexer::parse("\
MOD Testmodule 
    PROC rTest() 
        VAR num nTest1:=0; 
        nTest1:= 2 + 2 * 3 *4 + 1;
        TpWrite nTest1;
    ENDPROC 
ENDMOD");
    match parser::parse_tokens(tokens) {
        Err(err) => println!("Error: {}", err),
        _ => ()
    }
}

#[cfg(test)]
mod test{
    use super::*;

    #[test]
    fn my_test() {
        
    }
}