fn main() {
    let mut Vertor : Vec<String> = Vec::new();
    fn alocar(vec:&mut  Vec<String> , num: i32) {
    let value = format!("0xf{}",num);
    vec.push(value.to_string()); // estou passando uma string Slice tenho que convert pra jogar na memoria heap.
        }
    

    let mut num: i32 = 1000000;
    loop {
        //println!("{}",num);
        if num == 0 {
            break
        }
        else {
            alocar(&mut Vertor, num);
         // alocar(Vector);
            num -= 1;
            
        }
        

    }

    println!("{:?}", Vertor)
    
   // println!("Intems no Vector {:?} ", Vertor);

}
