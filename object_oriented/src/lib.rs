pub struct AveragedCollection {
    list: Vec<i32>,
    average: f64,
}

// We implement add, remove and average on struct 
// so that whenever a value is added or removed from the list, the average is also updated

impl AveragedCollection {
    pub fn add(&mut self, value: i32) {
        self.list.push(value);
        self.update_average();        
    }

    pub fn remove(&mut self) -> Option<i32> {
        let result = self.list.pop();
        match result {
            Some(value) => {
                self.update_average();
                Some(value)
            }
            None => None,
        }
    }

    pub fn average(&self) -> f64 {
        self.average
    }

    pub fn update_average(&mut self) {
        let total: i32 = self.list.iter().sum();
        self.average = total as f64 / self.list.len() as f64;
    }
}

pub trait Draw {
    fn draw(&self);
}

// Screen holds a vector components of type Box<dyn Draw>, a trait object
// it’s a stand-in for any type inside a Box that implements the Draw trait.

pub struct Screen<T: Draw> {
    pub components: Vec<T>,
}

impl<T> Screen<T> 
where
    T:Draw,
{
    pub fn run(&self) {
        for component in self.components.iter() {
            component.draw();
        }
    }
}

pub struct Button {
    pub width: u32,
    pub height: u32,
    pub label: String,
}

impl Draw for Button {
    fn draw(&self) {
        // code to actually draw a button
    }
}
