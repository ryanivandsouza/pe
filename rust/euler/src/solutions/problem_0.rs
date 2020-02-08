fn implementation1() -> String {
    let mut result = 0i32;
    for n in 1..1000 {
        if n % 3 == 0 || n % 5 ==0 {
            result += n;
        }
    }

    format!("{}", result)
}

fn implementation2() -> String {
    let result = (0..1000)
        .filter(|n| n % 3 == 0 || n % 5 == 0)
        .sum::<i32>();

    format!("{}", result)
}

fn sum_of_naturals(n: i32) -> i32 {
    n * (n + 1) / 2
}

fn implementation3() -> String {
    let result = sum_of_naturals(1000 / 3)
        + sum_of_naturals(1000 / 5)
        - sum_of_naturals(1000 / 15);

    format!("{}", result)
}

pub fn get_implementations() -> Vec<fn() -> String> {
    vec![
        implementation1,
        implementation2,
        implementation3
    ]
}
