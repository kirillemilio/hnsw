pub use super::super::animals::dog::Dog;
pub use super::super::animals::animal::Animal;

pub struct DogHouse {
    dogs: Vec<Dog>
}

impl DogHouse {
    pub fn new() -> DogHouse {
        return DogHouse{dogs: Vec::<Dog>::new()}
    }

    pub fn add(&mut self, dog: Dog) -> &mut Self {
        self.dogs.push(dog);
        return self;
    }

    pub fn print(&self) {
        for dog in self.dogs.iter() {
            println!("Dog '{}' in doghouse", dog.get_name())
        }
    }
}