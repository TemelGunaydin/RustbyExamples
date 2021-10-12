//Kendi data tipimizi ekrana basmak için Debug'ı implement etmek lazım.
/*
#![allow(unused)]
fn main() {
    // debug ı implement etmediği için ekrana basılamaz.
    // This structure cannot be printed either with `fmt::Display` or
    // with `fmt::Debug`.
    struct UnPrintable(i32);

    // altta struct'ı ekrana basmak için, derive niteliği, otomatik olarak gerekli implementation yapar.
    // The `derive` attribute automatically creates the implementation
    // required to make this `struct` printable with `fmt::Debug`.
    #[derive(Debug)]
    struct DebugPrintable(i32);
}
*/

//Alttakiler artık ekrana basılır.
// Derive the `fmt::Debug` implementation for `Structure`. `Structure`
// is a structure which contains a single `i32`.
#[derive(Debug)]
struct Structure(i32);

// Put a `Structure` inside of the structure `Deep`. Make it printable
// also.
#[derive(Debug)]
struct Deep(Structure);

fn main() {
    // alt tarafı {} ile de basılır. aynı çıktıdır.
    // Printing with `{:?}` is similar to with `{}`.
    println!("{:?} months in a year.", 12);
    // alt tarafı {} ile de basılır. aynı çıktıdır.
    println!(
        "{1:?} {0:?} is the {actor:?} name.",
        "Slater",
        "Christian",
        actor = "actor's"
    );

    //şimdi struct ekrana basılır.output : Structure(3)
    // `Structure` is printable!
    println!("Now {:?} will print!", Structure(3));
    // The problem with `derive` is there is no control over how
    // the results look. What if I want this to just show a `7`?
    //olduğu gibi ekrana verilir.
    println!("Now {:?} will print!", Deep(Structure(7)));
}
/*
#[derive(Debug)]
struct Person<'a> {
    name: &'a str,
    age: u8,
}

fn main() {
    let name = "Peter";
    let age = 27;
    let peter = Person { name, age };

    // Pretty print
    println!("{:#?}", peter);
}
*/
