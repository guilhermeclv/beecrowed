use std::io;

fn main() {
    let numero_maximo=entrada_padrao();
    let conjunto_impar = calcular_numero_impar(numero_maximo);
    imprime_impar(conjunto_impar);
}

fn imprime_impar(conjunto_impar:Vec<u32>)-> () {
    for n in conjunto_impar {
        println!("{}", n);
    }
}

fn entrada_padrao()-> u32 {
    let entrada_padrao=io::stdin();
    let mut entrada_1=String::new();
    entrada_padrao.read_line(&mut entrada_1).ok();
    let numero_inteiro=entrada_1.trim().parse::<u32>().unwrap();
    numero_inteiro
}

fn calcular_numero_impar(numero_maximo:u32)-> Vec<u32> {
    let mut conjunto_impar:Vec<u32>= Vec::new();
    let mut numero_maximo_interno=numero_maximo;
    if numero_maximo >= 1000_u32 {
        numero_maximo_interno=1000;
    }
    for i in 0..=numero_maximo_interno {
        if i%2!=0 {
            conjunto_impar.push(i);
        }
    }    
    conjunto_impar
}

#[test]
fn teste_calcular_numero_impar() {
    assert_eq!(calcular_numero_impar(6), vec![1, 3, 5]);
    assert_eq!(calcular_numero_impar(7), vec![1, 3, 5, 7]);
}