use::std::io;

fn main() {
    let entrada_padrao=io::stdin();
    let mut linha_1=String::new();
    let mut linha_2=String::new();
    entrada_padrao.read_line(&mut linha_1).ok();
    entrada_padrao.read_line(&mut linha_2).ok();
    // println!("Linha 1: {:?}", linha_1.trim());
    // println!("Linha 2: {:?}", linha_2);
    let a = linha_1.trim().parse::<i32>().unwrap();
    let b:i32 = linha_2.trim().parse().unwrap();
    let x =a + b;
    println!("X = {}", x);
}