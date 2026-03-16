#[cfg(test)]
mod tests {
    use std::{
        collections::{HashMap, HashSet},
        fmt::{Debug, Display},
    };

    #[test]
    fn char_type() {
        let a = 'a' as u32;
        assert_eq!(a, 97);
        let a = char::from_u32(97);
        assert_eq!(a, Some('a'));

        let a = unsafe { char::from_u32_unchecked(1000) };
        println!("a: {}", a);
    }

    #[test]
    fn agg() {
        struct TextMatch(usize, String);
        let m = TextMatch(12, "needle".to_string());
        assert_eq!(m.0, 12)
    }

    #[test]
    fn enum_type() {
        enum HttpStatusCode {
            Ok = 200,
            NotFound = 404,
            Teapot = 418,
        }
        let status = HttpStatusCode::Ok;
        match status {
            HttpStatusCode::Ok => println!("Ok"),
            HttpStatusCode::NotFound => print!("not found"),
            _ => {}
        }

        fn print_bad(color: bool, black_and_white: bool) {
            unimplemented!()
        }
        enum Color {
            BlackAndWhite,
            Color,
        }
        enum Sides {
            Both,
            Single,
        }

        fn print_good(side: Sides, color: Color) {
            match color {
                Color::BlackAndWhite => println!("Black and white"),
                Color::Color => print!("colored"),
            }
        }
        struct Job;
        enum SchedulerStatus {
            Inert,
            Pending(HashSet<Job>),
            Running(HashMap<usize, Vec<Job>>),
        }

        let can_be_empty = Option::Some("haystack".to_string());
    }

    #[test]
    fn func_pointers() {
        fn sum(x: i32, y: i32) -> i32 {
            x + y
        }
        let op: fn(i32, i32) -> i32 = sum;

        let op1 = op;
        let op2 = op;
        assert!(op1 == op2);
        println!("op: {:p}", op);

        assert_eq!(op(10, 20), 30);

        fn modify_all(data: &mut [u32], mutator: fn(u32) -> u32) {
            for value in data {
                *value = mutator(*value)
            }
        }

        fn add2(v: u32) -> u32 {
            v + 2
        }
        let mut data = vec![1, 2, 3];
        modify_all(&mut data, add2);
        assert_eq!(data, vec![3, 4, 5]);

        let amount_to_add = 3;
        let addn = |v: u32| -> u32 { v + amount_to_add };

        fn modify_all2<F>(data: &mut [u32], mut mutator: F)
        where
            F: FnMut(u32) -> u32,
        {
            for value in data {
                *value = mutator(*value)
            }
        }
        let mut data = vec![1, 2, 3];
        modify_all2(&mut data, addn);
        assert_eq!(data, vec![4, 5, 6]);

        // how closures captures the context
        let am_to_add = 3;
        struct InternalContext<'a> {
            amount_to_add: &'a u32,
        }

        impl<'a> InternalContext<'a> {
            fn internal_op(&self, y: u32) -> u32 {
                // body of lambda expression
                y + *self.amount_to_add
            }
        }
        let add_n = InternalContext {
            amount_to_add: &am_to_add,
        };
        let z = add_n.internal_op(5);
        assert_eq!(z, 8);
    }

    // trait bounds
    fn multiply<T>(x: T, y: T) -> T
    where
        T: std::ops::Mul<Output = T> + Copy + Debug + Display,
    {
        println!("{} * {} = {}", x, y, x * y);
        x * y
    }

    trait Node {
        fn broadcast(&self) -> ();
    }

    // trait object
    fn notify_all(nodes: Vec<Box<dyn Node>>) {
        nodes.iter().for_each(|node| node.broadcast());
    }

    #[test]
    fn option_and_result() {
        struct S {
            field: Option<i32>,
        }
        let s = S { field: Some(10) };
        // bad
        match &s.field {
            Some(f) => println!(" field is: {}", f),
            None => {}
        }
        // good
        if let Some(f) = &s.field {
            println!("field is: {}", f);
        }
        // if you just want to panic
        let result = std::fs::File::open("foo.txt");
        // bad
        let file = match result {
            Ok(f) => f,
            Err(e) => panic!("failed to open file: {}", e),
        };
        // good
        let result = std::fs::File::open("foo.txt").unwrap();

        fn find_user(username: &str) -> Result<String, String> {
            let f = std::fs::File::open("users.db")
                .map_err(|e| format!("failed to find the user: {}", e))?;
            Ok("some".into())
        }

        fn encrypt(payload: &[u8]) -> Vec<u8> {
            unimplemented!()
        }

        struct InputData {
            payload: Option<Vec<u8>>,
        }
        impl InputData {
            fn encrypt(&self) -> Vec<u8> {
                encrypt(self.payload.as_ref().unwrap_or(&vec![]))
            }
}
    }
}
