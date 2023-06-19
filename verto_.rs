#[derive(Debug)]
struct User{

    nome: String , idade: i32 , id:i32 ,Banco: Vec<i32>, 

}

// implemetation keyword
impl User {

    // Implementa a Struct 

    fn print_nome(&self){
        println!("Nome:{}", self.nome);
    }
    fn ver_usuario_banco_de_dados(&self){
        let mut array_fake = [21212,1212,2323,442, 12321];
        if array_fake.contains(&self.id){
            println!(" O id Está cadastrado ");
        }

        else {

            println!( "O id não esta cadastrado ");
        }
    }

}
fn main() {

    let pagar = User{nome:String::from(" D3c0d3.ex3") , idade: 10 as i32 , id: 12212 as i32 , Banco: vec![12121,12121,21212,212,212]} ;
    println!("{:#?}", pagar);

    pagar.ver_usuario_banco_de_dados(); 

    let mut Verto2: Vec<i32> = Vec::new();


    Verto2.push(323);


    println!(" Vertor 2 {:#?}", Verto2);
     // ver todos os ites do Vertor

    for i in Verto2.iter() {

        println!("{}",i);
    }


}
