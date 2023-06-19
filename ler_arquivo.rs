use std::fs::File;
use std::io::prelude::*;

fn main() {

    println!("Ola mundo ");

    let mut arquivo = File::open("lanby.txt").unwrap();

    let mut conteudo = String::new();
    let _ = arquivo.read_to_string(&mut conteudo);
    println!("{:?}", conteudo.bytes());


}
