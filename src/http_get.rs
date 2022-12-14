use std::io::Error;
use std::{fs};

pub fn get_aoc_day_data(day: &str) -> String {
    let data = match get_day_file_data(day) {
        Ok(data) => data,
        Err(_) => {
            let client = reqwest::blocking::Client::new();

            let response = match client
                .get(get_aoc_url(day))
                .header("cookie", get_cookie())
                .send()
            {
                Ok(resp) => match resp.status() {
                    reqwest::StatusCode::OK => resp.text(),
                    reqwest::StatusCode::BAD_REQUEST => panic!("Your cookie is malformed"),
                    reqwest::StatusCode::NOT_FOUND => panic!("Request url not found, check url"),
                    _ => panic!("Request failed unexpectedly, check your cookie.txt"),
                },
                Err(e) => panic!("Unknown error while calling aoc server: {:?}", e),
            };

            let data = match response {
                Ok(data) => data,
                Err(e) => panic!("Unknown error while convertin response: {:?}", e),
            };

            let inputs_path = get_inputs_directory_path();    

            fs::create_dir_all(inputs_path).unwrap(); 

            match fs::write(get_day_file_path(day), &data) {
                Ok(file) => println!("File {:?} saved succesfully", file),
                Err(e) => println!("An error occured: {:?}", e),
            };
            data
        }
    };

    return data;
}

fn get_day_file_data(day: &str) -> Result<String, Error> {
    return fs::read_to_string(get_day_file_path(day));
}

fn get_cookie() -> String {
    let mut cookie_path = std::env::current_dir()
        .unwrap()
        .to_str()
        .unwrap()
        .to_owned();

    cookie_path.push_str("/cookie.txt");

    let cookie = match fs::read_to_string(cookie_path) {
        Ok(cookie) => cookie,
        Err(e) => panic!("Cannot read cookie from location: {:?}", e),
    };

    return cookie;
}

fn get_aoc_url(day: &str) -> String {
    let mut url: String = "https:adventofcode.com/2022/day/".to_owned();
    url.push_str(day);
    url.push_str("/input");

    return url;
}

fn get_inputs_directory_path() -> String {
    let mut inputs_path = std::env::current_dir()
        .unwrap()
        .to_str()
        .unwrap()
        .to_owned();

    inputs_path.push_str("/inputs");

    return inputs_path;
}

fn get_day_file_path(day: &str) -> String {
    let mut day_path = get_inputs_directory_path().to_owned();

    day_path.push_str("/day");
    day_path.push_str(day);
    day_path.push_str(".txt");

    return day_path;
}
