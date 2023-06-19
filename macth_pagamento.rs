enum Pagamento {

    dinheiro , credito , boleto , Id,

}

fn main(){

    println!("enums> ");

    let pessoa_paga = Pagamento::Id;
    match pessoa_paga {

        Pagamento::dinheiro => println!(" O cara pagou em dinheiro "),
        Pagamento::credito => println!(" O cara pagou no credito "),
        Pagamento::boleto => println!(" O cara pagou no boleto "),
        _ => {println!(" A função è invalida ");}



    }



}
