fn main() {
    let positivos_e_media=positivos_e_media([7.0, -5.0, 6.0, -3.4, 4.6, 12.0]);
    println!("EX1064: {}", positivos_e_media.0);
    println!("EX1064: {}", positivos_e_media.1);
}
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
