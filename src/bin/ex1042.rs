fn main() {

}

fn ordena_numeros(numeros:Vec<i32>)-> Vec<i32> {

let mut saida=numeros.clone();

if numeros[0]>numeros[1] {
    if numeros[0]>numeros[2] {
        saida[0]=numeros[0];
        if numeros[1]>numeros[2] {
            saida[1]=numeros[1];
            saida[2]=numeros[2];
        }
        else { 
            saida[2]=numeros[1];
            saida[1]=numeros[2];
        }
    }
    else {
        saida[0]=numeros[2];
        saida[1]=numeros[0];
        saida[2]=numeros[1];
    }
}
else { 
    if numeros[1]>numeros[2] {
        saida[0]=numeros[1];
        if numeros[0]>numeros[2] {
            saida[1]=numeros[0];
            saida[2]=numeros[2];
        }
        else { 
            saida[2]=numeros[0];
            saida[1]=numeros[2];
        }
    }
    else {
        saida[0]=numeros[2];
        saida[1]=numeros[1];
        saida[2]=numeros[0];
    }

}
saida
}

fn bubble_sort(numeros_recebidos:Vec<i32>)-> Vec<i32> {
    let mut numeros=numeros_recebidos.clone();

    let quantidade_itens=numeros.len();

    for indice_ex in 0..quantidade_itens-1 {

        let mut trocou=false;
        
        for indice in 0..quantidade_itens-1-indice_ex {
            if numeros[indice] < numeros[indice+1] {
                let valor_temporario=numeros[indice];
                numeros[indice] = numeros[indice+1];
                numeros[indice+1] = valor_temporario;
                trocou=true;
            }
        
        }
        if trocou == true {
            break
        }
    }
    numeros
    
}

#[test]
fn teste_ordena_numeros() {
    assert_eq!(ordena_numeros(vec![1, 2, 3]), vec![3, 2, 1]);
    assert_eq!(ordena_numeros(vec![50, 2, 60]), vec![60, 50, 2]);
}
#[test]
fn teste_ordena_numeros_avancados() {
    assert_eq!(bubble_sort(vec![1, 2, 3, 4, 5, 6]), vec![6, 5, 4, 3, 2, 1]);
}