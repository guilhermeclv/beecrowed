fn main() {
    let ponto_flutuante=ponto_flutuante(3.0, 4.0, 5.2);
    println!("Area do Triangulo: {}", ponto_flutuante.0);
    println!("Area do Circulo: {}", ponto_flutuante.1);
    println!("Area do Trapezio: {}", ponto_flutuante.2);
    println!("Area do Quadrado: {:.2}", ponto_flutuante.3);
    println!("Area do Retangulo: {:.2}", ponto_flutuante.4);
}
fn ponto_flutuante(variavel_a:f32, variavel_b:f32, variavel_c:f32)-> (f32, f32, f32, f32, f32)
{
   let area_do_triangulo = (variavel_a*variavel_c)/2.0;
   let area_do_circulo = variavel_c * variavel_c * 3.14159;
   let area_do_trapezio = ((variavel_a+variavel_b)*variavel_c)/2.0;
   let area_do_quadrado = variavel_b*variavel_b;
   let area_do_retangulo = variavel_a*variavel_b;
   (area_do_triangulo, area_do_circulo, area_do_trapezio, area_do_quadrado, area_do_retangulo)
  
}