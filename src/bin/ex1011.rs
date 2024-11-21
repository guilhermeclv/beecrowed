use std::io;

fn main() {
    let raio = entrada_simples();
    let volume=calcular_volume_da_esfera(raio);
    println!("VOLUME = {}", volume);
}

fn entrada_simples()->f64 {
    let entrada_padrao=io::stdin();
    let mut entrada_1=String::new();
    entrada_padrao.read_line(&mut entrada_1).ok();
    let raio= entrada_1.trim().parse::<f64>().unwrap();
    raio
}

fn calcular_volume_da_esfera(raio:f64)->f64 {
    let pi=3.14159;
    let volume_da_esfera = (4.0 / 3.0) * pi * raio * raio * raio;
    volume_da_esfera as f64
}

#[test]
fn testa_calcular_volume_da_esfera() {
    assert_eq!(calcular_volume_da_esfera(3.0), 113.09723999999999);
    assert_eq!(calcular_volume_da_esfera(15.0), 14137.154999999999);
    assert_eq!(calcular_volume_da_esfera(1523.0), 14797486501.627373);
    assert!(calcular_volume_da_esfera(1523.0) <= (14797486501.62 + 0.01) || calcular_volume_da_esfera(1523.0) >= (14797486501.62-0.01));
    assert!((calcular_volume_da_esfera(1523.0) - 14797486501.62).abs()<= 0.01);
}