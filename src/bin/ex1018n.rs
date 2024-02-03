fn main() {
let valor_reais =cedulas(358, [70, 33, 5, 2, 1]);
println!("Quantidade Notas de 100: {}\nQuantidades Notas de 50: {}\n Quantidades Notas de 20: {}\n Quantidades Notas de 10: {}\n
Quantidades Notas de 5: {}\n Quantidades Notas de 2: {}\n Quantidades Notas de 1: {}", valor_reais.0, valor_reais.1, valor_reais.2, 
valor_reais.3, valor_reais.4, valor_reais.5, valor_reais.6);
{}
}

fn cedulas(valor_inteiro:i32, face_notas:[i32; 5])-> (i32, i32, i32, i32, i32, i32, i32)
{
    let mut quantidade_por_face:[i32;5] = [0; 5];
    for index in 0..face_notas.len() {
        let n = valor_inteiro/face_notas[index];
        let mut valor_sub = 0;
        for index2 in 0..=index {
            valor_sub=valor_sub+face_notas[index2]*quantidade_por_face[index2];
        }
        let result = (valor_inteiro-valor_sub)/face_notas[index];
        quantidade_por_face[index]=result;
        println!("Resultado:{} -Face: {}", result, face_notas[index]);
    }
    (0, 0, 0, 0, 0, 0, 0)
}