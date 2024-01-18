fn main() {
    println!("Hello, world!");
    let salario_funcionario=salario_funcionario(10, 10, 5.0);
    println!("EX1008: {}", salario_funcionario);
    let calcula_tempo_1016=calcula_tempo_1016(30);
    println!("EX1016: {}", calcula_tempo_1016);
    let produto_simples=produto_simples(10, 3);
    println!("EX1004: {}", produto_simples);
    let ponto_flutuante=ponto_flutuante(3.0, 4.0, 5.2);
    println!("Area do Triangulo: {}", ponto_flutuante.0);
    println!("Area do Circulo: {}", ponto_flutuante.1);
    println!("Area do Trapezio: {}", ponto_flutuante.2);
    println!("Area do Quadrado: {:.2}", ponto_flutuante.3);
    println!("Area do Retangulo: {:.2}", ponto_flutuante.4);
    let teste_de_selecao=teste_de_selecao(5, 6, 7, 8);
    let diferenca=diferenca(5, 6, 7, 8);
    println!("EX1007: {}", diferenca);
    let positivos_e_media=positivos_e_media([7.0, -5.0, 6.0, -3.4, 4.6, 12.0]);
    println!("EX1064: {}", positivos_e_media.0);
    println!("EX1064: {}", positivos_e_media.1);
}
//1008
fn salario_funcionario(registro:i32, quantidade_horas:i32, valor_da_hora:f32) -> f32
{
  println!("Number: {}", registro);
  println!("Salary: {}", quantidade_horas as f32 *valor_da_hora);
    quantidade_horas as f32 *valor_da_hora
}
//1016
fn calcula_tempo_1016(kilometros:i32)->i32
{
    kilometros*2
}

// Fazer 10 exercicios
// Como faria o problema da MODA de um array

//1004
fn produto_simples(variavel_1:i32, variavel_2:i32)->i32
{
    variavel_1*variavel_2
}
//1012
fn ponto_flutuante(variavel_a:f32, variavel_b:f32, variavel_c:f32)-> (f32, f32, f32, f32, f32)
{
   let area_do_triangulo = (variavel_a*variavel_c)/2.0;
   let area_do_circulo = variavel_c * variavel_c * 3.14159;
   let area_do_trapezio = ((variavel_a+variavel_b)*variavel_c)/2.0;
   let area_do_quadrado = variavel_b*variavel_b;
   let area_do_retangulo = variavel_a*variavel_b;
   (area_do_triangulo, area_do_circulo, area_do_trapezio, area_do_quadrado, area_do_retangulo)
  
}
//1035
fn teste_de_selecao(a:i32, b:i32, c:i32, d:i32)
{
    if b > c && d > a {
        if  c+ d> a + b {
            if c >0 && d > 0 {
                if a % 2==0 {
                    println!("Valores Aceitos");
                    return;
                }
            }
        }
    }
    else {
        println!("Valores não aceitos");
    };
    if (b > c && d > a) && (c+ d> a + b) && (c >0 && d > 0) && (a % 2==0) {
        println!("Valores Aceitos");
    }
    else {
        println!("Valores não aceitos");
    };
}
//1007
fn diferenca(a:i32, b:i32, c:i32, d:i32)->i32
{
    (a*b)-(c*d)
}
//1064
fn positivos_e_media(numeros:[f32; 6])-> (i32, f32)
{
    let mut quantidade_de_positivos = 0;
    let mut media_dos_positivos = 0_f32;
    //(1+2+3)/3 =1/3 + 2/3 + 3/3
    // [1, 2, 2, 2, 3, 3]
    for numero in numeros {
        if  numero>0.0 {
            quantidade_de_positivos=quantidade_de_positivos+1
        }
        media_dos_positivos=media_dos_positivos+numero/6.0
    }
    (quantidade_de_positivos, media_dos_positivos)

}
