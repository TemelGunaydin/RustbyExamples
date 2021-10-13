fn main() {
    // tipini :bool olarakta yazabiliriz.
    // Variables can be type annotated.
    let logical: bool = true;

    let a_float: f64 = 1.0; // Regular annotation
                            //altta suffif olarak i32 kullanılmış.
    let an_integer = 5i32; // Suffix annotation

    // Or a default will be used.
    //varsayılan olarak atanan değer otomtik bulunuyor.
    let default_float = 3.0; // `f64`
    let default_integer = 7; // `i32`

    // A type can also be inferred from context
    let mut inferred_type = 12; // Type i64 is inferred from another line
    inferred_type = 4294967296i64;
    // A mutable variable's value can be changed.
    let mut mutable = 12; // Mutable `i32`
    mutable = 21;
    // Error! The type of a variable can't be changed.
    mutable = true;
    // Variables can be overwritten with shadowing.
    //alt ise üstteki i32 olarak tanımlanan mutable depişkeni altta bool olarak tekrar tanımlanabiliyor.
    //üsttekini gölgeler.
    let mutable = true;
}
