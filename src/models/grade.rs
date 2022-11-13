#[derive(PartialEq, Debug)]
pub struct Grade {
    grade: i32,
    min: i32,
    max: i32,
}

impl Grade {
    fn new(grade: i32, min: i32, max: i32) -> Self {
        Grade { grade, min, max }
    }
}
