struct Persona {
    id: i64
    }

fn main(){
    let arr0: [Persona; 1] = [Persona{ id: 0 }];
    let arr1: [&str; 2] = ["Hola", "Mundo"];
    let arr2: [String; 2] = ["Hola".to_string(), "Mundo".to_string()];
    println!("{}", arr0[0].id);
    // imprime 0
    // arreglo de 3 dimensiones
    let mut arr3: [[[i64; 4];2]; 2] = [
    [ [ 1, 3, 5, 7], [ 9, 11, 13, 15] ],
    [ [ 2, 4, 6, 8], [10, 12, 14, 16] ]
    ];
    arr3[0][1][3] = 50;
    println!("{:?}", arr1);
    println!("{:?}", arr2);
    println!("{:?}", arr3[0][1]);
    // imprime ["Hola", "Mundo"]
    // imprime ["Hola", "Mundo"]
    // imprime [9, 11, 13, 50]
}
