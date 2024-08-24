use super::super::animal::Animal;

pub struct Kitten {
    age: i32,
    name: String
}

impl Kitten {
    pub fn new(age: i32, name: String) -> Kitten {
        return Kitten{age: age, name: name};
    }
}


impl Animal for Kitten {
    fn voice(&self) -> String {
        return "Meow-meow".to_string();
    }

    fn get_name(&self) -> String {
        return self.name.clone();
    }
}