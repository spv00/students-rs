#![allow(unused)]

use std::collections::HashMap;

#[derive(PartialEq, Debug)]
struct Student {
    name: String,
    points: i32,
}

impl Student {
    fn new(name: String, points: i32) -> Self {
        Student { name, points }
    }
}

#[derive(PartialEq, Debug)]
struct Grade {
    grade: i32,
    min: i32,
    max: i32,
}

impl Grade {
    fn new(grade: i32, min: i32, max: i32) -> Self {
        Grade { grade, min, max }
    }
}

pub fn main() {
    let grades = vec![
        Grade::new(1, 100, 85),
        Grade::new(2, 84, 70),
        Grade::new(3, 69, 60),
        Grade::new(4, 50, 59),
        Grade::new(5, 30, 49),
        Grade::new(6, 0, 29),
    ];

    let student = Student::new("hansi".to_string(), 35);

    for grade in grades {
        if (student.points > grade.min) && (student.points < grade.max) {
            println!("Student {} is grade {}", student.name, grade.grade);
            break;
        }
    }
}
