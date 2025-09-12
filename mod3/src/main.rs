
/**
 * Owerneship
 * - Cada valor em Rust tem um dono (owner)
 * - Só pode haver um dono por vez
 * - Quando o dono sai de escopo, o valor é descartado (drop)
 * Obs:
 *      - Tipos primitivos (inteiros, booleanos, floats) implementam o trait Copy, então eles são copiados em vez de movidos
 *      - Tipos compostos (String, Vec, HashMap) não implementam o trait Copy, então eles são movidos, pois possuem alocação dinâmica de memória
 *      - Referências (&T) não são donos, elas apenas emprestam o valor
 * */


fn main() {
    //Tipos primitivos
    let x = 5; // x é o dono do valor 5
    let y = x; // y é uma cópia de x, ambos são donos do valor 5
    println!("x: {}, y: {}", x, y); // nesse caso, como x é um tipo primitivo, ele implementa o trait Copy que é uma cópia bit a bit

    //Tipos compostos
    let s1 = String::from("hello"); // s1 é o dono da String
    let s2 = s1; // s1 é movido para s2, s1 não é mais válido
    // println!("{}", s1); // erro: s1 não é mais válido
    println!("{}", s2); // ok: s2 é o dono da String

    let s3 = &s2; // s3 é uma referência a s2, não é dono
    println!("{}", s3); // ok: s3 é uma referência válida

    {
        let s4 = String::from("world"); // s4 é o dono da String
    }
    // s4 sai de escopo aqui, a String é descartada
    // println!("{}", s4); // erro: s4 não é mais válido

    let s5 = String::from("hello");
    let s6 = s5.clone(); // s6 é uma cópia profunda de s5
    println!("s5: {}, s6: {}", s5, s6); // ok: ambos são válidos 
    // Clone faz uma cópia profunda, alocando nova memória para o valor copiado

}
