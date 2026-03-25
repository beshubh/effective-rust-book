mod ownership {

    fn main() {
        let x = 43;
        let y = x; // copied
        println!("{x}, {y}");

        let s1 = String::from("hello");
        let s2 = s1; // s1 is MOVED
        // println!("{s1}") // won't work

        let s1 = String::from("hello");
        let s2 = s1.clone();
    }

    // we need to manually add lifetimes
    fn longest<'a>(a: &'a str, b: &'a str) -> &'a str {
        if a.len() > b.len() { a } else { b }
    }

    mod smart_pointers {
        use std::rc::Rc;

        // when single ownership is too restrictive you can manually opt-in to reference counting for multiple ownership

        fn main() {
            let boxed = Box::new(42);
            let shared = Rc::new(vec![1, 2, 3]);
            let clone1 = Rc::clone(&shared); // increment ref counting
            let clone2 = Rc::clone(&shared); // increment ref counting

            // Arc<T> for multi thread
            use std::cell::RefCell;
            use std::sync::Arc;

            let thread_safe = Arc::new(vec![1, 2, 3]);

            let cell = RefCell::new(42);
            *cell.borrow_mut() = 55;
            *cell.borrow_mut() = 29;
        }
    }

    mod excercise {
        fn main() {
            let mut names = vec!["Alice".to_string(), "Bob".to_string()];
            let first = &names[0];
            println!("First: {first}");
            names.push("Charlie".to_string());

            let greeting = make_greeting(&names[0]);
            println!("{greeting}");
        }

        fn make_greeting(name: &str) -> String {
            format!("Hello, {name}!")
        }
    }
}
