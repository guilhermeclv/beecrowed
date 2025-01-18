use std::io;
fn main() {
    let sentenca=entrada();
    println!("{:?}", sentenca);
    let texto_pronto=converter_sentenca(&sentenca);
    println!("{:?}", texto_pronto);
}
fn converter_sentenca(sentenca:&str)-> String {
    let mut texto_pronto=String::new();

    for i in sentenca.chars() {
        let texto_original = i as u8;
        let mut texto_modificado = texto_original;

        if texto_original >= 65 && texto_original <=90 {
            texto_modificado = texto_original +32;
        }

        if texto_original >= 97 && texto_original <=122 {
                texto_modificado = texto_original -32;
            }
    
        else {texto_modificado = texto_original;
            }

    texto_pronto.push(texto_modificado as char);
     
    }
    texto_pronto.to_string()        
}

fn entrada()->String {
    let entrada=io::stdin();
    let mut entrada_1=String::new();
    entrada.read_line(&mut entrada_1).ok();
    let texto_sentenca=entrada_1.trim().to_string();
    texto_sentenca
}

#[test]
fn teste_converter_senteca() {
    assert_eq!(converter_sentenca("This is a dancing sentence"), "ThIs Is A dAnCiNg SeNtEnCe");
    assert_eq!(converter_sentenca("This   is         a  dancing   sentence"), "  ThIs   Is         A  dAnCiNg   SeNtEnCe  ");
}