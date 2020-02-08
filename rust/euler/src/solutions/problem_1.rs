fn implementation1() -> String {
    let mut result = 0i32;
    for n in 1..1000 {
        if n % 3 == 0 || n % 5 ==0 {
            result += n;
        }
    }

    format!("{}", result)
}

pub fn get_implementations() -> Vec<fn() -> String> {
    vec![
        implementation1
    ]
}
