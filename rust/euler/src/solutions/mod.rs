mod problem_0;
mod problem_1;

pub fn get_solutions() -> Vec<Vec<fn() -> String>> {
    vec![
        problem_0::get_implementations(),
        problem_1::get_implementations()
    ]
}
