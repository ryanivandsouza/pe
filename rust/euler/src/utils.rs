use std::io::{stdin,stdout,Write};
use std::time::{Instant, Duration};

pub fn safely_parse<T: std::default::Default + std::str::FromStr>(input: String) -> T {
    let mut problem_number: T = T::default();
    let trimmed = input.trim();
    let error_message = format!("this was not an valid input: {}", trimmed);
    match trimmed.parse::<T>() {
        Ok(i) => problem_number = i,
        Err(..) => println!("{}", error_message),
    };
    problem_number
}

pub fn get_user_input(prompt: &str) -> String {
    print!("{}", prompt);

    let _ = stdout().flush();
    let mut input = String::new();
    stdin().read_line(&mut input)
        .ok()
        .expect("Couldn't read line");

    input
}

pub fn run_with_time_logging(f: fn() -> String) {
    let now = Instant::now();
    println!("Result: {}. Executed in {}", f(), format_duration(now.elapsed()));
}

fn format_duration(d: Duration) -> String {
    if d.as_secs() != 0 {
        return format!("{}s", d.as_secs());
    }
    if d.as_millis() != 0 {
        return format!("{}ms", d.as_millis());
    }
    if d.as_micros() != 0 {
        return format!("{}μs", d.as_micros());
    }
    return format!("{}ns", d.as_nanos());
}
