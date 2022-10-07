fn main(){
    //Por default, as variaveis em Rust são imutaveis!

    //Declaracao uma variavel é com let, mesmo que sejam imutaveis por natureza
    let x = 5;

    println!("O valor de x eh {x}");

    //Criando uma variavel mutavel (mut)

    let mut y = 10;

    println!("O valor mutavel de Y agora eh {y}");

    y = 1010;

    println!("O valor mutavel de Y agora eh {y}");

    //Declarando uma constante, que em regra tem letra maiuscula!
    const FIXA: i32 = 10;

    println!("Aqui tenho uma constante de valor {FIXA}");

    //Shadowing: Declarando uma nova variavel com o mesmo nome de uma anteiror.
    let x = "Agora sou uma String!";

    println!("Mudei a variavel X, que era imutavel por shadowing para: {x}");
}