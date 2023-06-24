use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

fn main() {
    // Create a path to the desired file
    let path = Path::new("input.txt");
    let display = path.display();

    // Open the path in read-only mode, returns `io::Result<File>`
    let mut file = match File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}", display, why),
        Ok(file) => file,
    };

    // Read the file contents into a string, returns `io::Result<usize>`
    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Err(why) => panic!("couldn't read {}: {}", display, why),
        Ok(_) => {}
    }

    // Part 1
    // let result = s
    //     .split("\n\n")
    //     .map(|group| {
    //         group
    //             .split("\n")
    //             .map(|number| number.parse::<i32>().unwrap_or_default())
    //             .sum::<i32>()
    //     })
    //     .max()
    //     .unwrap();

    // Part 2
    let result: u64 = s
        .as_str()
        .split("\n\n")
        .map(|batch| {
            batch
                .lines()
                .map(|line| line.parse::<u64>().unwrap())
                .sum::<u64>()
        })
        .top_n(3)
        .iter()
        .sum();

    println!("{:?}", result);
}


trait TopN<T> {
    fn top_n(self, n: usize) -> Vec<T>;
}

impl<T: PartialOrd, U: Iterator<Item = T>> TopN<T> for U {
    fn top_n(self, n: usize) -> Vec<T> {
        let mut top = Vec::with_capacity(n);
        for value in self {
            for i in 0..n {
                if let Some(top_value) = top.get(i) {
                    if value > *top_value {
                        top[i..].rotate_right(1);
                        top[i] = value;
                        break;
                    }
                } else {
                    top.push(value);
                    break;
                }
            }
        }
        top
    }
}

pub fn part2_combinators_no_sort(input: &str) -> u64 {
    input
        .split("\n\n")
        .map(|batch| {
            batch
                .lines()
                .map(|line| line.parse::<u64>().unwrap())
                .sum::<u64>()
        })
        .top_n(3)
        .iter()
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn top_value_functional() {
        let arr = [3, 5, 8, 2, 1, 9, 0, 4, 7, 1];

        let out = arr.iter().top_n(3);

        println!("Input {:?}, output {:?}", arr, out);

        assert_eq!(out, vec![&9, &8, &7]);
    }
}
