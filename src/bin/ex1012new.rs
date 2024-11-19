use std::io;

fn main() {
    let (a, b, c)=entrada_simples();
    let areas = calcular_area(a, b, c);
    println!("Area do Triangulo: {}", areas.0);
    println!("Area do Circulo: {}", areas.1);
    println!("Area do Trapezio: {}", areas.2);
    println!("Area do Quadrado: {}", areas.3);
    println!("Area do Retangulo: {}", areas.4);
}

fn entrada_simples()-> (f64, f64, f64) {
    let entrada_padrao=io::stdin();
    let mut entrada_1=String::new();
    let mut entrada_2=String::new();
    let mut entrada_3=String::new();
    entrada_padrao.read_line(&mut entrada_1).ok();
    entrada_padrao.read_line(&mut entrada_2).ok();
    entrada_padrao.read_line(&mut entrada_3).ok();
    let a= entrada_1.trim().parse::<f64>().unwrap();
    let b= entrada_2.trim().parse::<f64>().unwrap();
    let c= entrada_3.trim().parse::<f64>().unwrap();
    (a, b, c)
}

fn calcular_area(a:f64, b:f64, c:f64)-> (f64, f64, f64, f64, f64) {
   let area_do_triangulo =(a*c)/2.0;
   let area_do_circulo = (c*c) * 3.14159;
   let area_do_trapezio = ((a+b)*c)/2.0;
   let area_do_quadrado = b * b;
   let area_do_retangulo = a * b;
   (area_do_triangulo, area_do_circulo, area_do_trapezio, area_do_quadrado, area_do_retangulo)
}

#[test]
fn testa_calcular_area() {
    assert_eq!(calcular_area(3.0, 4.0, 5.2), (7.800000000000001, 84.94859360000001, 18.2, 16.0, 12.0));
    assert_eq!(calcular_area(12.7, 10.4, 15.2), (96.52, 725.8329536, 175.56, 108.16000000000001, 132.07999999999998));
}