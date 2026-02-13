fn main() {
    let s = String::from("Sheryar Tahir");
    let (s, len) = calculate_len(s);

    println!("The len of {s} is {len}");

}

fn calculate_len(s: String) -> (String, usize) {
    let result = s.len();
    (s, result)
}

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