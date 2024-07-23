use std::io;

fn main() {
    let resultado =somar_simples();
    println!("SOMA = {}", resultado);
}

fn somar_simples()-> i32 {
    let entrada_padrao=io::stdin();
    let mut entrada_1=String::new();
    let mut entrada_2=String::new();
    entrada_padrao.read_line(&mut entrada_1).ok();
    entrada_padrao.read_line(&mut entrada_2).ok();
    let numero_1 = entrada_1.trim().parse::<i32>().unwrap();
    let numero_2 = entrada_2.trim().parse::<i32>().unwrap();
    let soma = numero_1 + numero_2;
    soma
}

#[test]
fn testar_soma_simples() {
    assert_eq!(somar_simples(), 4)
}