fn main() {
let valor_reais =cedulas(11257);
println!("Quantidade Notas de 100: {} Quantidades Notas de 50: {} Quantidades Notas de 20: {} Quantidades Notas de 10: {}
Quantidades Notas de 5: {} Quantidades Notas de 2: {} Quantidades Notas de 1: {}", valor_reais.0, valor_reais.1, valor_reais.2, 
valor_reais.3, valor_reais.4, valor_reais.5, valor_reais.6);
}

fn cedulas(valor_inteiro:i32)-> (i32, i32, i32, i32, i32, i32, i32)
{
    let notas_100 = valor_inteiro / 100;
    let resto_notas_100 = valor_inteiro % 100;

    let notas_50 = resto_notas_100 / 50;
    let resto_notas_50 = resto_notas_100 % 50;

    let notas_20 = resto_notas_50 / 20;
    let resto_notas_20 = resto_notas_50 % 20;

    let notas_10 = resto_notas_20 / 10;
    let resto_notas_10 = resto_notas_20 % 10;

    let notas_5 = resto_notas_10 / 5;
    let resto_notas_5 = resto_notas_10 % 5;

    let notas_2 = resto_notas_5 / 2;
    let resto_notas_2 = resto_notas_5 % 2;

    let notas_1 = resto_notas_2 / 1;

    (notas_100, notas_50, notas_20, notas_10, notas_5, notas_2, notas_1)
}