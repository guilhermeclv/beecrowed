use std::io;

fn main() {
    let numero = entrada_simples();
    let resultado = area_do_circulo(numero);
    println!("A={:.4}", resultado);
}

fn entrada_simples()-> i32 {
    let entrada_padrao=io::stdin();
    let mut numero_str=String::new();
    entrada_padrao.read_line(&mut numero_str).ok();
    let numero = numero_str.trim().parse::<i32>().unwrap();
    numero
}

fn area_do_circulo(numero: i32)-> f32 {
    let calculo = numero * numero;
    let n = 3.14159;
    let resultado = calculo as f32 * n;
    resultado as f32
}

#[test]
fn teste_area_do_circulo() {
    assert_eq!(area_do_circulo(0), 0.0);
    assert_eq!(area_do_circulo(2), 12.56636);
    assert_eq!(area_do_circulo(3), 28.274311);
}