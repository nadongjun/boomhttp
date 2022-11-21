use std::str;
use colored::Colorize;

pub fn read_input_configfile(path : &str) {
    println!("{} : {}", format!(">> Configuration file path ").bold().green(), path);
    let path = path.replace('"', "");
    //let mut file = std::fs::File::create(path).expect("create failed");

    let result = std::fs::read_to_string(&path);
    match result {
        Ok(content) => { println!("{}", format!("file content: \n{}", content).bold());},
        Err(error) => { println!("{} {} : {}", format!("Not found : ").bold().red() , format!("{}", error).bold().red(), path);}
    }
}

// POST
pub fn create_config_content(path : &str) {

}


// GET
pub fn get_config_content(path : &str) {

}

// PUT
pub fn update_config_content(path : &str) {

}

// DELETE
pub fn delete_config_content(path : &str) {

}


pub fn generate_response(response_code : &str,status : &str) -> String {
    let contents = String::new();
    let response = format!(
        "HTTP/1.1 {} {}\r\nContent-Length: {}\r\n\r\n{}",
        response_code,
        status,
        contents.len(),
        contents
    );

    return response;
}

pub fn parse_url(_url : &str) -> String {
    let contents = String::new();
    let response = format!("{}", contents);
    return response
}

