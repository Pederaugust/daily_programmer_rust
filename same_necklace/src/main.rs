
fn main() {
    let a = String::from("abcabcabc");
    let b = String::from("bcabcabca");

    if same_necklace(&a, &b) {
        println!("De er like");
    } else {
        println!("De er ikke like");
    }
    println!("repeats(\"abcabcabc\") {}", repeats(&a));
    println!("repeats(\"abc\") => {}", repeats(&String::from("abc")));
    println!("repeats(\"abcabcabcx\") => {}", repeats(&String::from("abcabcabcx")));
    println!("repeats(\"aaaaaa\") => {}", repeats(&String::from("aaaaaa")));
    println!("repeats(\"\") => {}", repeats(&String::from("")));
    println!("repeats(\"a\") => {}", repeats(&String::from("a")));
}

fn move_last_letter_first(s: &mut String ) {
    let x = s.remove(0);
    s.push(x);
}

fn same_necklace(a: &String, b: &String) -> bool {

    let mut s = b.clone();
    let mut counter = 0;

    while counter < a.len() {
        if s == a.to_string() {
            return true;
        }
        move_last_letter_first(&mut s);
        counter = counter + 1;
    }
    return false;
}

fn repeats(initial: &String) -> usize {
    if initial == ""{
        return 1;
    }

    let mut s = initial.clone();
    let mut times = 0;
    let mut counter = 0;

    while counter < initial.len() {
        move_last_letter_first(&mut s);
        counter = counter + 1;
        if s == initial.to_string() {
            times = times + 1;
        }
    }
    return times;
}
