const Pula_linha: &str = "\n";

fn main(){

    let guess: &str = "42";
    let binario = 0b0000000000001010;
    let hexdecimal = 0xff;
    let octal = 0o77;
    let decimal = 98_222;
    let bytes = b'A';
    
    println!("String Literal -> {}\nand\nBinario -> {}\nand\nhexdecimal -> {}\nand\noctal {}\nand\nDecimal -> {}\nand\nBytes -> {}\n", guess , binario , hexdecimal,octal, decimal , bytes);

    // Contas matematicas 

    let soma = 5 + 5; // adição 
    let menos = 5 - 5; // Subtração 
    let multiplicar = 10 * 10; // Multiplicação 
    let dividir = 10 / 2; // Divisão
    let resto = 10 % 6; // resto 


    println!("A soma entre 5 + 5 = {}\nA Subtração entre 5 -5 = {} \nA Multiplicação entre 10 * 10 = {} \nA Divisão 10 / 5 = {} \nO resto de 10 % 6 = {}\n", soma, menos ,multiplicar, dividir , resto);


    // tipo boleano 

    let verdadeiro: bool = true;
    let falso: bool = false;  // isso vc aprende nas outras langs facil 
    

    // Strings 

    let nome = "Lanby"; // Uma String literal : &str
    let nome2 = "Lanby".to_string(); //Eu forcei a ser uma String as String em rust vem da memoria {'heap' = valores dinamicos } : String 
    let letra = "L"; // char :  O tipo char representa um valor unicode 


    // tuplas 

    let tupla = (1.3,10,6, "lanby"); // uma tupla normal 

    let (f,i,u,s) = tupla; // estou atribuido um valor para cada posição

    println!("O valor de 'f' é {}\n", f); // neste caso tem que imprimir na tela 1.3 

    // lista ou matriz 

    let friends_on = ["Lanby", "Joao", "W4li", "Decode.exe", "Wolf", "habil", "DarK_M", "MDx", "sacadis", "Kat enjoyer", "L0lgo"]; // matriz seve para valores fixos é diferente de vetor.

    println!("Value pegado na matriz -> {}\n", friends_on[3]);

    // funçoes 

    // Aqui cria uma função , dentro dessa função vc pode add qualquer coisa , vamos usar uma função para fazer os exemplos de função.
    // fn minha_fuction (){}
    
    // vou chamar a função logo abaixo

    exemplo_fuction();


    // Vamos fala de contrele de fluxo 

    // if and else and else if 


    let nome_comun = "lanby";


    if nome_comun == "lanby" { // por tras tem uma codição que resulta em true ou false

        println!("É igual a lanby ");

        // aqui eu posso chamar uma função  e fazer qualquer coisa 
    }

    else if nome_comun == "kaue" {
        println!("O nome é igual  a kaue");
    }

    else if nome_comun == "30" {

        println!("O Numero é 30 i32");


    }
    else{

        println!("Nenhuma opção ");

    }   // fim da condição 



    /*
    
    vamos causar um erro 

    let calculo = 10 + 20;

    if calculo {

        println!(" a soma é {}", calculo);

    } // Ao contrário de linguagens como Ruby e JavaScript, o Rust não tentará automaticamente converter tipos não-booleanos em um booleano. 
    
    erro :: note: expected type `bool`
             found type `{integer}`



     */

    // simples while 

    let mut numero = 3; // tenho que por como mutavel 

    while numero != 0 { // aqui vai fazer a seguinte pergunta: numero é diferente de 0 ? sim , porque 3 não é igual a 0. depois disso vai subtrai 1 ate chega em 0;


        println!("{}!", numero);

        numero = numero - 1;
    } // da frente pra tras.


    // outra condição

    let condicao = true; // bolean 
    let numero = if condicao { // True or false
        5 // return
    } else {
        6 // Return 
    };

    println!("O valor do número é: {}", numero);

    // loop ifinito 

    loop {

        println!("Estou em um Loop ");

        break; // vou usar o break Pra para o loop
        
    }

    // while 

    let mut num = 10;

    while num !=0 {

        println!("\rEstou no laço de repetição {}!", num);
        num = num - 1; // decrementando 

    }
    println!("Fim {}!", num);

    /*
    let a = [10, 20, 30, 40, 50];
    let mut indice = 0;

    while indice < 5 {
        println!("O valor é: {}", a[indice]);

        indice = indice + 1;
    }
    
     */

    // laço for 

    let friends_on = &friends_on; 

    for amigos in friends_on.iter() {

        println!("Meu amigo : {}",amigos);
    }

    // laço for de tras pra frente 

    for c in (1..100).rev(){

        println!("Number: {}", c);
    } // para fazer o loop començando de 1 ate 100 é so tira o .rev();
    
   // ownership // https://rust-br.github.io/rust-book-pt-br/ch04-01-what-is-ownership.html

   let mut filme = String::from("Invasão dos hackers"); // apenas um exemplo

    filme.push_str(",O melhor."); // add na string 

    println!("{}", filme); 

    // resultado : invasão dos hackers , O melhor.


    // fazendo um exemplo de multabilidade com funçoes 

    let mut _mut_nome = String::from("Lanby"); // Vou mudar esse valor 
    modificar(&mut _mut_nome); // vai chamar a função e vai mudar o valor 

    fn modificar(y: &mut String) {

        y.push_str("Mudei o valor heap ");
        
    }
    println!("{}", _mut_nome);
{
    // String Slice

    let nome_s = "Eu Sou you friend ";

    let a = &nome_s[0..3]; // Vai pega da indice 0 ate o 3 então o resultado é [E,u, ,S];

    println!("{}", a);

}



}

    


fn exemplo_fuction(){




    // eu posso criar outra função no lado de fora mas vou cria aqui mesmo

    fn outra_fuction(){

        println!("Outra fuction 1 ");
    }

    // tenho que chamar a função

    outra_fuction();

    println!("Funçaõ  0 ");


    // esse foi um exemplo de função sem args 


    // agora vamos fazer com argumento ok

    fn qual_o_valor(x: i32){

        println!("O valor que tu Passou foi  {} ",x);
    }

    qual_o_valor(10);

    // essa situação vai printar o valor na tela.

    
    // agora vamos criar uma  expressão

    let y = {
        println!("oi"); // criei uma  expressão 

        let x = 10;

        x //   Expressões não terminam com ponto e vírgula. Se você adicionar um ponto e vírgula ao fim de uma expressão, você a transforma em uma declaração, que então não retornará um valor. Tenha isso em mente

    };

    println!("o valor da expressão é {} ", y); // 10 

    /*
    fn cinco() -> i32 {
    
    5
    
    }

    fn main() {
    
    let x = cinco();

    println!("O valor de x é: {}", x);
    
    }

     */



    
}



fn Funcoes_to_rust() {
// vou deixa aqui no fundo funçoes usanda no rust

// convert uma string em uma array de bytes 
let a: String = "".to_string(); // Converter em uma String
let bytes = a.as_bytes();// array de bytes  // Decimal

println!("{:?}", bytes);

// 
}
