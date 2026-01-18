pub(crate) fn echo (args: &[String]){

    let mut start_idx=0;

    if let Some(argument)=args.get(0) {
        if argument == "-n" || argument == "--no-newline" {
            start_idx=1;
        }
    }

    for arg in args[start_idx..].iter() { 
        print!("{} ", arg);

        if start_idx == 0 {
            println!();
        }
    }

}