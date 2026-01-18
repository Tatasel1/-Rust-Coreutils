use std::fs;

pub(crate) fn rm(args: &[String]){

    let mut start_idx = 0;
    let mut is_rec=false;
    let mut is_dir=false;

    if let Some(argument) = args.get(0) {
        if argument == "-d" || argument == "--dir" {
            start_idx=1;
            is_rec=false;
            is_dir=true;
        }
        else if argument == "-r" || argument == "--recursive" {
            start_idx=1;
            is_rec=true;
            is_dir=true;
        }
    }

    for arg in &args[start_idx..] {
        
        let meta = match fs::metadata(arg) {

            Ok(meta) => meta,
            Err(error) => panic!("{:?}", error),

        };

       /*  if meta.is_dir() {
            if is_dir {
                if is_rec {
                    match fs::remove_dir_all(arg) {
                        Ok(_) => println!("Deleted successfully"),
                        Err(err) => println!("{:?}", err),
                    }
        } else {
            match fs::remove_dir(arg) {
                Ok(_) => println!("Deleted successfully"),
                Err(err) => println!("{:?}", err),
            }
        }
    } else {
        println!("{} is a directory. Use -d or -r to remove directories.", arg);
    }
} else {
    if is_rec {
        println!("{} is a file. -r can only be used with directories.", arg);
    } else {
        match fs::remove_file(arg) {
            Ok(_) => println!("File deleted successfully"),
            Err(err) => println!("{:?}", err),
        }
    }
}*/

        
       if meta.is_dir() {
            match fs::read_dir(arg){
                Ok(mut entries) => {
                    let is_empty = entries.next().is_none();

                    if is_empty && is_rec==false {
                        match fs::remove_dir(arg) {
                        Ok(_) => println!("Deleted succesfully"),
                        Err(err) => println!("{:?}",err),

                        }
                    }
                    else{
                        match fs::remove_dir_all(arg){
                        Ok(_) => println!("Deleted recursively succesfully"),
                        Err(err) => println!("{:?}",err),
                        }
                    }
                }
           
                Err(err) => println!("{:?}",err),
              
        }
    }
        else {
         if is_rec {
                println!("{} is a file. -r can only be used with directories.", arg);
        } else if is_rec == false && is_dir ==true {
            println!("{} is a file. -d can only be used with directories.", arg);
        
        }
        else if is_rec == false && is_dir == false {
            match fs::remove_file(arg) {
            Ok(_) => println!("File deleted successfully"),
            Err(err) => println!("{:?}", err),
            }
        }
          
    }
}

}


