#[derive(PartialEq, Debug)]
pub struct Student {
    name: String,
    points: i32,
}

impl Student {
    fn new(name: String, points: i32) -> Self {
        Student { name, points }
    }
}
