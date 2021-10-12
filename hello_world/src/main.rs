//Formatting Genel bilgilendirme
fn main() {
    // direkt 31 ekrana basar
    // Altta 31 varsayılan olarak i32 kabul edilir.Eğer suffix kullanılırsa
    // 31i64 gibi bu sefer interger 64 bit gibi davranılır.
    // Without a suffix, 31 becomes an i32. You can change what type 31 is
    // by providing a suffix. The number 31i64 for example has the type i64.
    println!("{} days", 31);

    // Pythonda var. 0.yer ,1.yer gibi ayrımlar var.Positional arguments olarak geçer
    // There are various optional patterns this works with. Positional
    // arguments can be used.
    println!("{0}, this is {1}. {1}, this is {0}", "Alice", "Bob");

    // argumanlara isim vererek ekrana verilebilir. artık {} içine argumanın ismi yazılır.
    // As can named arguments.
    println!(
        "{subject} {verb} {object}",
        object = "the lazy dog",
        subject = "the quick brown fox",
        verb = "jumps over"
    );

    //{:} ile farklı formatta sayı ekrana verilir. 2 sayısı binary olarak 10 olarak verilir.
    // Special formatting can be specified after a `:`.
    println!(
        "{} of {:b} people know binary, the other half doesn't",
        1, 2
    );

    // named argument kullanarak, ve :>$ kullanarak 1 sayısı sağa 6 dayalı olarak ekrana verilir.
    // You can right-align text with a specified width. This will output
    // "     1". 5 white spaces and a "1".
    println!("{number:>width$}", number = 1, width = 6);

    // named argument kullanarak, ve :0>$ kullanarak 1 sayısı sağa 6 dayalı olarak ve 0 olcak şekilde ekrana verilir.
    // You can pad numbers with extra zeroes. This will output "000001".
    println!("{number:0>width$}", number = 1, width = 6);

    // positional argument olarak 1 verilmiş,ama değer atanmamış.hata verir.
    // Rust even checks to make sure the correct number of arguments are
    // used.
    // println!("My name is {0}, {1} {0}", "Bond");
    // FIXME ^ Add the missing argument: "James"

    //i32 içeren bir struct yaratılmış.
    // Create a structure named `Structure` which contains an `i32`.
    #[allow(dead_code)]
    struct Structure(i32);

    // However, custom types such as this structure require more complicated
    // handling. This will not work.
    //println!("This struct `{:}` won't print...", Structure(3));
    // FIXME ^ Comment out this line.
    //ödev : pi sayısını virgülden sonra 3 basamak olarak ekrana ver.{:.} kullanılıyor
    let pi = 3.141592;
    println!("{:.3}", pi);
}
