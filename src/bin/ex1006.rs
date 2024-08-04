use std::io;

fn main(){
    let (nota_sem_peso_1, nota_sem_peso_2, nota_sem_peso_3) = entrada_simples();
    let media = calculo_media(nota_sem_peso_1, nota_sem_peso_2, nota_sem_peso_3);
    println!("MEDIA = {}", media);
}

fn entrada_simples()-> (f32, f32, f32) {
    let entrada_padrao=io::stdin();
    let mut entrada_nota_1=String::new();
    let mut entrada_nota_2=String::new();
    let mut entrada_nota_3=String::new();
    entrada_padrao.read_line(& mut entrada_nota_1).ok();
    entrada_padrao.read_line(&mut entrada_nota_2).ok();
    entrada_padrao.read_line(&mut entrada_nota_3).ok();
    let nota_sem_peso_1:f32= entrada_nota_1.trim().parse().unwrap();
    let nota_sem_peso_2:f32= entrada_nota_2.trim().parse().unwrap();
    let nota_sem_peso_3:f32= entrada_nota_3.trim().parse().unwrap();
    (nota_sem_peso_1, nota_sem_peso_2, nota_sem_peso_3)
}
// Porque preciso colocar o tipo da variável na entrada da função? O sistema não deveria reconhecer automaticamente
// uma vez que a main já entende que está como f32?
fn calculo_media(nota_sem_peso_1:f32, nota_sem_peso_2:f32, nota_sem_peso_3:f32)-> f32 {
    let nota_a=nota_sem_peso_1*2.0;
    let nota_b=nota_sem_peso_2*3.0;
    let nota_c= nota_sem_peso_3*5.0;
    let nota_total= nota_a+nota_b+nota_c;
    let mut media = nota_total/10.0;
    if media > 10.0 {
        media =10.0
    }
    if media < 0.0 {
        media =0.0
    }
    media
}

#[test]
fn testa_calculo_media(){
    assert_eq!(calculo_media(5.0, 6.0, 7.0), 6.3);
    assert_eq!(calculo_media(5.0, 10.0, 10.0), 9.0);
    assert_eq!(calculo_media(10.0, 10.0, 5.0), 7.5);
    assert_eq!(calculo_media(10.0, 10.0, 10.0), 10.0);
    assert_eq!(calculo_media(0.0, 0.0, 0.0), 0.0);
}