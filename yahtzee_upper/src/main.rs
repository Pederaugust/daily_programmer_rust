fn main() {
    let test_a:[i8; 5] = [2, 3, 5, 5, 6]; // => 10
    let test_b:[i8; 5] = [1, 1, 1, 1, 3]; // => 4
    let test_c:[i8; 5] = [1, 1, 1, 3, 3]; // => 6
    let test_d:[i8; 5] = [1, 2, 3, 4, 5]; // => 5
    let test_e:[i8; 5] = [6, 6, 6, 6, 6]; // => 30

    println!("a => {}", yahtzee_upper(test_a));
    println!("b => {}", yahtzee_upper(test_b));
    println!("c => {}", yahtzee_upper(test_c));
    println!("d => {}", yahtzee_upper(test_d));
    println!("e => {}", yahtzee_upper(test_e));
}

fn yahtzee_upper(arr: [i8; 5]) -> i8 {
    let mut max: i8 = 0;
    let mut counter:i8 = 1;

    while counter <= 6 {

        let mut x = 0;

        for item in arr.iter() {
            if item == &counter {
                x = item + x;
            }
        }
        if x > max {
            max = x;
        }
        counter = counter + 1;
    }

    return max;
}
