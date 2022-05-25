use std::env;
use std::process::exit;

mod lexer;
use lexer::tokenize;

fn help() {
    println!("Useage janus [filename]");
    exit(-1);
}


fn main() {
    let file = env::args().nth(1);

    let tokens = match file {
         None => { help(); vec![] },
         Some(filename) => { tokenize(filename) },
    };

    for tok in tokens {
        println!("{:#?}", tok);
    }

}
