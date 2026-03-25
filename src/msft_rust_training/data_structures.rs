mod ds {

    mod arrays_and_slices {
        use core::num;

        fn main() {
            //1. Array - fixed size, stack allocated
            let numbers: [i32; 5] = [1, 2, 3, 4, 5]; // size is part of the type;
            // numbers.push(6); // X arrays can't grow
            let zeroes = [0; 10]; // initialize with all same elements

            //2. Slice type, a view into the array or a vec
            let slice: &[i32] = &numbers[1..4];

            let data = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
            let first_three = &data[..3]; // &[i32] view
            let last_two = &data[3..];

            let last_two = &data[data.len() - 2..];

            let reversed = data.iter().rev().collect::<Vec<&i32>>();
            let reversed_copy: Vec<i32> = data.iter().rev().copied().collect();

            // 3. Vec, growable heap allocated array
            let mut numbers = vec![1, 2, 3];
            numbers.push(4);

            let emtpy: Vec<i32> = Vec::new();
            let repeated = vec![0; 10];

            let from_range: Vec<i32> = (0..10).collect();
            // extend
            numbers.extend([5, 6]);

            // pop the last element
            let last = numbers.pop();

            // inplace sort
            numbers.sort();

            let mut sorted_copy = numbers.clone();
            sorted_copy.sort();
            sorted_copy.reverse();

            let contains = numbers.contains(&10);
            let index = numbers.iter().position(|&x| x == 3);
        }
    }

    mod traits {
        use std::fmt::format;

        trait Animal {
            fn name(&self) -> &str;
            fn speak(&self) -> String;
        }

        struct Dog {
            name: String,
        }
        struct Cat {
            name: String,
        }

        impl Animal for Cat {
            fn name(&self) -> &str {
                &self.name
            }
            fn speak(&self) -> String {
                format!("{} says Meow!", self.name)
            }
        }
        impl Animal for Dog {
            fn name(&self) -> &str {
                &self.name
            }
            fn speak(&self) -> String {
                format!("{} says Woof!", self.name)
            }
        }
    }

    mod hashmaps {
        use std::collections::hash_map::HashMap;

        fn main() {
            let scores = HashMap::from([("Alice", 30), ("Bob", 45)]);
            let empty: HashMap<i32, i32> = HashMap::new();
            let from_pairs = [("a", 10), ("b", 20)]
                .into_iter()
                .collect::<HashMap<&str, i32>>();
            let keys = [10, 20, 30];
            let values = ["a", "b", "c"];
            let comprehension: HashMap<_, _> = keys.iter().zip(values.iter()).collect();

            let mut d = HashMap::new();
            d.insert("a", 1);
            d.insert("b", 2);

            d.remove("b");

            let exists = d.contains_key("a");
            let keys: Vec<_> = d.keys().collect();
            let values: Vec<_> = d.values().collect();

            let mut word_count: HashMap<&str, i32> = HashMap::new();
            let words = ["hellow", "hellow", "hell", "a", "b"];
            for word in words {
                *word_count.entry(word).or_insert(0) += 1;+
            }
        }

        fn excercise() {
            fn word_freq(sentence: &str) -> HashMap<&str, usize>{
                let mut result  = HashMap::new();

                for word in sentence.split_whitespace() {
                    *result.entry(word).or_insert(0) += 1;
                }
                result

            }
        }
    }
}
