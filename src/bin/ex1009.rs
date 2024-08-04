use std::io;

fn main() {
    let (_nome, salario_fixo, comissao)= entrada_simples();
    let salario= calcular_salario(salario_fixo, comissao);
    println!("TOTAL = R${}", salario);
}

fn entrada_simples()-> (String, f64, f64) {
    let entrada_padrao=io::stdin();
    let mut entrada_1=String::new();
    let mut entrada_2=String::new();
    let mut entrada_3= String::new();
    entrada_padrao.read_line(&mut entrada_1).ok();
    entrada_padrao.read_line(&mut entrada_2).ok();
    entrada_padrao.read_line(&mut entrada_3).ok();
    let nome:String = entrada_1.trim().to_string();
    let salario_fixo:f64 = entrada_2.trim().parse().unwrap();
    let comissao:f64 = entrada_3.trim().parse().unwrap();
    (nome, salario_fixo, comissao)
}

fn calcular_salario(salario_fixo:f64, comissao:f64)-> f64 {
    let calculo_salario= salario_fixo + (comissao * 0.15);
    calculo_salario
}

#[test]
    fn teste_calcular_salario() {
        assert_eq!(calcular_salario(500.00, 1230.30), 684.545);
        assert_eq!(calcular_salario(700.00, 0.00), 700.00);
        assert_eq!(calcular_salario(1700.00, 1230.50), 1884.575)
}