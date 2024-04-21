use std::fs::File;
use std::path::Path;
use std::io::BufRead;
use std::io::BufReader;
#[allow(non_snake_case)]

fn main (){
    //create a class called cell (in rust we have to enumerate to establish variables within a class, initialize the struct and implement the features)
    //Update: enum can be used to establish multiple variable types within one structure's variable like a string. So we can additionally make a variable using a enum to convert the values easily
    //For this program we have multiple values to convert from String to Float. For simplicities sake I made all of their types StrFloat so that they can be both Strings and Integers
    enum StrFloat {
        Strval(String),
        Intval(i64),
        Fltval(f64),
    }
    enum Cellvar {
        Oem(StrFloat),
        Model(StrFloat),
        Launchannounced(StrFloat),
        Launchstatus(StrFloat),
        Bodydimensions(StrFloat),
        Bodyweight(StrFloat),
        Bodysim(StrFloat),
        Displaytype(StrFloat),
        Displaysize(StrFloat),
        Displayresolution(StrFloat),
        Featuressensors(StrFloat),
        Platformos(StrFloat),
    }
    //Set value of the class to the enum
    struct Cell{
    Oem: Option<StrFloat>,
    Model: Option<StrFloat>,
    Launchannounced: Option<StrFloat>,
    Launchstatus: Option<StrFloat>,
    Bodydimensions: Option<StrFloat>,
    Bodyweight: Option<StrFloat>,
    Bodysim: Option<StrFloat>,
    Displaytype: Option<StrFloat>,
    Displaysize: Option<StrFloat>,
    Displayresolution: Option<StrFloat>,
    Featuressensors: Option<StrFloat>,
    Platformos: Option<StrFloat>,}
    //implement the class when it establishes a new instance, gather a value and set a value
    impl Cell{
        //creates a new cell utilizing the enum for the value
        //Update: Has to refer to ones self twice and assign "null" values (none)
        fn new() -> Self { Self {
        Oem: None,
        Model: None,
        Launchannounced: None,
        Launchstatus: None,
        Bodydimensions: None,
        Bodyweight: None,
        Bodysim: None,
        Displaytype: None,
        Displaysize: None,
        Displayresolution: None,
        Featuressensors: None,
        Platformos: None,
        }
    }
        //Gathers value using the & to point at the address (similar to pointer)
        //Update:Due to the fact that it isn't one set value we have to get each and every one using match fields to check the values
        fn get_value(&self, field: &Cellvar) -> Option<&StrFloat> {
            match field{
            Cellvar::Oem(_) => self.Oem.as_ref(),
            Cellvar::Model(_) => self.Model.as_ref(),
            Cellvar::Launchannounced(_) => self.Launchannounced.as_ref(),
            Cellvar::Launchstatus(_) => self.Launchstatus.as_ref(),
            Cellvar::Bodydimensions(_) => self.Bodydimensions.as_ref(),
            Cellvar::Bodyweight(_) => self.Bodyweight.as_ref(),
            Cellvar::Bodysim(_) => self.Bodysim.as_ref(),
            Cellvar::Displaytype(_) => self.Displaytype.as_ref(),
            Cellvar::Displaysize(_) => self.Displaysize.as_ref(),
            Cellvar::Displayresolution(_) => self.Displayresolution.as_ref(),
            Cellvar::Featuressensors(_) => self.Featuressensors.as_ref(),
            Cellvar::Platformos(_) => self.Platformos.as_ref(),
            }
        }
        //Enforces a mutable version of ones self to be able to set the new value
        //Update: Similar to before we have to check the value using match field and use Some to assign the value
        fn set_value(&mut self, field: Cellvar, value: String){
            match field {
            Cellvar::Oem(_) => self.Oem = Some(StrFloat::Strval(value)),
            Cellvar::Model(_) => self.Model = Some(StrFloat::Strval(value)),
            Cellvar::Launchannounced(_) => self.Launchannounced = Some(StrFloat::Strval(value)),
            Cellvar::Launchstatus(_) => self.Launchstatus = Some(StrFloat::Strval(value)),
            Cellvar::Bodydimensions(_) => self.Bodydimensions = Some(StrFloat::Strval(value)),
            Cellvar::Bodyweight(_) => self.Bodyweight = Some(StrFloat::Strval(value)),
            Cellvar::Bodysim(_) => self.Bodysim = Some(StrFloat::Strval(value)),
            Cellvar::Displaytype(_) => self.Displaytype = Some(StrFloat::Strval(value)),
            Cellvar::Displaysize(_) => self.Displaysize = Some(StrFloat::Strval(value)),
            Cellvar::Displayresolution(_) => self.Displayresolution = Some(StrFloat::Strval(value)),
            Cellvar::Featuressensors(_) => self.Featuressensors = Some(StrFloat::Strval(value)),
            Cellvar::Platformos(_) => self.Platformos = Some(StrFloat::Strval(value)),
        }
        }
}
    
    //establish mutabile var file for the file that checks if the file is Ok and if there runs an arror (Wrong thought)
    //Update:Instead of using a mutable var, use a fixed file that has a path since we do not need a mutable variable
    let pathtofile = Path::new("./cells.csv");
    //This acts like a file check to ensure that the csv file can be read
    let _file = match File::open(&pathtofile){
        Ok(file) => file,
        Err(ohnoerror) => {
            println!("There was an error reading the file: {}", ohnoerror);
            return;
        }
        
    };
    //this right here is the actual file reader that assigns it a value/reference in the program
    let readfile = BufReader::new(_file);
    //uses a for loop that sifts through the lines and checks if it is a line.
    for line in readfile.lines() {
    if let Ok(line) = line {
        let parts: Vec<&str> = line.split(',').collect();
        let mut cell = Cell::new();

        // Iterate over each part and assign the value if it exists
        for (index, part) in parts.iter().enumerate() {
            let value = if part.is_empty() {
                None // Assign None for empty slots
            } else {
                Some(StrFloat::Strval(part.to_string())) // Assign the value if it exists
            };

            // Match the index to the corresponding field in the Cell struct
            //I realize now that there is a far more easy and productive solution to doing such. This allows a far more simple way to sift through and assign values
            if parts.len() == 12 {
            match index {
                    0 => cell.Oem = value,
                    1 => cell.Model = value,
                    2 => cell.Launchannounced = value,
                    3 => cell.Launchstatus = value,
                    4 => cell.Bodydimensions = value,
                    5 => cell.Bodyweight = value,
                    6 => cell.Bodysim = value,
                    7 => cell.Displaytype = value,
                    8 => cell.Displaysize = value,
                    9 => cell.Displayresolution = value,
                    10 => cell.Featuressensors = value,
                    11 => cell.Platformos = value,
                _ => {}
            }
        }
        else {
        }
        //Checks for output
        /*if let Some(Oem) = &cell.Oem {
        match Oem {
        StrFloat::Strval(s) => println!("Oem: {}", s),
        _ => {}
        }
        }*/
    }
}
}
}
