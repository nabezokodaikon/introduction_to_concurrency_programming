use std::ops::Add;
use std::thread::spawn;

fn hello2() {
    println!("Hello World!");
}

fn my_func11() {
    spawn(hello2).join();
    let h = || println!("Hello World!");
    spawn(hello2).join();
}

fn my_func12() {
    let v = 10;
    let f = move || v * 2;

    let result = spawn(f).join();
    println!("result = {:?}", result);

    match spawn(|| panic!("I'm panicked")).join() {
        Ok(_) => {
            println!("successed");
        }
        Err(a) => {
            let s = a.downcast_ref::<&str>();
            println!("failed: {:?}", s);
        }
    }
}

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

fn even_odd() {
    for n in 0..10 {
        println!("{} is {}", n, if is_even(n) { "even" } else { "odd" });
    }
}

fn even_odd2() {
    let mut n = 0;
    loop {
        println!("{} is {}", n, if is_even(n) { "even" } else { "odd" });
        n += 1;
        if n >= 10 {
            break;
        }
    }
}

fn mul(x: &mut u64, y: &u64) {
    *x *= *x * *y;
}

fn my_func2() {
    let mut n = 10;
    let m = 20;
    println!("n = {}, m = {}", n, m);
    mul(&mut n, &m);
    println!("n = {}, m = {}", n, m);
}

fn app_n(f: fn(u64) -> u64, mut n: u64, mut x: u64) -> u64 {
    loop {
        if n == 0 {
            return x;
        }
        x = f(x);
        n -= 1;
    }
}

fn mul2(x: u64) -> u64 {
    x * 2
}

fn my_func3() {
    println!("app_n(mul2, 4, 3) = {}", app_n(mul2, 4, 3));
}

fn mul_x(x: u64) -> Box<dyn Fn(u64) -> u64> {
    Box::new(move |y| x * y)
}

fn my_func4() {
    let f = mul_x(3);
    println!("f(5) = {}", f(5));
}

struct Apple {}
struct Gold {}
struct FullStomach {}

fn get_gold(a: Apple) -> Gold {
    Gold {}
}

fn get_full_stomach(a: Apple) -> FullStomach {
    FullStomach {}
}

fn my_func5() {
    let a = Apple {};
    let g = get_gold(a);
    // let s = get_full_stomach(a);
}

struct Foo {
    val: u32,
}

fn add2<'a>(x: &'a Foo, y: &'a Foo) -> u32 {
    x.val + y.val
}

fn my_func6() {
    let x = Foo { val: 10 };
    {
        let y = Foo { val: 20 };
        let z = add2(&x, &y);
        println!("z = {}", z);
    }
}

struct Foo2 {
    val: u32,
}

fn add_val(x: Foo2, y: Foo2) -> (u32, Foo2, Foo2) {
    (x.val + y.val, x, y)
}

fn mul_val(x: Foo2, y: Foo2) -> (u32, Foo2, Foo2) {
    (x.val * y.val, x, y)
}

fn my_func7() {
    let x = Foo2 { val: 3 };
    let y = Foo2 { val: 6 };
    let (a, xn, yn) = add_val(x, y);
    let (b, _, _) = mul_val(xn, yn);
    println!("a = {}, b = {}", a, b);
}

struct Foo3 {
    val: u32,
}

fn my_func8() {
    let mut x = Foo3 { val: 10 };
    {
        let a = &mut x;
        println!("a.val = {}", a.val);
        // println!("x.val = {}", x.val);

        let b: &Foo3 = a;
        // a.val = 20;
        println!("b.val = {}", b.val);

        a.val = 30;
    }
}

#[derive(Clone, Copy)]
struct Vec2 {
    x: f64,
    y: f64,
}

impl Add for Vec2 {
    type Output = Vec2;

    fn add(self, rhs: Vec2) -> Vec2 {
        Vec2 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Vec2 {
    fn new(x: f64, y: f64) -> Self {
        Vec2 { x, y }
    }

    fn norm(&self) -> f64 {
        (self.x * self.x + self.y * self.y).sqrt()
    }

    fn set(&mut self, x: f64, y: f64) {
        self.x = x;
        self.y = y;
    }
}

fn my_func9() {
    let mut v = Vec2::new(10.0, 5.0);
    println!("v.norm = {}", v.norm());
    v.set(3.8, 9.1);
    println!("v.norm = {}", v.norm());
}

fn my_func10() {
    let v1 = Vec2 { x: 10.0, y: 5.0 };
    let v2 = Vec2 { x: 3.1, y: 8.7 };
    let v = v1 + v2;
    println!("v.x = {}, v.y = {}", v.x, v.y);
}

fn get_option_1() -> Option<u32> {
    Some(1)
}

fn get_option_2() -> Option<u32> {
    None
}

fn use_option_1() -> Option<u32> {
    let a = get_option_1()?;
    Some(a)
}

fn use_option_2() -> Option<u32> {
    let a = match get_option_1() {
        Some(e) => e,
        None => return None,
    };
    Some(a)
}

fn result_option_1() {
    let a = use_option_1();
    println!("{}", a.unwrap());
}

fn result_option_2() {
    let a = use_option_2();
    println!("{}", a.unwrap());
}

fn get_result_1() -> Result<u32, String> {
    Ok(2)
}

fn get_result_2() -> Result<u32, String> {
    Err("3".to_string())
}

fn use_result_1() {
    let a = get_result_1();
    println!("{}", a.unwrap());
}

fn use_result_2() {
    let a = get_result_2();
    println!("{}", a.unwrap());
}

fn main() {
    let _p = Person {
        age: 20,
        gender: Gender::Female,
        role: Role::Supporter(70),
    };

    println!("Hello, world!");
    println!("{}", let_example());

    my_func1();

    println!("{}", is_even(10));

    print_pred(1);

    even_odd();
    even_odd2();

    my_func2();
    my_func3();
    my_func4();
    my_func5();
    my_func6();
    my_func7();
    my_func8();
    my_func9();
    my_func10();

    result_option_1();
    result_option_2();
    use_result_1();
    // use_result_2();
    let option_1 = get_option_1().unwrap();
    println!("{}", option_1);
    // let option_2 = get_option_2().unwrap();
    // println!("{}", option_2);

    let result_1 = get_result_1().unwrap();
    println!("{}", result_1);
    // let result_2 = get_result_2().unwrap();
    // println!("{}", result_2);

    my_func11();
    my_func12();

    println!("End");
}
