//  criando um jogo em rust , um simples jogo de cli. 

// libs


use std::io::stdin; 
extern crate rand;


use rand::Rng;


// libs

#[warn(non_camel_case_types)]
pub  struct Receber_valor {

    valor: String ,

}

impl Receber_valor {

    pub fn convert(&self) -> Result<i32 , bool> {

        let valor = self.valor.trim().parse::<i32>();

        match valor{

            Ok(_) => Ok(valor.unwrap()), 
            Err(_) => Err(false), 
        }


    }
}

fn main() { 
    // mensagem de boas vindas //
    help();
    let mut vidas: i32  = 3; 

    loop {

        println!("\nRestam {} Vidas ", vidas); 
        let random = gera_um_numero()  + 1 ; 
        let numero = pergunta_valor(); 

        if numero > 50 {
            limpar_tela();
            println!("Digite o numero entre 1 a 10 não pode passar desse limite ");

        }
        else if numero as i32 != 0 as i32 {

            // minha logica vai aqui

            limpar_tela();
            

            if numero != random {

                vidas -= 1; 
                println!("Você Errou O computador jogou {} ! Só restam {} vidas agora. \nSe você perder todas as suas vidas, não haverá mais chances de recuperá-las. \nPortanto, jogue com sabedoria e tente aproveitar ao máximo cada vida que você tem. \nBoa sorte! \nClick Enter para continua", random,vidas); 
                enter();
            }
            
            else {
                println!("Parabéns! Você acertou e agora ganhou mais uma vida!"); 
                vidas += 1; 
           
            }

        }
        if vidas == 0{
            println!(" Voce gastou todas as suas vidas");
            break;
        }
    
       
    }
}


fn pergunta_valor() -> i32 {

    println!("\r\nDigite o valor abaixo : ") ; 
    let mut pergunta_valor__ = String::from("\n");

    stdin().read_line(&mut pergunta_valor__).expect("ao comprender a escrita ");
    let pergunta_valor_ = pergunta_valor__.trim(); 
    if pergunta_valor_ == "help".to_string() || pergunta_valor_ == "Help".to_string(){

        
        help(); 
        0 

    }
    else {

    let valor_input = Receber_valor{valor:pergunta_valor__};

    let converter = valor_input.convert();

    match  converter {
        Ok(_) => {
            converter.unwrap() as i32 
        },
        Err(_) => {
            limpar_tela(); 
            println!("Por favor digite apenas numero:  de 1 a 10 ");
            0
        }
    }
    
    }


}



fn limpar_tela() {

    println!("\x1B[2J\x1B[1;1H");
}

fn help() {
    limpar_tela();
    println!("\rEste é um simples jogo de adivinhação. \nVocê tem três vidas para adivinhar o que está escondido. \nSe você errar, perderá uma vida. Mas se voce acerta ganha mais uma \nSe você perder todas as três vidas, sairá do jogo. \nBoa sorte! - digite help para ver a ajuda. ");


}

fn gera_um_numero() -> i32 {

    let mut numero_secreto = rand::thread_rng(); 
    let random_number: i32 = numero_secreto.gen_range(1..9);

    random_number 
}

fn enter() {

    let mut valor = String::new(); 
    
    stdin().read_line(&mut valor).expect("Buffer");
    limpar_tela(); 
}





