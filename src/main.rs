fn main() {
    println!("4th fib: {}", fib(4));
    println!("5th fib: {}", fib(5));
    println!("10th fib: {}", fib(10));
    println!("11th fib: {}", fib(11));
    println!("100F in C: {}", ftoc(100.0));
    println!("37C in F: {}", ctof(37.78));
}

// Chapter 3 exercises
fn ctof(c: f32) -> f32 {
   c * 1.8 + 32.0
}

fn ftoc(f: f32) -> f32 {
    (f - 32.0) / 1.8
}

fn fib(number: i32) -> i32 {
    if number < 1 {
        return 0;
    }

    if number <= 2 {
        return 1;
    }

    let mut one_before = 1;
    let mut two_before = 1;
    let mut answer = 1;

    for _number in 1..number - 1 {
        answer = one_before + two_before;
        two_before = one_before;
        one_before = answer;
    }

    answer
}

// TODO: 12 Days of christmas
