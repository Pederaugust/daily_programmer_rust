

fn main() {
    let brackets = [Bracket {cap: 10000.0, percent:0.1},
                    Bracket {cap: 30000.0, percent: 0.25},
                    Bracket {cap: 100000.0, percent: 0.4}];

    let test_data:[f32; 7] = [0.0, 10000.0, 10009.0, 10010.0, 12000.0, 56789.0, 1234567.0];

    for data in test_data.iter() {
        println!("tax({}) => {}", *data, tax(*data, &brackets));
    }
}

struct Bracket {
    cap: f32,
    percent: f32,
}

fn tax(income: f32, brackets: &[Bracket]) -> f32 {
    let mut tax:f32 = 0.0;
    let mut remaining: f32 = income;
    for bracket in brackets.iter().rev() {
        if remaining > bracket.cap {
            tax = bracket.percent * (remaining - bracket.cap) + tax;
            remaining = bracket.cap;
        }
    }
    tax
}
