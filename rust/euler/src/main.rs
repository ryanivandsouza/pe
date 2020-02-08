mod solutions;
mod utils;

fn main() {
    let i = get_problem_number();
    let solutions = solutions::get_solutions();
    for i in &solutions[i] {
        utils::run_with_time_logging(*i)
    }
}

fn get_problem_number() -> usize {
    let input = utils::get_user_input("Enter problem number: ");
    utils::safely_parse::<usize>(input)
}
