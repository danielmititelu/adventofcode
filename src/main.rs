mod day_one;
use day_one::print_day_one_answers;

mod day_two;
use day_two::print_day_two_answers;

mod day_two_part_two;
use day_two_part_two::print_day_two_part_two_answers;

fn main() {
    let day_to_print = 2;
    if day_to_print == 1 {
        print_day_one_answers();
    } else if day_to_print == 2 {
        print_day_two_answers();
        print_day_two_part_two_answers();
    } else {
        println!("Day {} not implemented", day_to_print);
    }
}
