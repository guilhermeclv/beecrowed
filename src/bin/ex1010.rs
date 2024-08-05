use std::io;

fn main() {
    let (valor_2, valor_3, valor_5, valor_6)=entrada_simples();
    let resultado = valor_a_pagar(valor_2, valor_3, valor_5, valor_6);
    println!("VALOR A PAGAR: R$ {:.2}", resultado);
}

fn entrada_simples()-> (i32, f32, i32, f32) {
    let entrada_padrao=io::stdin();
    let mut entrada_1=String::new();
    let mut entrada_2=String::new();
    entrada_padrao.read_line(&mut entrada_1).ok();
    entrada_padrao.read_line(&mut entrada_2).ok();
    let valores_de_entrada_1:Vec<&str>= entrada_1.trim().split(" ").collect();
    let valores_de_entrada_2:Vec<&str>= entrada_2.trim().split(" ").collect();
    let valor_2=valores_de_entrada_1[1].parse::<i32>().unwrap();
    let valor_3=valores_de_entrada_1[2].parse::<f32>().unwrap();
    let valor_5=valores_de_entrada_2[1].parse::<i32>().unwrap();
    let valor_6=valores_de_entrada_2[2].parse::<f32>().unwrap();
    (valor_2, valor_3, valor_5, valor_6)
}

fn valor_a_pagar(valor_2:i32, valor_3:f32, valor_5:i32, valor_6:f32)-> f32 {
    let peca_1=valor_2 as f32 * valor_3;
    let peca_2= valor_5 as f32 * valor_6;
    let resultado = peca_1 + peca_2;
    resultado
}

#[test]
fn testa_valor_a_pagar() {
    assert_eq!(valor_a_pagar(1, 5.30, 2, 5.10), 15.50);
    assert_eq!(valor_a_pagar(2, 15.30, 4, 5.20), 51.40);
    assert_eq!(valor_a_pagar(1, 15.10, 1, 15.10), 30.20);
}