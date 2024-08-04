use std::io;

fn main() {
    let (nota_1, nota_2)=entrada_simples();
    let resultado_media=calculo_media(nota_1, nota_2);
    println!("{:.5}", resultado_media);
}

fn entrada_simples()-> (f32, f32) {
    let entrada_padrao=io::stdin();
    let mut entrada_1= String::new();
    let mut entrada_2= String::new();
    entrada_padrao.read_line(&mut entrada_1).ok();
    entrada_padrao.read_line(&mut entrada_2).ok();
    let nota_1=entrada_1.trim().parse::<f32>().unwrap();
    let nota_2:f32=entrada_2.trim().parse().unwrap();
    (nota_1, nota_2)
}

fn calculo_media(nota_1:f32, nota_2:f32)-> f32 {
    let nota_sem_peso_1= nota_1;
    let nota_sem_peso_2= nota_2;
    let peso_nota_a= 3.5;
    let peso_nota_b= 7.5;
    let nota_a=(nota_sem_peso_1*peso_nota_a) as f32;
    let nota_b=(nota_sem_peso_2*peso_nota_b) as f32;
    let mut media= (nota_a+nota_b)/11.0;
    if media > 10.0 {
        media=10.0
    }
    if media < 0.0 {
        media=0.0
    }
    media
}

#[test]
fn teste_calculo_media() {
    assert_eq!(calculo_media(0.0, 0.0), 0.0);
    assert_eq!(calculo_media(0.0, 7.1), 4.840909);
    assert_eq!(calculo_media(5.0, 7.1), 6.431818);
    assert_eq!(calculo_media(10.0, 10.0), 10.0);
    assert_eq!(calculo_media(-5.0, 7.1), 0.0);
    assert_eq!(calculo_media(-5.0, -7.1), 0.0);
}