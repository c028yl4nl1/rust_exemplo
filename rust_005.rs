struct Types{  // Primeiro eu fiz uma Struct

    largura: i32, // criei as os metodos de entrada da Struct
    comprimento: i32, 
}

impl Types{ // Implementei a Struct Types 

    fn pixels(&self) -> i32 { // Criei uma Função dentro da Implementação.
        // Tenho que retorna um valor , é obrigatorio.
        self.largura * self.comprimento 
        
        // estou medindo o Comprimento com a largura
    } 

    fn tamanho_total(&self) -> i32 {

        let soma:i32 =  self.largura + self.comprimento;
        soma
    }
}


fn main() {

    let valores = Types {largura: 10 , comprimento: 50}; // Vou atribuir os valores pra la 

    println!("Sejam bem vindo ao cli");

    println!("{:x?}", valores.tamanho_total());
}
