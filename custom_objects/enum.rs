//2.Enum konusu

// Create an `enum` to classify a web event. Note how both
// names and type information together specify the variant:
// `PageLoad != PageUnload` and `KeyPress(char) != Paste(String)`.
// Each is different and independent.

//enum içinde istediğimiz data tipleri eklenebiliyor.
//pageLoad,padeUnload,keypress,PAste ,struct gibi datalar tanımlanabiliyor.
enum WebEvent {
    // An `enum` may either be `unit-like`,
    PageLoad,
    PageUnload,
    // like tuple structs,
    KeyPress(char),
    Paste(String),
    // or c-like structures.
    Click { x: i64, y: i64 },
}

//web eventi arguman olarak alan bir fn. alttta tanımlanmış.f64
//match ile hangisi tespit ediliyor.f64

// A function which takes a `WebEvent` enum as an argument and
// returns nothing.
fn inspect(event: WebEvent) {

    match event {
        WebEvent::PageLoad => println!("page loaded"),
        WebEvent::PageUnload => println!("page unloaded"),
        // Destructure `c` from inside the `enum`.
        WebEvent::KeyPress(c) => println!("pressed '{}'.", c),
        WebEvent::Paste(s) => println!("pasted \"{}\".", s),
        // Destructure `Click` into `x` and `y`.
        WebEvent::Click { x, y } => {
            println!("clicked at x={}, y={}.", x, y);
        },
    }
}

//enumdan objeler yaratılmış.
fn main() {
    use crate::WebEvent::{PageLoad,PageUnload};
    
    let pressed = WebEvent::KeyPress('x');
    // `to_owned()` creates an owned `String` from a string slice.
    let pasted  = WebEvent::Paste("my text".to_owned());//owned string nedir bilmiyoruz.
    let click   = WebEvent::Click { x: 20, y: 80 };
    let load    = PageLoad;
    let unload  = PageUnload;

    inspect(pressed);
    inspect(pasted);
    inspect(click);
    inspect(load);
    inspect(unload);
}

