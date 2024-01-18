fn main() {
let teste_de_selecao=teste_de_selecao(5, 6, 7, 8);
}

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