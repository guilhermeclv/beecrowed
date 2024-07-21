use std::io;
fn main() {
    let vetor_lido=ler_vetor();
    let maior_valor=retornar_maior_valor(vetor_lido);
    println!("{} eh o maior", maior_valor);
}

fn ler_vetor()-> Vec<i32> {
    let entrada_padrao =io::stdin();
    let mut linha_1= String::new();
    entrada_padrao.read_line(&mut linha_1).ok();
    let vetor_str:Vec<&str>= linha_1.trim().split(' ').collect();
    let mut vetor_num:Vec<i32>=Vec::new();
    for numero_str in vetor_str {
        vetor_num.push(numero_str.parse::<i32>().unwrap());
    }
    vetor_num
}

fn retornar_maior_valor(numeros:Vec<i32>)-> i32 {
    let mut maior_valor=*numeros.get(0).unwrap_or(&0) as i32;
    for valor_atual in numeros {
        if maior_valor < valor_atual {
            maior_valor=valor_atual;
        }
    }
    maior_valor
}

#[test]
fn teste_retornar_maior_valor() {
    assert_eq!(retornar_maior_valor(vec!(1, 2, 3)), 3);
    assert_eq!(retornar_maior_valor(vec!(7, 5, 2, 1, -2)), 7);
}

#[test]
fn teste_retornar_maior_valor_nulo() {
    assert_eq!(retornar_maior_valor(vec!()), 0);
}