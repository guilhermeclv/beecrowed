fn main(){
    println!("teste");
    // test 1 
    let resposta = ex1192(4, 'c', 3);
    println!("resposta 1: {resposta}");
    // test 2 
    let resposta = ex1192(3, 'A', 3);
    println!("resposta 2: {resposta}");
    // test 3 
    let resposta = ex1192(4, 'a', 3);
    println!("resposta 3: {resposta}");

}
fn ex1192(number_1:i32,letter:char, number_2:i32)->i32{

    if number_1 == number_2 {
        return number_1*number_2;
    }
    // else if (letter as u8)>97 {
    //     return number_1+number_2;
    // }
    // number_1-number_2
    
    // match letter as u8 {
    //     65..=90=>number_1-number_2,
    //     _ => number_1+number_2 
    // }

    match letter.is_lowercase() {
        false=>number_1-number_2,
        _ => number_1+number_2 
    }
}