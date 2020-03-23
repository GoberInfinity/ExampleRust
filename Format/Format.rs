fn main(){
    //Width
    // All of these print "Hello x    !"
    println!("Hello {:5}!", "x");
    println!("Hello {:1$}!", "x", 5);
    println!("Hello {:width$}!", "x", width = 5);

    //Debug
    format!("{:?}", (3, 4)); // => "(3, 4)"

}