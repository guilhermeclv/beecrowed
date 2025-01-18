use std::io;
fn main() {
    let (colunas_por_linha, quantidade_de_itens)=entrada_padrao();
    let texto_sequencia=cria_sequencia_logica(colunas_por_linha, quantidade_de_itens);
    println!("{}", texto_sequencia);
}

fn cria_sequencia_logica(colunas_por_linha:i32, quantidade_de_itens:i32)-> String {
    let mut texto_linha2=String::new();
    for n in 1..=quantidade_de_itens {
         texto_linha2.push_str(&n.to_string());
        
        if n==quantidade_de_itens {
            break;
        }
        if n%colunas_por_linha==0 {
            texto_linha2.push_str("\n");    
        }
        else { 
            texto_linha2.push_str(" ");
        }
    }
    texto_linha2
}

fn entrada_padrao()-> (i32, i32) {
    let entrada_padrao=io::stdin();
    let mut entrada_1=String::new();
    let mut entrada_2=String::new();
    entrada_padrao.read_line(&mut entrada_1).ok();
    entrada_padrao.read_line(&mut entrada_2).ok();
    let numero_inteiro_1= entrada_1.trim().parse::<i32>().unwrap();
    let numero_inteiro_2= entrada_2.trim().parse::<i32>().unwrap();
    (numero_inteiro_1, numero_inteiro_2)
}

#[test]
fn testa_cria_sequencia_logica() {
    assert_eq!(cria_sequencia_logica(3, 6), "1 2 3\n4 5 6");
    assert_eq!(cria_sequencia_logica(3, 3), "1 2 3");
}