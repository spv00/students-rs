use derive_getters::Getters;

#[derive(PartialEq, Debug, Getters)]
pub struct Grade {
    grade: i32,
    min: i32,
    max: i32,
}

impl Grade {
    pub fn new(grade: i32, min: i32, max: i32) -> Self {
        Grade { grade, min, max }
    }
}
