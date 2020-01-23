extern crate ureq;
extern crate colored;

use colored::*;
//use ureq::json;

fn main() {
    let mut args: Vec<String> = std::env::args().collect();
    let base_url = "http://0.0.0.0:3000/";
    match args.len() {
        1 => {
            let resp = ureq::get(base_url).call();
            println!("{}: {}", resp.status(), resp.status_text());
        },
        _ =>  {
            args[0] = "".to_string();
            for url in args {
                println!("-8<------8<------");
                let resp = ureq::get(&format!("{}{}", base_url, url)).call();
                println!("page:\n\t {}",  format!("{}{}", base_url, url).cyan());
                println!("response:");
                let resp = if resp.ok() {
                    format!("{}: {}", resp.status(), resp.status_text()).green()
                } else {
                    format!("{}: {}", resp.status(), resp.status_text()).red()
                };
                println!("\t{}", resp);
            }
            println!("-8<------8<------");
        }

    }
}
