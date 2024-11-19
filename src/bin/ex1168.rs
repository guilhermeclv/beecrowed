use std::io;

fn main() {
    let numero_primario=entrada_dinamica();

    for numero in numero_primario {
        let numero_de_leds = calcular_leds(&numero);
        println!("{} leds", numero_de_leds);
    }
}

fn entrada_dinamica()-> Vec<String> {
    let entrada_padrao=io::stdin();
    let mut entrada_1=String::new();
    entrada_padrao.read_line(&mut entrada_1).ok();
    let numero_inteiro=entrada_1.trim().parse::<i32>().unwrap();
    
    let mut entradas: Vec<String> = Vec::new();

    for i in 0..numero_inteiro {
    let mut entrada_2=String::new();
    entrada_padrao.read_line(&mut entrada_2).ok();
    entradas.push(entrada_2.trim().to_string());
    }
    entradas
}

fn calcular_leds(numero_primario: &str)-> i32 {
    let mut numero_de_leds=0;

   for i in numero_primario.to_string().chars() {
   let contador = match i.to_digit(10).unwrap() {
        0 => 6,
        1 => 2,
        2 => 5,
        3 => 5,
        4 => 4,
        5 => 5,
        6 => 6,
        7 => 3,
        8 => 7,
        9 => 6,
        _=> 0,
    };
    numero_de_leds =numero_de_leds + contador;
        }
    numero_de_leds
}

#[test]
fn testa_calcular_leds() {
    assert_eq!(calcular_leds("115380"), 27);
    assert_eq!(calcular_leds("2819311"), 29);
    assert_eq!(calcular_leds("23456"), 25);
}