use std::io;

fn main() {
    let vetor_de_notas=leitura_de_notas();
    let tupla_notas:(f32, f32, f32, f32)=(vetor_de_notas[0], vetor_de_notas[1], vetor_de_notas[2], vetor_de_notas[3]);
    let nota_media_semestre=calcula_media_semestre(tupla_notas);
    println!("{}", nota_media_semestre);

    if nota_media_semestre<=6.9 && nota_media_semestre>=5.0 {
        let nota_de_recuperacao=leitura_de_notas();
        let media_de_recuperacao=calcula_media_recuperacao(nota_media_semestre, nota_de_recuperacao[0]);
            println!("{}", media_de_recuperacao);
    }

}

fn leitura_de_notas()-> Vec<f32> {

    let entrada_padrao=io::stdin();
    let mut entrada_1=String::new();
    entrada_padrao.read_line(&mut entrada_1).ok();
    let vetor_de_notas_texto:Vec<&str>=entrada_1.trim().split(' ').collect();
    let mut vetor_de_notas:Vec<f32>=Vec::new();

    for i in vetor_de_notas_texto {
        vetor_de_notas.push(i.parse::<f32>().unwrap());
    }
    vetor_de_notas
    
}

fn calcula_media_semestre(conjunto_de_notas:(f32, f32, f32, f32))-> f32 {
    (conjunto_de_notas.0*2.0+conjunto_de_notas.1*3.0+conjunto_de_notas.2*4.0+conjunto_de_notas.3*1.0)/10.0
}

fn calcula_media_recuperacao(nota_semestre:f32, nota_recuperacao:f32)->f32 {
    (nota_semestre+nota_recuperacao)/2.0
}

#[test]
fn teste_calcula_media_semestre() {
    assert_eq!(calcula_media_semestre((2.0, 4.0, 7.5, 8.0)), 5.4);
}
#[test]
fn teste_calcula_media_recuperacao() {
    assert_eq!(calcula_media_recuperacao(5.4, 6.4), 5.9);
}