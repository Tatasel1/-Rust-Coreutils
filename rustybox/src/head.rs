use std::{fs::File, io::{ BufReader}};
use std::io::BufRead;

pub(crate) fn head (args:&[String]) {
    
    //initializare n=10 (default 10 linii);
    let mut n=10;
    let mut start_idx = 0;

    if let Some(argument)=args.get(0) {
        if argument == "-n" || argument == "--number-of-lines" {
            start_idx = 2;

            if let Some(number) = args.get(1){
                n = str::parse(number).unwrap();
            }

        }

    }

    for arg in &args[start_idx..] {
         
        let res=File::open(arg);
        let file= match res {
            Ok(file) => file,

            Err(error) => panic!("{:?}",error),

        };

        let reader = BufReader::new(file);
        let lines = reader.lines();
        
        for line in lines.take(n) {
            println!("{}", line.unwrap());
        }
    }
    


    

    

}