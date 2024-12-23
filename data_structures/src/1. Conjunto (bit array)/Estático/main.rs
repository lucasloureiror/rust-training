use std::io;
fn main(){

    let mut vetor: [usize; 100] = [0;100];

    loop{
        println!("Digite o numero que deseja guardar na bit array ou digite Q para sair");
        let mut input = String:: new();

        io::stdin()
            .read_line(&mut input)
            .expect("Falha na leitura");

            match input == "Q"{
                break;

            }
               let input: usize = input.trim().parse().expect("Nao digitou um numero!");
               vetor[input - 1] = 1;

            }
    }
    
    
    