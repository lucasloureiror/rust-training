/*

                                    COMPOUNDS
    1.Conceito: Agrupam variáveis de tipos diferentes na mesma estrutura.
    2.Tuples: Tipo com tamanho fixo que agrupa múltiplas variáveis em uma única estrutura.
    3. Arrays: Uma coleção de elementos do mesmo tipo com tamanho fixo.

*/

fn main(){

    //Tuple
    let tup: (i32, f64, u8) = (500,10.10,10); 

    let (x,y,z) = tup; //Desmembrando tup em 3 elementos diferentes.

    println!("{y}");

    let segundaPosicao = tup.1; //Acessando a segunda posição da estrutura tup

    println!("{segundaPosicao}");



    //Array

    let vetor = [1,2,3,4,5,6,7,8,9,10];

    let vetorTipado: [i32; 2] = [1,2];

    println!("Acessando a posição 0 do vetor {}", vetor[0]);



}