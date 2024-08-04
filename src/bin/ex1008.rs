use std::io;

fn main() {
    let (number, hours, value)= entrada_simples();
    println!("NUMBER = {}", number);
    let salary= calcular_salario(hours, value);
    println!("SALARY = U$ {:.2}", salary);
}

fn entrada_simples()-> (i32, i32, f32) {
    let entrada_padrao=io::stdin();
    let mut entrada_1=String::new();
    let mut entrada_2=String::new();
    let mut entrada_3= String::new();
    entrada_padrao.read_line(&mut entrada_1).ok();
    entrada_padrao.read_line(&mut entrada_2).ok();
    entrada_padrao.read_line(&mut entrada_3).ok();
    let number:i32 = entrada_1.trim().parse().unwrap();
    let hours:i32 = entrada_2.trim().parse().unwrap();
    let value:f32 = entrada_3.trim().parse().unwrap();
    (number, hours, value)
}

fn calcular_salario(hours:i32, value:f32)-> f32 {
    let calculo_salario = hours as f32 *value;
    calculo_salario
}

#[test]
fn teste_calcular_salario() {
    assert_eq!(calcular_salario(100, 5.5), 550.00);
    assert_eq!(calcular_salario(200, 20.5), 4100.00);
    assert_eq!(calcular_salario(145, 15.55), 2254.75);
}