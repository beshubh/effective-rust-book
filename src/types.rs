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

    fn idiomatic_errors() {
        use std::error::Error;

        #[derive(Debug)]
        pub struct MyError(String);

        impl std::fmt::Display for MyError {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{}", self.0)
            }
        }

        impl std::error::Error for MyError {}

        impl From<String> for MyError {
            fn from(value: String) -> Self {
                MyError(value)
            }
        }

        pub fn find_user(username: &str) -> Result<String, MyError> {
            let f = std::fs::File::open("/etc/passwd")
                .map_err(|e| format!("Failed to open password file: {:?}", e))?;
            unimplemented!()
        }

        #[derive(Debug)]
        pub enum CustomFileError {
            Io(std::io::Error),
            Utf8(std::string::FromUtf8Error),
            General(String),
        }

        impl std::fmt::Display for CustomFileError {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                match self {
                    CustomFileError::Io(e) => write!(f, "IO Error: {}", e),
                    CustomFileError::Utf8(e) => write!(f, "UTF-8 Error: {}", e),
                    CustomFileError::General(e) => write!(f, "General error: {}", e),
                }
            }
        }

        impl Error for CustomFileError {
            fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
                match self {
                    CustomFileError::Io(e) => Some(e),
                    CustomFileError::Utf8(e) => Some(e),
                    CustomFileError::General(_) => None,
                }
            }
        }

        impl From<std::string::FromUtf8Error> for CustomFileError {
            fn from(value: std::string::FromUtf8Error) -> Self {
                Self::Utf8(value)
            }
        }

        impl From<std::io::Error> for CustomFileError {
            fn from(value: std::io::Error) -> Self {
                Self::Io(value)
            }
        }

        use std::io::BufRead;

        const MAX_LEN: usize = 1024;

        pub fn first_line(filename: &str) -> Result<String, CustomFileError> {
            // let file = std::fs::File::open(filename).map_err(CustomFileError::Io)?;
            let file = std::fs::File::open(filename)?; // after From impl
            let mut reader = std::io::BufReader::new(file);

            let mut buf = vec![];
            let len = reader.read_until(b'\n', &mut buf)?; // After From
            let result = String::from_utf8(buf)?; // After From impl 
            if result.len() > MAX_LEN {
                return Err(CustomFileError::General(format!("Line too long: {}", len)));
            }
            Ok(result)
        }

        #[derive(Debug)]
        enum WrappedError {
            Wrapped(Box<dyn Error>),
            General(String),
        }

        impl std::fmt::Display for WrappedError {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                match self {
                    Self::Wrapped(e) => write!(f, "Inner error: {}", e),
                    Self::General(e) => write!(f, "{}", e),
                }
            }
        }
    }
}

mod type_conversions {
    fn main() {
        /// Integer value from an IANA-controlled range.
        #[derive(Copy, Debug, Clone)]
        pub struct IanaAllocated(pub u64);

        /// Indicated whether the value is reserved.
        pub fn is_iana_reserved(s: IanaAllocated) -> bool {
            s.0 == 0 || s.0 == 65535
        }

        let s = IanaAllocated(1);
        assert_eq!(is_iana_reserved(s), false);

        impl From<u64> for IanaAllocated {
            fn from(value: u64) -> Self {
                Self(value)
            }
        }

        // DOES NOT WORKS
        // assert_eq!(is_iana_reserved(64), false);
        fn is_iana_reserved_generic<T>(s: T) -> bool
        where
            T: Into<IanaAllocated>,
        {
            let s = s.into();
            s.0 == 0 || s.0 == 65535
        }

        // WORKS for both u64 and IanaAllocated direct types
        assert_eq!(is_iana_reserved_generic(65), false);
        assert_eq!(is_iana_reserved_generic(IanaAllocated(64)), false);

        // Casts

        let x: u32 = 9;
        let y = x as u64;
        let z: u64 = x.into();

        let i = 900_u64;
        let j = i as u32; // lossy conversion

        let x: u32 = 9;
        let y = x as u16;
        // this won't work
        // let y: u16 = x.into();
        
    }
}
