/*
 * Regras de referências em Rust
 * Uma referência mutável por vez (Não pode haver mais de uma referência mutável ao mesmo tempo para a mesma variável)
 * Pode  ter muitas referências imutáveis ao mesmo tempo para a mesma variável. Isso permite que várias partes do código leiam o valor ao mesmo tempo, desde que nenhuma delas tente modificá-lo.
 * Mutável e imutável de uma varável não podem coexistir
 * Scope of a reference (escopo de uma referência)
 * Os dados não devem mudar quando referências imutáveis estão em escopo
*/

fn main() {
    // 1° regra
    let mut heap_num: Vec<i32> = vec![1, 2, 3, 4, 5];
    let _ref1 = &mut heap_num;
    let _ref2 = &mut heap_num; // erro: cannot borrow `heap_num` as mutable more than once at a time

    // 2° regra
    let mut heap_num2: Vec<i32> = vec![3, 4, 5];
    let ref1 = &heap_num2; // referência imutável
    let ref2 = &heap_num2; // outra referência imutável
    println!("{:?} ,{:?}", ref1, ref2);

    // 3° regra
    let mut heap_num3: Vec<i32> = vec![6, 7, 8];
    let ref3: &Vec<i32> = &heap_num3; // referência imutável
    let ref4: &mut Vec<i32> = &mut heap_num3; // outra referência mutável -> error because it is also borrowed as immutable

    //println!("{:?} ,{:?}", ref3, ref4);

    // 4° regra
    let mut heap_num4: Vec<i32> = vec![9, 10, 11];
    let ref5: &Vec<i32> = &heap_num4; // referência imutável
    println!("{:?}", ref5);

    // Depois que a referência imutável sai de escopo (ela saiu pq ref5 saiu de escopo depois de ser usado), podemos criar uma referência mutável
    let ref6: &mut Vec<i32> = &mut heap_num4; // referência mutável
    ref6.push(12);
    println!("{:?}", ref6);

    // 5° regra
    let mut heap_num5: Vec<i32> = vec![13, 14, 15];
    let ref7: &Vec<i32> = &heap_num5; // referência imutável

    /* Tu declarou heap_num5 como mutável → beleza, poderia dar heap_num5.push(99).
       Só que tu criou ref7 como &Vec<i32> → isso é uma referência imutável.
       Enquanto existir uma referência imutável,
       o compilador não deixa você criar uma referência mutável (como o &mut usado por dentro do push),
       porque poderia dar data race (alguém lendo enquanto alguém escreve).
    */

    /*
        O método .push() em um Vec<T> no Rust:
        Adiciona um novo elemento no final do vetor.
        Recebe o valor por ownership (ou seja, o valor é movido pro vetor).
        Se o vetor já tiver atingido a capacidade interna, ele realoca mais memória no heap (dobra ou aumenta a capacidade) e copia os elementos antigos pro novo espaço.
        Por isso ele precisa de &mut self → está modificando a estrutura interna.
        */
    println!("{:?}", ref7);

    heap_num5.push(16); // ok, pq ref7 saiu de escopo

}
