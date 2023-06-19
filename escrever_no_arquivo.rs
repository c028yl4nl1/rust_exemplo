use std::fs::File;
use std::io::prelude::*;
#[derive(Debug)]
struct Informa {

    nome:String , 
    id: i32 , 
    cidade: String,

}

fn main() {

    let mut nome_do_arquivo = File::create("Lanby2.txt").expect("Erro ao Cria o arquivo");
    let mut conteudo = String::new();

    let add = Informa {nome: String::from("lanbyShell"), id: 332 as i32 , cidade: String::from(" Bahia")};
    

    let dados = format!("{:#?}",add).to_string(); 
   
    conteudo = dados; 


    nome_do_arquivo.write_all(conteudo.as_bytes()).expect("Erro"); 


}
