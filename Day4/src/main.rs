extern crate bigint;
use std::fmt;
use std::ops::Add;
use bigint::U256;

struct StrDisplayable<'a>(Vec<&'a str>);

#[derive(Debug)]
struct Fibonacci {
    a: U256,
    b: U256,
}

impl Iterator for Fibonacci {
    type Item = U256;

// Trả về số fibonaci tiếp theo dựa trên kiểu dữ liệu struct Fibonacci
    fn next(&mut self) -> Option<Self::Item> {
        let tmp = self.a.add(self.b);
    self.a = self.b;
    self.b = tmp;
    Some(tmp)
    }
}

// Khởi tạo ban đầu cho Fibonaci: 0, 1
fn fibonacci_numbers() -> Fibonacci {
    Fibonacci { a: U256::from(1), b: U256::from(0) }
}


// Bài 2: Lifetime
// Yêu cầu: Sửa lỗi Lifetime


impl<'a> fmt::Display for StrDisplayable<'a> {
    fn fmt(&self, f: &mut  fmt::Formatter) -> fmt::Result {
        for v in &self.0 {
            write!(f, "\n{}", v)?;
        }
        Ok(())
    }
}

fn main() {
    //Bai 1
    let mut count = 0;
    for number in fibonacci_numbers() {
        println!("{}", number);
        if count == 200  {
            break;
        }
        count+=1;
    }

    //Bai2
        let vec: Vec<&str> = vec!["a","bc","def"];
        let vec_Foo = StrDisplayable(vec);
        println!("{}",vec_Foo);
}