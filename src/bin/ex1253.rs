use std::io;

fn main() {
    let entradas = entrada_dinamica();
    for (string, variacao) in entradas {
        let texto_decodificado = decodificar_cifra_de_cesar(string, variacao);
        println!("{}", texto_decodificado);
    }

}
fn entrada_dinamica()-> Vec<(String, u32)> {
    let entrada_padrao=io::stdin();
    let mut entrada_1=String::new();
    entrada_padrao.read_line(&mut entrada_1).ok();
    let numero_inteiro=entrada_1.trim().parse::<i32>().unwrap();
    let mut entradas: Vec<(String, u32)> = Vec::new();
    for i in 0..numero_inteiro {
        let mut entrada_2=String::new();
        let mut entrada_3=String::new();
        entrada_padrao.read_line(&mut entrada_2).ok();
        entrada_padrao.read_line(&mut entrada_3).ok();
        let string=entrada_2.trim().to_string();
        let variacao=entrada_3.trim().parse::<u32>().unwrap();
        entradas.push((string, variacao));
    }
    entradas
}

fn decodificar_cifra_de_cesar(string:String, variacao:u32)-> String {
    let mut texto_decodificado= String::new();

    for n in string.chars() {
        let posicao_original = n as u8;
        let mut nova_posicao = posicao_original;
       
        if posicao_original >= ('A' as u8) && posicao_original <= ('Z' as u8) {
            nova_posicao = posicao_original - variacao as u8;

            if nova_posicao < ('A' as u8)  {
                nova_posicao = nova_posicao +26;
            }
       }
        
        texto_decodificado.push(nova_posicao as char);
    }
    texto_decodificado
}
#[test]
fn testa_decodificar_cifra_de_cesar() {
    assert_eq!(decodificar_cifra_de_cesar("VQREQFGT".to_string(), 2), "TOPCODER");
    assert_eq!(decodificar_cifra_de_cesar("ABCDEFGHIJKLMNOPQRSTUVWXYZ".to_string(), 10), "QRSTUVWXYZABCDEFGHIJKLMNOP");
}
