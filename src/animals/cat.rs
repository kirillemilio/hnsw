use super::animal::Animal;

pub struct Cat {
    age: i32,
    name: String
}

impl Cat {
    pub fn new(age: i32, name: String) -> Cat {
        return Cat{age: age, name: name};
    }
}

impl Animal for Cat {
    fn voice(&self) -> String {
        return "Meow".to_string();
    }
    fn get_name(&self) -> String {
        return self.name.clone();
    }
}