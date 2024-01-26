fn main() {
let informa_tempo=informa_tempo(140153);
println!("Horas: {:.0} Minutos: {:.0} Segundos: {}", informa_tempo.0, informa_tempo.1, informa_tempo.2);
}
fn informa_tempo(n:i32)-> (f32, f32, f32)
{
    let mut horas = n as f32 / 3600.0;
    let mut minutos = (n as f32 / 60.0) % 60.0;
    if minutos > 60.0 { 
        minutos= minutos % 60.0;
        horas = horas + (minutos /60.0);
    }
    let segundos = n as f32 % 60.0;
    (horas, minutos, segundos)
}
