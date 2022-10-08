/*

Rust possui 4 escalares: inteiros, float, booleans e char

I. Inteiros: Podem ser signed ou unsigned (sem possibilidade de ser negativo)
    signed: vai de -2^n-1 até 2^n-1 -1, sendo n o número de bits.
    unsigned: vai até 2^n-1, sendo n o número de bits.
    1. 8bit: i8 para signed e u8 para unsigned.
    2. 16bit: i16 para signed e u16 para unsigned.
    3. 32bit(default): i32 para signed e u32 para unsigned.
    4. 64bit: i64 para signed e u64 para usigned.
    5. 128bit: i128 para signed e u128 para unsigned.
    6. Arch(arquitetura do processador que vai rodar): isize para signed e usize para unsigned.

II. Ponto Flutuante: Possui apenas os tipos de 32 e 64 bits, sendo que são sempre signed.
    1. 32bit: f32 -  Precisão menor, mas ocupa menos espaço com leve ganho de performance.
    2. 64bit(default): f64 -  Precisão maior, com pequeno impacto na performance.

 */

fn main(){

    let _inteiro: i128 = 1010; //Declaração explícita de tipo

    let _resposta: bool = true;

    let _caracter: char = 'L';
    
}