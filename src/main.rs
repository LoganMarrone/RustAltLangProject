use std::fs::File;

fn main (){
    //establish mutabile var file for the file that checks if the file is Ok and if there runs an arror
    let mut file = match File::open("./cells.csv"){
        Ok(file) => file,
        Err(ohnoerror) => {
            println!("There was an error reading the file: {}", ohnoerror);
            return;
        }
    };

}