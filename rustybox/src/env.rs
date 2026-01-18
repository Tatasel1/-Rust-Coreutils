pub(crate) fn env() {
    let vars= std::env::vars();

    for var in vars {
        println!("{} = {}", var.0,var.1);
    }
}