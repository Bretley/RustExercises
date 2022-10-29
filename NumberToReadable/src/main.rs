mod decimal_integer;
mod triple;
fn human_readable_hundreds(x: usize) -> String {
    if x == 0 {
        String::from("")
    } else {
        human_readable_ones(x) + &String::from("Hundred")
    }
}
fn human_readable_bottom_19(x: usize) -> String {
    match x {
        0 => String::from("Zero"),
        1 => String::from("One"),
        2 => String::from("Two"),
        3 => String::from("Three"),
        4 => String::from("Four"),
        5 => String::from("Five"),
        6 => String::from("Six"),
        7 => String::from("Seven"),
        8 => String::from("Eight"),
        9 => String::from("Nine"),
        10 => String::from("Ten"),
        11 => String::from("Eleven"),
        12 => String::from("Twelve"),
        13 => String::from("Thirteen"),
        14 => String::from("Fourteen"),
        15 => String::from("Fifteen"),
        16 => String::from("Sixteen"),
        17 => String::from("Seventeen"),
        18 => String::from("Eighteen"),
        19 => String::from("Nineteen"),
        _ => panic!(),
    }
}

fn human_readable_tens(x: usize) -> String {
    match x {
        2 => String::from("Twenty"),
        3 => String::from("Thirty"),
        4 => String::from("Forty"),
        5 => String::from("Fifty"),
        6 => String::from("Sixty"),
        7 => String::from("Seventy"),
        8 => String::from("Eighty"),
        9 => String::from("Ninety"),
        _ => String::from("Error in tens "),
    }
}

fn human_readable_ones(x: usize) -> String {
    // 0 <= x  <= 10
    let x = x % 10;
    match x {
        0 => String::from("Zero"),
        1 => String::from("One"),
        2 => String::from("Two"),
        3 => String::from("Three"),
        4 => String::from("Four"),
        5 => String::from("Five"),
        6 => String::from("Six"),
        7 => String::from("Seven"),
        8 => String::from("Eight"),
        9 => String::from("Nine"),
        10 => String::from("Ten"),
        _ => panic!("From human_readable_ones, num > 10"),
    }
}

pub fn human_readable(x: usize) -> String {
    let hundreds_place = x / 100;
    let tens_place = (x % 100) / 10;
    let ones_place = x % 10;
    let teens = tens_place * 10 + ones_place;
    println!(
        "{} {} {} {} {}",
        hundreds_place, tens_place, ones_place, x, teens
    );
    if 0 <= teens && teens < 20 {
        // handled bby edge case
        human_readable_hundreds(hundreds_place) + &human_readable_bottom_19(teens)
    } else {
        human_readable_hundreds(hundreds_place)
            + &human_readable_tens(tens_place)
            + &human_readable_ones(ones_place)
    }
}

/*
struct Wrap<T> {
    value: T,
}
 */

fn main() {}
