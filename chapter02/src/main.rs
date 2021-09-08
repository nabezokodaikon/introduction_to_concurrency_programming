enum Gender {
    Male,
    Female,
}

enum Role {
    Player(u32, u64),
    Supporter(u32),
}

struct Person {
    age: u32,
    gender: Gender,
    role: Role,
}

struct Pair<A> {
    first: A,
    second: A,
}

enum Result<T, E> {
    Ok(T),
    Err(E),
}

fn let_example() -> u32 {
    let x = 100;
    let mut y = 20;
    let z: u32 = 5;
    let w;
    y = x + z;
    w = 8;
    y + w
}

fn hello(v: u32) {
    println!("Hello World!: v = {}", v);
}

fn add(x: u32, y: u32) -> u32 {
    x + y
}

fn my_func1() {
    let n = add(10, 5);
    hello(n);
}

fn is_even(v: u32) -> bool {
    if v % 2 == 0 {
        true
    } else {
        false
    }
}

fn pred(v: u32) -> Option<u32> {
    if v == 0 {
        None
    } else {
        Some(v - 1)
    }
}

fn print_pred(v: u32) {
    match pred(v) {
        Some(w) => {
            println!("pred({}) = {}", v, w);
        }
        None => {
            println!("pred({}) is undefined", v);
        }
    }
}

fn main() {
    let p = Person {
        age: 20,
        gender: Gender::Female,
        role: Role::Supporter(70),
    };

    println!("Hello, world!");
    println!("{}", let_example());

    my_func1();

    println!("{}", is_even(10));

    print_pred(1);
}
