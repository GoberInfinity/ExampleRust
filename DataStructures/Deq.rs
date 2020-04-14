fn main(){
    // A VecDeque is best thought of as a
    // First-In-First-Out (FIFO) queue

    // Usually, you will use it to push_back data
    // and then remove it again with pop_front
    let mut orders = VecDeque::new();
    orders.push_back("oysters");
    orders.push_back("fish and chips");
    let prepared = orders.pop_front();

    // You can freely switch your pushing
    // from front to back and vice versa
    let mut sentence = VecDeque::new();
    sentence.push_back("a");
    sentence.push_front("had");
    sentence.push_back("little");
    sentence.push_front("Mary");
    sentence.push_back("Lamb");
    println!("sentence: {:?}", sentence);

    // The same applies to popping data
    sentence.pop_front();
    sentence.push_front("Jimmy");
    sentence.pop_back();
    sentence.push_back("Cat");
    println!("sentence: {:?}", sentence);

}