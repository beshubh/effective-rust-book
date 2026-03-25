mod types {

    mod strings {
        fn main() {
            let name = "alice";
            let upper = name.to_uppercase();
            let contains = name.contains("lic");
            let parts = "a,b,c".split(",").collect::<Vec<&str>>();
            let joined = ["a", "b", "c"].join("-");
            let stripped = "   Hello  ".trim(); // Hello
            let replaced = name.replace("a", "A"); // alice -> Alice
        }
    }

    mod type_annotation {
        use std::collections::HashMap;

        fn add(a: i32, b: i32) -> i32 {
            a + b
        }

        fn first(items: &[i32]) -> Option<i32> {
            items.first().copied()
        }

        type UserId = i64;
        type Mapping = HashMap<String, Vec<i32>>;
    }

    mod ranges_and_loops {

        fn main() {
            for i in 0..10 {
                println!("i: {}", i) // 0, 1, 2, 3
            }

            for i in (0..20).step_by(2) {
                println!("i: {i}") // 0, 2, 4
            }
        }

        fn loops() {
            // loop can return a value
            let result = loop {
                let input = "10";
                if let Ok(num) = input.parse::<i32>() {
                    break num;
                }
            };
            println!("result: {}", result);
        }

        fn list_comphrehensions_vs_iterator_chains() {
            let squares = (0..10).map(|x| x * x).collect::<Vec<i32>>();
            let evens = (0..20).filter(|num| num % 2 == 0).collect::<Vec<i32>>();
            let pairs = (0..3)
                .flat_map(|x| (0..3).map(move |y| (x, y)))
                .collect::<Vec<(i32, i32)>>();
        }
    }

    mod expressions {
        // everything in rust is expression or can be an expression
        fn main() {
            let result = if 10 % 2 == 0 { "yes" } else { "no" };
            let value = {
                let x = 5;
                let y = 10;
                x + y
            };
            let temprature = 100;
            let description = match temprature {
                t if t > 100 => "boiling",
                t if t > 50 => "hot",
                t if t > 20 => "warm",
                _ => "cold",
            };
        }
    }

    mod func_type_sign {
        use core::num;

        fn greet(name: &str, greeting: &str) -> String {
            format!("{} {}!", greeting, name)
        }

        fn greet_with_default(name: &str, greeting: Option<&std>) -> String {
            let greeting = greeting.unwrap_or("Hello");
            format!("{} {}!", greeting, name)
        }

        fn sum_all(numbers: &[i32]) -> i32 {
            numbers.iter().sum()
        }

        // First call functions and closures
        fn apply(f: fn(i32) -> i32, x: i32) -> i32 {
            f(x)
        }

        fn test_apply() {
            let result = apply(|x| x * 2, 5);
            assert_eq!(result, 10);
        }

        fn min_max(numbers: &[i32]) -> (i32, i32) {
            let min = *numbers.iter().min().unwrap();
            let max = *numbers.iter().max().unwrap();
            (min, max)
        }
    }
    fn fizbuzz() {
        (1..=30).for_each(|num| match num {
            n if n % 3 == 0 => println!("Fizz"),
            n if n % 5 == 0 => println!("Buzz"),
            _ => println!("FizzBuzz"),
        });
    }

    fn fizbuzz_2() {
        (1..=30).for_each(|num| {
            let result = match (num % 3, num % 5) {
                (0, 0) => String::from("FizzBuzz"),
                (0, _) => String::from("Fizz"),
                (_, 0) => String::from("Buzz"),
                _ => num.to_string(),
            };
            println!("{}", result);
        });
    }
}
