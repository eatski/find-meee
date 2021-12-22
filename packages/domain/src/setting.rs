#[derive(Clone)]
pub struct Setting {
    pub hints_num: usize
}

impl Setting {
    pub fn recommend() -> Self {
        Self {
            hints_num: 3
        }
    }
}