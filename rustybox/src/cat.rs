use std::{fs::File, io::BufReader};
use std::io::BufRead;


pub(crate) fn cat(files: &[String]) {
    for file_name in files {
        let res=File::open(file_name);
        let file= match res {
            Ok(file) => file,

            Err(error) => panic!("{:?}",error),

        };

        let reader = BufReader::new(file);
        let lines = reader.lines();
        
        for line in lines{
            println!("{}",line.unwrap());
            
        }
    }
    

}








