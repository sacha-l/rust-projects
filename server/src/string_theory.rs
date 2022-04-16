
fn main() {

    let string = String::from("127.0.0.1:8080");
        
    // create a slice of the string 
    let string_slice = &string[10..];

    // borrow a slice of the string
    let string_borrow: &str = &string;

    // create a string literal
    let string_literal = "1234";

    dbg!(&string);
    dbg!(string_slice);
    dbg!(string_borrow);
    dbg!(string_literal);

}