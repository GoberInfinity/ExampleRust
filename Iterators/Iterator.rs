fn main(){
    let countries = [
                "U.S.A.",
                "Germany",
                "France",
                "Italy",
                "India",
                "Pakistan",
                "Burma",
            ];
            
    // Don't get the searched item but rather its index
    if let Some(pos) = countries
    .iter()
    .position(|country| country.starts_with('I'))
    {
        println!("It's index is: {}", pos);
    }
    
    // Creates an iterator which gives the current iteration count as well as the next value.
    for (count, elem) in countries.iter().enumerate(){
    } 
}
