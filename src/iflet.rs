use std::sync::Mutex;

#[derive(Debug)]
struct Thing;

impl Thing {
    fn do_something(&self) {}
}

pub struct Resource {
    x: Mutex<Option<Thing>>,
}

impl Resource {
    pub fn iflet_temporary_lifetimes(&self) {
        if let Some(x) = self.x.lock().unwrap().as_ref() {
            x.do_something();

            // In the 2024 edition the guard will be dropped here.
        } else {
            println!("oops!");

            // In the 2021 edition the guard will be dropped here.
        }
    }
}
