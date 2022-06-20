use std::io;

fn receive_input() -> Vec<u32> {
    let mut str = String::new();
    io::stdin().read_line(&mut str).expect("Enter number");
    let array = {
        let init = str.trim().split(',');
        let mut array: Vec<u32> = vec![];
        for x in init {
            let value = x.parse::<u32>().unwrap();
            array.push(value);
        };
        array
    };
    array
}

fn check_sub_array(initial: Vec<u32>, sub: Vec<u32>) -> bool {
    let mut x = 0;
    let mut y = 0;
    while x <  sub.len() {
        while y < initial.len() {
            if &sub[x] == &initial[y] {
                x+=1;
                y+=1;
                break;
            }
            x=0;
            y+=1;
        }
        if y == initial.len(){
            break;
        }
    }
    if x == sub.len() {
        return true;
    }
    false
}

fn main() {
    // no1: Find sub array in the initial arrayzz
    println!("Enter initial array (ex:1,2,3,4): ");
    let initial = receive_input();
    println!("Enter sub array (ex:1,2,3,4): ");
    let sub = receive_input();
    if check_sub_array(initial, sub) {
        println!("Yes");
    } else {
        println!("No");
    }
}