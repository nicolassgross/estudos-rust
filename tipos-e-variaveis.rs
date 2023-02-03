fn main() {
    
    //4 bytes
    let variavel:i32 = 300;

    println!("variavel = {}, tamanho = {} bytes", variavel, std::mem::size_of_val(&variavel));

    //4 bytes
    let decimal:f32 = 2.5;

    println!("decimal = {}, tamanho = {} bytes", decimal, std::mem::size_of_val(&decimal));
    
    //1 bytes
    let booleana:bool = true;

    println!("Booleana = {}, Tamanho booleana = {}", booleana, std::mem::size_of_val(&booleana));

    //4 bytes
    let letra:char = 'N';

    println!("Tamanho do char = {}, Tamanho letra = {}", letra, std::mem::size_of_val(&letra));


}