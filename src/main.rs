// all about conditionals from book Rust by Example

// if/else

fn main() {
    let n = 500;
    if n < 0 {
        println!("{} is negative!", n);
    } else if n > 0 {
        println!("{} is positive!", n);
    } else {
        println!("{} is zero!", n);
    }

    let big_n = if n < 10 && n > 0 {
        println!("{} is a small number so increase ten-folds!", n);
        n * 10
    } else {
        println!("{} is a big number so cut in half!", n);
        n / 2
    }; // all let bindings need closing semicolon
    println!("{} -> {}", n, big_n);

    // loop
    let mut count = 0u32; // look at this unsigned 32 formatting
    println!("Lets count until infinity!");
    loop {
        // infinite loop and keywords
        count += 1; // increment on each loop
        if count == 3 {
            println!("skip three");
            // skip this loop and move to the next
            continue;
        }
		println!("{}", count);
		if count == 15 {
			println!("OK thats enough");
			// exit the loop
			break;
		}
    }
    // call function using for-loops
    forloop();
}

fn forloop() {
    println!("\nAll about for-loops.");
    let numbers = 30..51;   // note this is a range
    for num in numbers {
        println!("the number is {}", num);
    }
    // using vector data type and its methods
    println!("\nFor-loops using vectors and iter method.");
    let animals = vec!["Rabbit", "Dog", "Cat", "Mouse"];
    for an in animals.iter() {
        println!("\nAnimal name is {}", an);
    }
    // using enumerate method to get index in iterator
    for (index, an) in animals.iter().enumerate() {
        println!("\nindex:{}, name:{}", index, an);
    }
    
}