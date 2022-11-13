use derive_getters::Getters;

#[derive(PartialEq, Debug, Getters)]
pub struct Student {
    name: String,
    points: i32,
}

impl Student {
    pub fn new(name: String, points: i32) -> Self {
        Student { name, points }
    }
}
