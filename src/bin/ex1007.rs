use std::io;

fn main() {
    let (a, b, c, d)=entrada_simples();
    let resultado =calcular_diferenca(a, b, c, d);
    println!("DIFERENCA = {}", resultado);
}

fn entrada_simples()-> (i32, i32, i32, i32) {
    let entrada_padrao=io::stdin();
    let mut entrada_a=String::new();
    let mut entrada_b= String::new();
    let mut entrada_c= String::new();
    let mut entrada_d=String::new();
    entrada_padrao.read_line(&mut entrada_a).ok();
    entrada_padrao.read_line(&mut entrada_b).ok();
    entrada_padrao.read_line(&mut entrada_c).ok();
    entrada_padrao.read_line(&mut entrada_d).ok();
    let a=entrada_a.trim().parse::<i32>().unwrap();
    let b=entrada_b.trim().parse::<i32>().unwrap();
    let c=entrada_c.trim().parse::<i32>().unwrap();
    let d=entrada_d.trim().parse::<i32>().unwrap();
    (a, b, c, d)
}

fn calcular_diferenca(a:i32, b:i32, c:i32, d:i32)-> i32 {
    let produto_1= a*b;
    let produto_2= c*d;
    let resultado= produto_1-produto_2;
    resultado
}

#[test]
fn teste_calcular_diferenca() {
    assert_eq!(calcular_diferenca(5, 6, 7, 8), -26);
    assert_eq!(calcular_diferenca(0, 0, 7, 8), -56);
    assert_eq!(calcular_diferenca(5, 6, -7, 8), 86);
}