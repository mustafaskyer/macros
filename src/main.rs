use std::num::ParseIntError;

macro_rules! our_macro {
    () => {};
    ($e1: expr, $e2: expr) => {
        $e1 + $e2;
    };
    ($a: expr, $b: expr; $c: expr) => {
        $a + $b + $c;
    };
}

macro_rules! input {
    ($t: ty) => {{
        let mut n = String::new();
        std::io::stdin()
            .read_line(&mut n)
            .expect("Failed to read line");
        // variable n can be any type that has been matched by the type capture
        let n: $t = n.trim().parse().expect("Please type a number!");
        n
    }};
}

macro_rules! some_macro {
    ($var: ident) => {
        $var = $var + 1
    };
}

macro_rules! create_function {
    ($func_name: ident, $input: ident, $type: ty, $output: ty) => {
        fn $func_name($input: $type) -> $output {
            println!("You called {:?}()", stringify!($func_name));
            println!("You passed {:?} = {:?}", stringify!($input), $input);
        }
    };
}

create_function!(foo, x, i32, ());

macro_rules! string_concat {
    ($($some_macro_str: expr),*) => {{
        let mut temp_str = String::new();
        // kinda like a for loop
        $(temp_str.push_str($some_macro_str);)*
        temp_str
    }};
}

macro_rules! vec_macro {
    ($($element: expr), *) => {{
        let mut some_vec = Vec::new();
        $(some_vec.push($element);)*
        some_vec
    }};
}

macro_rules! make_struct {

    ($name:ident {$($field:ident: $ty:ty),* }) => {

        struct $name {
            $($field: $ty,)*
        }

    };

}

make_struct!(Person {
    name: String,
    age: u8
});

/*
Question Mark Operator
*/
fn parse_string(input: &str) -> Result<i32, ParseIntError> {
    let integer = input.parse()?;
    println!("the value {:?} is integer {:?}", input, integer);
    Ok(integer)
}

fn divisor(divident: f64, divisor: f64) -> Result<f64, String> {
    let answer = match divisor {
        0.0 => Err(String::from("Error: Divison by zero!!")),
        _ => Ok(divident / divisor),
    };
    let correct = answer?;
    println!("This line will not print in case of error {:?}", correct);
    Ok(correct)
}

#[derive(Debug)]
enum MathError {
    DivisionError_DivisionByZero,
    LogError_NonPositiveLogrithm,
    SqrtError_NegativeSquareRoot,
}
type MathResult = Result<(), MathError>;
fn division(x: f64, y: f64) -> MathResult {
    if (y == 0.0) {
        Err(MathError::DivisionError_DivisionByZero)
    } else {
        println!("success division {}", x / y);
        Ok(())
    }
}
fn sqrt(x: f64) -> MathResult {
    if x <= 0.0 {
        Err(MathError::SqrtError_NegativeSquareRoot)
    } else {
        println!("success sqrt {}", x.sqrt());
        Ok(())
    }
}
fn ln(x: f64) -> MathResult {
    if x <= 0.0 {
        Err(MathError::SqrtError_NegativeSquareRoot)
    } else {
        println!("success ln {}", x.ln());
        Ok(())
    }
}

fn operations(x: f64, y: f64) -> MathResult {
    division(x, y)?;
    sqrt(x)?;
    ln(x)?;

    Ok(())
}
fn main() {
    // println!("{}", our_macro!());
    // println!("{}", our_macro!(1, 2));
    // println!("A + B + C = {}", our_macro!(1, 2; 3));

    // our_macro!();
    // our_macro![];
    // our_macro! {};

    // capturing types
    // println!("Please enter a floating point number:");
    // let n = input!(f32);
    // println!("You entered: {:?}", n);

    // identifiers
    // let mut x = 4;
    // some_macro!(x);

    // println!("x = {}", x);

    // let z = 4;
    // foo(z);

    /*
    Repeating Patterns
     */
    // let str_null = string_concat!();
    // let str_single = string_concat!("Hello");
    // let str_single = string_concat!("Hello", "World");

    // let str_vec = vec_macro!(1, 2, 3, 4, 5);
    // println!("{:?}", str_vec);

    // let p = Person {
    //     name: String::from("Alice"),
    //     age: 30,
    // };
    // println!("Name: {}, Age: {}", p.name, p.age);

    /*
    Question Operator
     */
    let some_value = vec!["123", "some1", "some(123)", "abc", "53"];
    for value in some_value {
        println!("{:?}", parse_string(value));
    }

    println!(
        "Call from main with result equals to {:?}",
        divisor(9.0, 3.0)
    );
    println!(
        "Call from main with result equals to {:?}",
        divisor(4.0, 0.0)
    );
    println!(
        "Call from main with result equals to {:?}",
        divisor(0.0, 2.0)
    );

    let result = operations(0.0, 19.0);
    if result.is_ok() {
        println!("Success result")
    } else {
        println!("{:?}", result);
    };
}
