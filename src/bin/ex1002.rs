use std::io;

fn main() {
    let elevar_ao_quadrado=elevar_ao_quadrado();
    let n = 3.14159;
    let resultado = elevar_ao_quadrado as f32 *n;
    println!("A={:.4}", resultado);
}

fn elevar_ao_quadrado()->i32 {
    let entrada_padrao=io::stdin();
    let mut numero=String::new();
    entrada_padrao.read_line(&mut numero).ok();
    let numero_str = numero.trim().parse::<i32>().unwrap();
    let resultado =numero_str*numero_str;
    // println!("Numero STR: {:?}", numero_str);
    // println!("Numero Resultado: {:?}", resultado);
    resultado
}

#[test]
fn teste_eleva_ao_quadrado(){
    assert_eq!(elevar_ao_quadrado(), 4);
    //4 usado como referência, lembrar que o valor do asserteq vai ser igual ao imput ao quadrado
}