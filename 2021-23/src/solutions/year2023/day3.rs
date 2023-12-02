pub fn run(input: String) -> (u32, u32) {
    let input: Vec<&str> = input.lines().collect();

    let answer_one = part_one(&input);
    let answer_two = part_two(&input);

    (answer_one, answer_two)
}

fn part_one(input: &[&str]) -> u32 {
    0
}

fn part_two(input: &[&str]) -> u32 {
    0
}

#[macro_export]
macro_rules! safe_unpack {
    ($list:tt, ()) => { };
    ($list:tt, $name:tt) => {
        let $name = match $list.next() {
            Some(val) => val,
            None => return Err(String::from("Not enough values in list.")),
        };
    };
    ($list:tt, $name:tt $(, $vars:tt)+) => {
        safe_unpack!($list, $name);
        safe_unpack!($list $(, $vars)+)
    };
    ($e:expr $(, $vars:tt)+) => {
        let mut temp_iter = $e;
        safe_unpack!(temp_iter $(, $vars)+);
    };
}
