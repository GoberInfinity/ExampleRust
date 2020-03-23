
fn main(){
    //Vec it is implemented as a continuous chunk of memory stored on the heap
    // The `vec!` macro can be used to initialize a vector
    let fruits = vec!["apple", "tomato", "pear"];
    println!("fruits: {:?}", fruits);

    // Creates an empty vector and fill it
    let mut fruits = Vec::new();
    fruits.push("apple");
    fruits.push("tomato");
    fruits.push("pear");
    println!("fruits: {:?}", fruits);

    // Initialize the vector with a value
    let bunch_of_zeroes = vec![0; 5];

    // Initialize the vector with a certain capacity
    let mut large_vec: Vec<i32> = Vec::with_capacity(1_000_000);

    // Access arbitrary elements
    let second = fruits.get(1);
    if let Some(second) = second {
    println!("Second fruit: {}", second);
    }

    // Access arbitrary elements without bonds checking
    let second = fruits[1];
    println!("Second fruit: {}", second);

}

