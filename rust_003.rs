#[warn(unused_assignments)]
#[warn(non_snake_case)]

use std::mem; // men de memoria 

fn main() {
    let mut n: i32 = 10;

    n = 10 * 2 ;

    println!("{}", n);

    let array = [2,3,4,5,6,7];

    let array = array[3];

    println!("Array pegado , {}", array);

    // UMA TUPLA DENTRO DE OUTRA TUPLA 

    let tupla = (1.43,34,3,2,4,"lanby");

    let outra_tupla = ("caio","cozmo","Joao",tupla);

    println!("Tupla com outra tupla dentro : {:?} " , outra_tupla); // Tenho que mostra em modo de debug


    println!("{:?}", tupla); /* 
     com base nos meus estudos se eu chama a tupla ela não vai existi , caso ele existi siginifca que a memoria heap é apenas para valores dinamicos como String 
    */
    

    let array_ver: [i32; 5] = [0,2,3,4,5]; // array de 5 elementos que cada elemento é i32 
    println!("O Valor na idice 2 é {} O tamanho da Array é de {} e O tamanho de Bytes na Array é {}", array_ver[2], array_ver.len(), mem::size_of_val(&array_ver)); // Tenho que passar como referecia 

}
