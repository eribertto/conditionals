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
}
