#[path = "./gust-bytecode/mod.rs"]
mod bytecode;
#[path = "./gust-compiler/mod.rs"]
mod compiler;
#[path = "./gust-vm/mod.rs"]
mod vm;

use compiler::lexer::Lexer;

fn main() {
    let source = r#"
    {}[]
    != < > <= >= = == ()
    let x = 10
        let y = x
        let z = x + 5
        if y >= x && z != 10 {
            let hello = 100
        }
        let long_name = true"#;
    let mut lexer = Lexer::new(source);
    loop {
        let token = lexer.next();
        if token.is_some() {
            println!("Token {:?}", token);
        } else {
            break;
        }
    }
    println!("END");
}
