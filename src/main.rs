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
    let str_null = string_concat!();
    let str_single = string_concat!("Hello");
    let str_single = string_concat!("Hello", "World");

    let str_vec = vec_macro!(1, 2, 3, 4, 5);
    println!("{:?}", str_vec);

    let p = Person {
        name: String::from("Alice"),
        age: 30,
    };
    println!("Name: {}, Age: {}", p.name, p.age);
}
