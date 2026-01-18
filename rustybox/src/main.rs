mod cat;
mod date;
mod echo;
mod env;
mod head;
mod rm;

fn main() {
    let arguments: Vec<String> = std::env::args().collect();

    let command=match arguments.get(1){
        None => panic!("No command"),
        Some(command) => command,
    };


    match command.as_str() {
        "cat" => cat::cat(&arguments[2..]) ,
        "date" => date::date(&arguments[2..]),
        "echo" => echo::echo(&arguments[2..]),
        "env" => env::env(),
        "head" => head::head(&arguments[2..]),
        "rm" => rm::rm(&arguments[2..]),
        _ => panic!("Unknown command"),
    }

}