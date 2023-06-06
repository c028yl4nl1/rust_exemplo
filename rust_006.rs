use std::io;

// vou cria um Struct 

struct Analise {

    number: i128,
    number2: i128,

}

impl Analise{

    fn soma(&self) -> i128 {

        self.number +  self.number2

    }

    fn multiplica(&self) -> i128{

        self.number * self.number2
    }


    fn ver_hex_decimal (&self) -> [String; 1] {
        let value = format!("{} = {:x?} hex , {} = {:x?} hex", self.number , self.number,self.number2 , self.number2);
        
        ///println!("{}", value);

    let array_ = [value];
    
    array_

    }

}


fn main(){

    println!("Ola sejam muito bem vindo ao meu codigo ");

    


    let mut lista_de_numeros: Vec<i128> = Vec::new();
    for c in (1..3){

        println!("Digite o {} numero ", c);

        let _pergunta = pergunta_to_int();

        lista_de_numeros.push(_pergunta);
    }

    let input_struct = Analise{number: lista_de_numeros[0] , number2: lista_de_numeros[1]};


   println!("A soma entre os numeros são = {}\nA multiplicação é = {}\nO Numeros em hexdecimal São {} ",input_struct.soma() , input_struct.multiplica(),input_struct.ver_hex_decimal()[0] );
   // println!("{}", input_struct.ver_hex_decimal()[0]);

    
}



fn pergunta_to_int() -> i128 {
    

    let mut ler_outuput = String::new();
    io::stdin().read_line(&mut ler_outuput).expect("Erro ao ler o cli");
    
    let valor = ler_outuput.trim().parse::<i128>().expect("Erro ao converter A string");

    

    valor
    //ler_outuput
}

