//mem isimli modulu kullanıyoruz.
use std::mem;

// This function borrows a slice
fn analyze_slice(slice: &[i32]) {
    println!("first element of the slice: {}", slice[0]);
    println!("the slice has {} elements", slice.len());
}

fn main() {
    // Fixed-size array (type signature is superfluous)
    // 5 elemanlı i32 tipi sayı içeren dizi tanımlaması. sadece let xs yazılsada olurdu.
    let xs: [i32; 5] = [1, 2, 3, 4, 5];

    // All elements can be initialized to the same value
    //i32;500 ile 500 elemanlı dizi tanımlanıyor. ancak = [0;500] ise 500 elemanında 0 olmasını sağlar.
    let ys: [i32; 500] = [0; 500];

    // Indexing starts at 0 ...Anlaşılabilir...
    println!("first element of the array: {}", xs[0]);
    println!("second element of the array: {}", xs[1]);

    // `len` returns the count of elements in the array ..Anlaşılabilir...
    println!("number of elements in array: {}", xs.len());

    // Arrays are stack allocated
    //array stackte yaratılır fln demiş. arrayin kapladığı alan byte olarak verilmiş.
    //xs dizisi 5 elemanlı ve i32 olarak data alıyor.yani 4 byte * 5 = 20 byte sonuc verir.
    println!("array occupies {} bytes", mem::size_of_val(&xs));

    // Arrays can be automatically borrowed as slices
    //şimdi ise slice kısmına geldik.tüm diziyi slice olarak alıyor. detaylı fonk. içinde tanımlı yukarda..
    println!("borrow the whole array as a slice");
    analyze_slice(&xs);

    // Slices can point to a section of an array
    // They are of the form [starting_index..ending_index]
    // starting_index is the first position in the slice
    // ending_index is one more than the last position in the slice
    println!("borrow a section of the array as a slice");
    analyze_slice(&ys[1..4]); //dizinin sadece 1-4 arasını alıyor. 4 dahil değil... aralarında .. nokta var.

    // Out of bound indexing causes compile error ..Anlaşılabilir...
    //println!("{}", xs[5]);
}
