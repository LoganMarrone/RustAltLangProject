extern crate csv;
use std::fs::File;
use std::error::Error;
use std::path::Path;


fn main (){
    //create a class called cell (in rust we have to enumerate to establish variables within a class, initialize the struct and implement the features)
    enum Cellvar {
        Oem(String),
        Model(String),
        Launchannounced(String),
        Launchstatus(String),
        Bodydimensions(String),
        Bodyweight(String),
        Bodysim(String),
        Displaytype(String),
        Displaysize(String),
        Displayresolution(String),
        Featuressensors(String),
        Platformos(String),
    }
    //Set value of the class to the enum
    struct Cell{ value: Cellvar}
    //implement the class when it establishes a new instance, gather a value and set a value
    impl Cell{
        //creates a new cell utilizing the enum for the value
        fn new(value: Cellvar) -> Self {
            Cell { value }
        }
        //Gathers value using the & to point at the address (similar to pointer)
        fn get_value(&self) -> &Cellvar {
            &self.value
        }
        //Enforces a mutable version of ones self to be able to set the new value
        fn set_value(&mut self, new: Cellvar){
            self.value = new;
        }
    }
    //establish mutabile var file for the file that checks if the file is Ok and if there runs an arror (Wrong thought)
    //Instead of using a mutable var, use a fixed file that has a path since we do not need a mutable variable
    let pathtofile = Path::new("./cells.csv");
    let file = match File::open(&pathtofile){
        Ok(file) => file,
        Err(ohnoerror) => {
            println!("There was an error reading the file: {}", ohnoerror);
            return;
        }
    };

}