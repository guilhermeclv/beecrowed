fn main() {
let gasto_de_combustivel = gasto_de_combustivel(2, 92);
println!("Total de Litros Gastos: {:.3}", gasto_de_combustivel);
}

fn gasto_de_combustivel(tempo:i32, velocidade:i32)-> f32
{
    const KM_POR_LITRO:f32= 12.0;
    let quantidade_de_litros=tempo as f32 * velocidade as f32 / KM_POR_LITRO;
    quantidade_de_litros
}