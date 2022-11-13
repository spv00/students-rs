#![allow(unused)]

use std::collections::HashMap;

mod models;
use models::{grade::*, student::*};

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
