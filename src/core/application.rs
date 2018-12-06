//user MUST declare #![no_main] in their main.rs
//User MUST implement this function for a struct serving as their app
//#[no_mangle]
//pub fn create_application() -> impl Box<Application>;

pub trait Application {

    fn run(&self) -> () {

        debug!("Application {} Started", self.get_name());
        loop {

        }
    }

    fn get_name(&self) -> String {
        String::from("Application")
    }
}

