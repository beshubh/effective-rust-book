#[unsafe(no_mangle)]
pub extern "C" fn raw_ffi_function() -> i32 {
    match std::panic::catch_unwind(|| 42) {
        Ok(res) => res,
        Err(_) => -1, // return error code instead of panicking into C/Python code
    }
}

unsafe extern "C" {
    fn abs(input: i32) -> i32;
}

fn main() {
    let result = unsafe { abs(-42) };
    println!("{result}")
}

use rstest::{fixture, rstest};

fn add(a: i32, b: i32) -> i32 {
    a + b
}

#[rstest]
#[case(1, 2, 3)]
#[case(0, 0, 0)]
#[case(-1, -1, -2)]
#[case(100, 200, 300)]
fn test_add(#[case] a: i32, #[case] b: i32, #[case] expected: i32) {
    assert_eq!(add(a, b), expected)
}

// like pytest fixture
#[fixture]
fn sample_data() -> Vec<i32> {
    vec![1, 2, 3, 4, 5]
}

#[rstest]
fn test_sum(sample_data: Vec<i32>) {
    assert_eq!(sample_data.iter().sum::<i32>(), 15);
}

use mockall::{automock, predicate::*};

pub struct User {
    name: String,
}

#[automock] // Generates MockDatabase automatically
trait Database {
    fn get_user(&self, id: i64) -> Option<User>;
}

fn fetch_user_name(db: &dyn Database, id: i64) -> Option<String> {
    db.get_user(id).map(|u| u.name)
}

#[test]
fn test_fetch_user() {
    let mut mock = MockDatabase::new();
    mock.expect_get_user()
        .with(eq(1)) // assert_called_with(1)
        .times(1) // assert_called_once
        .returning(|_| {
            Some(User {
                name: "Alice".into(),
            })
        });
    let result = fetch_user_name(mock, 1);
    assert_eq!(result, Some("Alice".to_string()))
}

// Challenge: Write a safe function split_at_mid that takes a &mut [i32] and returns two mutable slices (&mut [i32], &mut [i32])
// split at the midpoint. Internally, use unsafe with raw pointers (simulating what split_at_mut does). Then wrap it in a safe API.
fn split_at_mid(numbers: &mut [i32]) -> (&mut [i32], &mut [i32]) {
    let len = numbers.len();
    let mid = len / 2;
    let ptr = numbers.as_mut_ptr();
    assert!(mid <= len);
    unsafe {
        (
            std::slice::from_raw_parts_mut(ptr, mid),
            std::slice::from_raw_parts_mut(ptr.add(mid), len - mid),
        )
    }
}
