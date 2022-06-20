use std::borrow::Borrow;
use std::collections::HashMap;

pub struct School<T> {
    students: HashMap<String,T>
}

impl<T:Copy+Clone+ std::cmp::Ord> School<T> {
    pub fn new() -> School<T> {
        School{
            students: HashMap::new()
        }
    }

    pub fn add(&mut self, grade: T, student: String) {
        self.students.insert(student.parse().unwrap(), grade);
    }

    pub fn grades(&self) -> Vec<T> {
        let grades = self.students.values();
        let mut result:Vec<T> = Vec::new();
        for x in grades.into_iter(){
            result.push((*x.borrow()).clone());
        }
        result.sort();
        result.dedup();
        result
    }

    pub fn grade(&self, grade: T) -> Vec<String> {
        let mut result: Vec<String> = Vec::new();
        for (k,v) in &self.students {
            if *v == grade {
                result.push(k.parse().unwrap());
            }
        }
        result.sort();
        result
    }
}

fn main() {
    let mut school:School<u32> = School::new();
    school.add(5,String::from("Jade"));
    school.add(6,String::from("Trang"));
    school.add(5,"Alice".to_string());
    school.add(5,"Liam".to_string());
    school.add(7,"Alice".to_string());
    school.add(7,"Kai".to_string());

    println!("{:?}",school.grades());

    println!("{:?}",school.grade(5));

    // let mut school2:School<String> = School::<String>::new();
    // school2.add("A".to_string(),String::from("Jade"));
    // school2.add("B".to_string(),String::from("Heal"));
    // school2.add("B".to_string(),"Alice".to_string());
    // school2.add("B+".to_string(),"Liam".to_string());
    // school2.add("A".to_string(),"Alice".to_string());
    // school2.add("A".to_string(),"Lily".to_string());
    //
    // println!("{:?}",school2.grades());
    //
    // println!("{:?}",school2.grade("A".to_string()));
}
