pub mod file_handler;

pub mod days;

fn main() {
    // print_result(crate::days::day_01::Day01::run());
    // print_result(crate::days::day_02::Day02::run());
    print_result(crate::days::day_03::Day03::run());
}

fn print_result(result: (String, String, String, u128, u128)) {
    print!(
        "{}\nPart_1: {}\nPart_2: {}\n\nDuration_1(ns): {}\nDuration_2(ns): {}\n-----\n\n",
        result.0, result.1, result.2, result.3, result.4
    );
}
