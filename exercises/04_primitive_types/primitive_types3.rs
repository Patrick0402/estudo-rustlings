fn main() {
    // TODO: Create an array called `a` with at least 100 elements in it.
    // let a = ???

    // let a = ["Array com 100 elementos"; 100];

    // let a = [0; 100]; // 100 zeros

    // let a = "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa";

    // let a: Vec<u32> = (0..100).collect();

    // let a: [u32; 100] = std::array::from_fn(|i| i as u32);

    // let a: [u32; 100] = std::array::from_fn(|i| i as u32 + 1);


    // let a: [u32; 100] = std::array::from_fn(|i| if i % 2 == 0 { 0 } else { 1 });
    
    let a: [u32; 100] = std::array::from_fn(|i| {
        if i % 2 == 0 {
            (i + 100) as u32
        } else {
            (i + 200) as u32
        }
    });

    //Se a for um Vec<32> é necessário utilizar a função clone() aqui, pois esse tipo não implementa 'Copy' trait.
    for item in a {
        println!("{}", item);
    }
    //O exercício funciona se a for uma string com 100 caracteres, mas é preciso comentar esse for.

    if a.len() >= 100 {
        println!("Wow, that's a big array!");
    } else {
        println!("Meh, I eat arrays like that for breakfast.");
        panic!("Array not big enough, more elements needed");
    }
}
