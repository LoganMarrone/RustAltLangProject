use std::fs::File;
use std::path::Path;
use std::io::BufRead;
use std::io::BufReader;


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
    for line in readfile.lines(){
        if let Ok(line) = line {
            //Split them by comma using a string vector that collects the values and assigns them to parts
            let parts: Vec<&str> = line.split(',').collect();
            //create a new cell to 
            let mut cell = Cell::new();
            //Assign attributes properly and check if it is there
            if let Some(Oem) = parts.get(0){
                cell.Oem = Some(StrFloat::Strval(Oem.to_string()));
            }
            else{
                cell.Oem = None;
            }
            if let Some(Model) = parts.get(1){
                cell.Model = Some(StrFloat::Strval(Model.to_string()));
            }
            else{
                cell.Model = None;
            }
            if let Some(Launchannounced) = parts.get(2){
                cell.Launchannounced = Some(StrFloat::Strval(Launchannounced.to_string()));
            }
            else{
                cell.Launchannounced = None;
            }
            if let Some(Launchstatus) = parts.get(3){
                cell.Launchstatus = Some(StrFloat::Strval(Launchstatus.to_string()));
            }
            else{
                cell.Launchstatus = None;
            }
            if let Some(Bodydimensions) = parts.get(4){
                cell.Bodydimensions = Some(StrFloat::Strval(Bodydimensions.to_string()));
            }
            else{
                cell.Bodydimensions = None;
            }
            if let Some(Bodyweight) = parts.get(5){
                cell.Bodyweight = Some(StrFloat::Strval(Bodyweight.to_string()));
            }
            else{
                cell.Bodyweight = None;
            }
            if let Some(Bodysim) = parts.get(6){
                cell.Bodysim = Some(StrFloat::Strval(Bodysim.to_string()));
            }
            else{
                cell.Bodysim = None;
            }
            if let Some(Displaytype) = parts.get(7){
                cell.Displaytype = Some(StrFloat::Strval(Displaytype.to_string()));
            }
            else{
                cell.Displaytype = None;
            }
            if let Some(Displaysize) = parts.get(8){
                cell.Displaysize = Some(StrFloat::Strval(Displaysize.to_string()));
            }
            else{
                cell.Displaysize = None;
            }
            if let Some(Displayresolution) = parts.get(9){
                cell.Displayresolution = Some(StrFloat::Strval(Displayresolution.to_string()));
            }
            else{
                cell.Displayresolution = None;
            }
            if let Some(Featuressensors) = parts.get(10){
                cell.Featuressensors = Some(StrFloat::Strval(Featuressensors.to_string()));
            }
            else{
                cell.Featuressensors = None;
            }
            if let Some(Platformos) = parts.get(11){
                cell.Platformos = Some(StrFloat::Strval(Platformos.to_string()));
            }
            else{
                cell.Platformos = None;
            }
        }
    }
}