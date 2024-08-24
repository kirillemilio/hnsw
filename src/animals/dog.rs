use super::animal::Animal;

pub struct Dog {
    age: i32,
    name: String
}

impl Dog {
    pub fn new(age: i32, name: String) -> Dog {
        return Dog{age: age, name: name};
    }
}

impl Animal for Dog {
    fn voice(&self) -> String {
        return String::from("wawf");
    }

    fn get_name(&self) -> String {
        return self.name.clone();
    }
}