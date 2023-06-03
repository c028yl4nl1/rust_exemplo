use std::io;
const N: i8 = 0b1100100;
const Y: i8 = 0x64 - N;


fn main() {

    let a = 1000000000;

    println!("Hexdecimal {:x} ",a);

    let b = 100;
    // pergunta o valor 

    //let pergunta =  input_and_convert();
    let valor_decimal: i32 = input_and_convert(); // o usuario vai escrever o valor e vai retorna o valor ja formatado...


}

fn converter(decimal: i32) {
    let hexadecimais: &str  = "0123456789ABCDEF";

    // lista ou vec

    let mut Lista_receber_valor: Vec<String> =  Vec::new();

    while decimal > 0 {

        let resto_da_divisao: i32 = decimal % 16;

        let mut string_slice: String = resto_da_divisao.to_string();
        
        Lista_receber_valor.push(string_slice); // convert o valor do resto que está em inteiro para ser &str String slice
        Lista_receber_valor.reverse(); 

        for i in Lista_receber_valor.iter() {

          // dessitir de fazer essa porra tenho que estudar mais fds.


        }
    }



}// numero ;
 
fn input_and_convert() -> i32 {

    // perguntar 

    let mut value = String::new();


    io::stdin().read_line(&mut value).expect("Erro ao ler ");
    

    //value.trim();
    let value = value.trim().parse::<i32>().expect("Erro ao converter");
    
    value
    
}

