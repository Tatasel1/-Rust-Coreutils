use chrono::Local;
use chrono::Utc;


pub(crate) fn date(args: &[String]){

    let mut utc=false;

    for arg in args{
        if arg == "-u" || arg == "--utc"{
            utc=true;
            break;
        }
    }


    if utc {
        let now_utc=Utc::now();
        let utc_formatted=now_utc.format("%Y-%m-%d %H:%M:%Z");
        println!("{}",utc_formatted);
    }
    else {
        let now_local=Local::now();
        let local_formatted = now_local.format("%Y-%m-%d %H:%M:%Z");

        println!("{}",local_formatted);
    }



}