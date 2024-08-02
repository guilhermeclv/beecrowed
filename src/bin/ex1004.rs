use std::io;

fn main() {
    let (numero_1, numero_2) = entrada_simples();
    let produto=produto_simples(numero_1, numero_2);
    println!("PROD = {}", produto);
}

fn entrada_simples()-> (i32, i32) {
    let entrada_padrao=io::stdin();
    let mut entrada_1= String::new();
    let mut entrada_2=String::new();
    entrada_padrao.read_line(& mut entrada_1).ok();
    entrada_padrao.read_line(& mut entrada_2).ok();
    let numero_1 = entrada_1.trim().parse::<i32>().unwrap();
    let numero_2:i32=entrada_2.trim().parse().unwrap();
    (numero_1, numero_2)
}

fn produto_simples(numero_1:i32, numero_2:i32)-> i32 {
    let produto = numero_1*numero_2;
    produto
}

#[test]
fn teste_produto_simples() {
    assert_eq!(produto_simples(2, 3), 6);
    assert_eq!(produto_simples(-2, 3), -6);
    assert_eq!(produto_simples(0, 0), 0);
}