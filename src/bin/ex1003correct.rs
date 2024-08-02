use std::io;

fn main() {
    let (numero_1, numero_2) = entrada_simples();
    let soma = soma_simples(numero_1, numero_2);
    println!("SOMA = {}", soma);
}

fn soma_simples(numero_1:i32, numero_2:i32)-> i32 {
    let soma = numero_1 + numero_2;
    soma
}

fn entrada_simples()-> (i32, i32) {
    let entrada_padrao=io::stdin();
    let mut entrada_1=String::new();
    let mut entrada_2=String::new();
    entrada_padrao.read_line(&mut entrada_1).ok();
    entrada_padrao.read_line(&mut entrada_2).ok();
    let numero_1 = entrada_1.trim().parse::<i32>().unwrap();
    let numero_2 = entrada_2.trim().parse::<i32>().unwrap();
    (numero_1, numero_2)
}

#[test]
fn testar_soma_simples() {
    assert_eq!(soma_simples(8, 12), 20);
    assert_eq!(soma_simples(-8, 12), 4);
    //assert_eq!(soma_simples(8.5, 12.5), 21.0);
}