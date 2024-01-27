fn main() {
let informa_tempo=informa_tempo(140153);
println!("Horas: {:.0} Minutos: {:.0} Segundos: {}", informa_tempo.0, informa_tempo.1, informa_tempo.2);
}
fn informa_tempo(n:i32)-> (i32, i32, i32)
{
    let horas = n / 3600;
    let minutos = (n % 3600) / 60;
    let segundos = (n % 3600) % 60;
    (horas, minutos, segundos)
}
