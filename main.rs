// variaveis tem que ser definidas com valores antes de serem usadas
fn bin_muta(){
    let x: i32 = 5; // initialized variable 
    //let y: i32; uninitialized variable
    let mut z: i32 = 4; //mutable variable
    
    z = z + 1;

    assert_eq!(x, 5);
    assert_eq!(z, 5);
    println!("success!");
}


// escopo são blocos de codigo dentro de funcoes que servem para executar um contexto de codigo
fn scope(){
    let x: i32 = 10;
    let y: i32 = 5;
    
    {
        println!("The value of x is {} and value of y is {}", x, y);
    }
}


//Fix the error with the use of define_x
fn define_x() {
    // let x = "hello"; error
    
    let x: &str = "hello";
    println!("{}, world!", x);
}



 