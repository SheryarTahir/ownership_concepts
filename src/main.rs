/* Slice Type */
fn main() {
    let s = String::from("HelloWorld");
    let res = find_first_word(&s);

    println!("For String {s} the Result is {}", res.len());

}

fn find_first_word(input: &String) -> &str {
    let s = input.as_bytes();

    for (i, &item) in s.iter().enumerate() {
        if item == b' ' {
            return &input[..i];
        }
    }
    &input[..]
}



/*  Referncing
fn main() {
    let mut s1 = String::from("Sheryar Tahir");
    let len = calculate_length(&mut s1);
    s1.push_str(" ABC");

    println!("The len of {s1} is {len}");

}

fn calculate_length(s: &mut String) -> usize {
    s.push_str(" Hello World");
    s.len()
}
*/













/* 
fn main() {
    let num = 15;
    let result = add(num);
    let s = gives_ownership();
    let s = takes_and_gives_back(s);

    println!("s = {s}");
    let name = String::from("Sheryar Tahir");
    takes_ownership(name);
    println!("Num is {num} and result is {result}"); 

}

fn gives_ownership() -> String {
    let s = String::from("yours");
    s
}

fn takes_ownership(s: String) {
    println!("Inside Ownership {s}");
}

fn takes_and_gives_back(s: String) -> String {
    println!("S takes_and_gives_back {s}");
    s
}

fn add(x: i32) -> i32 {
    x + 10
}
*/