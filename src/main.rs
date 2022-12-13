pub mod file_handler;

pub mod Y2015;
pub mod Y2022;

fn main() {
    // print_result(crate::Y2015::days::day_01::Day01::run());
    // print_result(crate::Y2015::days::day_02::Day02::run());

    // print_result(crate::Y2022::days::day_01::Day01::run());
    // print_result(crate::Y2022::days::day_02::Day02::run());
    // print_result(crate::Y2022::days::day_03::Day03::run());
    // print_result(crate::Y2022::days::day_04::Day04::run());
    // print_result(crate::Y2022::days::day_05::Day05::run());
    // print_result(crate::Y2022::days::day_06::Day06::run());
    // print_result(crate::Y2022::days::day_07::Day07::run());
    // print_result(crate::Y2022::days::day_08::Day08::run());
    // print_result(crate::Y2022::days::day_09::Day09::run());
    // print_result(crate::Y2022::days::day_10::Day10::run());
    // print_result(crate::Y2022::days::day_11::Day11::run());
    // print_result(crate::Y2022::days::day_12::Day12::run());
    print_result(crate::Y2022::days::day_13::Day13::run());
}

fn print_result(result: (String, String, String, u128, u128)) {
    print!(
        "{}\nPart_1: {}\nPart_2: {}\n\nDuration_1(ns): {}\nDuration_2(ns): {}\n-----\n\n",
        result.0, result.1, result.2, result.3, result.4
    );
}
