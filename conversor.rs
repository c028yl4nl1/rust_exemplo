use std::io::stdin; 

struct Numero{
    numero: i32 , 
}


impl Numero {
    fn converter_Binario(&self){ 
        println!("decimal: {} and Binario {:b} ", self.numero , self.numero);
    }

    fn converter_hex(&self){

        println!("Decimal {} and Hexdecimal {:x} ", self.numero, self.numero);

    }
    fn converter_octal(&self) {
        println!("Decimal {} and Octal {:o} ", self.numero , self.numero);
    }
}
fn main() {

    
    loop {
        let mut valor = String::new(); // menmori heap 

        println!("
        \r1 - Converter em Binario
        \r2 - Converter em Hexdecimal 
        \r3 - Converter em Octal 
        \r4 - All 
        ");
         
        match stdin().read_line(&mut valor) {
            Ok(_) => {
                // limpar_tela();

                let parse = valor.trim().parse::<i32>();

                match parse {

                    Ok(parse) => {
                        
                        if parse == 1 {
                            limpar_tela(); 
                            println!("Digite o Numero para converter em Binario : ");
                            let mut Bytes_ = String::new();
                            match stdin().read_line(&mut Bytes_) {
                                Ok(_) => { 
                                    let corta_string = Bytes_.trim().parse::<i32>(); 
                                    match corta_string {
                                        Ok(corta_string) => {
                                            limpar_tela(); 
                                           let Numero_ = Numero{ numero: corta_string };
                                           Numero_.converter_Binario(); 

                                        },
                                        Err(_) => println!("Digite Apenas numero"),
                                    }
                                }

                            Err(_) => println!("Buffer 0x33"), 
                            }
                        
                            
                        }
                        else if parse == 2 {
                            limpar_tela(); 
                            println!("Digite o Numero para converter em Hexdecimal : ");
                            let mut hex_ = String::new();
                            match stdin().read_line(&mut hex_) {
                                Ok(_) => { 
                                    let corta_string = hex_.trim().parse::<i32>(); 
                                    match corta_string {
                                        Ok(corta_string) => {
                                            limpar_tela(); 
                                           let Numero_ = Numero{ numero: corta_string };
                                           Numero_.converter_hex(); 

                                        },
                                        Err(_) => println!("Digite Apenas numero"),
                                    }
                                }

                            Err(_) => println!("Buffer 0x33"), 
                            }
                        
                            
                        }
                        else if  parse == 3 {
                            limpar_tela(); 
                            println!("Digite o Numero para converter em Octal : ");
                            let mut oc_ = String::new();
                            match stdin().read_line(&mut oc_) {
                                Ok(_) => { 
                                    let corta_string = oc_.trim().parse::<i32>(); 
                                    match corta_string {
                                        Ok(corta_string) => {
                                            limpar_tela(); 
                                           let Numero_ = Numero{ numero: corta_string };
                                           Numero_.converter_octal(); 

                                        },
                                        Err(_) => println!("Digite Apenas numero"),
                                    }
                                }

                            Err(_) => println!("Buffer 0x33"), 
                            }
                        
                            
                        }

                        else if parse == 4 {
                            limpar_tela(); 
                            println!("Digite o Numero para converter: Hexdecimal , Octal e Binario: ");
                            let mut all_ = String::new();
                            match stdin().read_line(&mut all_) {
                                Ok(_) => { 
                                    let corta_string = all_.trim().parse::<i32>(); 
                                    match corta_string {
                                        Ok(corta_string) => {
                                            limpar_tela(); 
                                           let Numero_ = Numero{ numero: corta_string };
                                           Numero_.converter_Binario(); 
                                           Numero_.converter_hex();
                                           Numero_.converter_octal();

                                        },
                                        Err(_) => println!("Digite Apenas numero"),
                                    }
                                }

                            Err(_) => println!("Buffer 0x33"), 
                            }
                        
                            
                        }
                        else {
                            limpar_tela();
                            println!("Opção não reconhecida: ")
                        }
                    },
                    Err(_) => {limpar_tela(); println!("Digite apenas Numero");  }, // println!("Digite apenas numero "), 
                    
                }
            },
            Err(_) => println!(" Erro Ao ler o input "), 
        }


    }


}



fn limpar_tela() {

    println!("\x1B[2J\x1B[1;1H");
}
