fn main() {
    println!("Welcome");
    fiz_buzz();
}

fn fiz_buzz() {
    let mut count = 0;
    for i in 1..=301 {
        // println!("{}", i);
        if i % 3 == 0 {
            println!("Fizz");
        }
        if i % 3 == 0 {
            println!("Buzz");
        }

        if i % 3 == 0 && i % 5 == 0 {
            println!("Fizz Buzz");
            count += 1;
        }
    }

    println!("Fizz Buzz was printed {} times", count);
}
