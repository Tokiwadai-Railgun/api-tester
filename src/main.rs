use std::fs;
use regex::Regex;
use ureq::serde_json::{to_string, Value};

struct Request {
    title: String,
    method: String,
    url: String,
    body: Option<String>,
    expected_code: u16,
    expected_body: Option<String>
}
fn main() {
    let file_argument: Vec<_> = std::env::args().collect();
    if file_argument.len() < 2 {panic!("Please precise a file as first argument")}


    let requests = read_file_content(format!("./{}", file_argument[1])).unwrap();


    // requesting without client
    requests.iter().for_each(|req| {
        let result  = request(req).unwrap();
        println!("{} - Status : {}", req.title, if result { "Success" } else { "Failed" })
    })
}

fn request(request_data: &Request) -> Result<bool, Box<dyn std::error::Error>> {
    match request_data.method.as_str() {
        "GET" => {
            let status = ureq::get(&request_data.url).call()?.status();
            Ok(status == request_data.expected_code)
        },
        "POST" => {
            if request_data.body.is_none() {panic!("Data must be entered for post requests")};

            // convert body to json
            let json_body: Value = ureq::serde_json::from_str(request_data.body.clone().unwrap().as_str())?;
            let expected_json_body: Value = ureq::serde_json::from_str(request_data.expected_body.clone().unwrap().as_str())?;

            let resp = ureq::post(&request_data.url)
                .set("Content-Type", "application/json")
                .send_json(json_body);
            // println!("Response Body {}", resp?.into_string().unwrap());
            let resp = resp?;
            let status = resp.status();
            let body: Value = resp.into_json()?;
            // println!("Response bodu : {body}");
            Ok(status == request_data.expected_code && body == expected_json_body)
        },
        _ => panic!("Method not supported")
    }
}


fn read_file_content(filepath: String) -> Result<Vec<Request>, std::io::Error> {
    let content = fs::read_to_string(filepath)?;

    let title_regex = Regex::new(r"\[[^\]]+\][^\[]*").unwrap();
    let requests= title_regex.find_iter(&content).map(|m| m.as_str());
    let requests_result: Vec<Request> = requests.filter_map(|req| {
        let mut request_data = req.split("\n");
        let mut first_line = request_data.next();
        while first_line.eq(&Some("")) { first_line = request_data.next() };
        first_line?;
        // Equals to :
        // match first_line {
        //     Some(line) => line,
        //     None => return None
        // };


        let title = String::from(&first_line.unwrap()[1..&first_line.unwrap().len() - 1]);
        let request_line: Vec<&str> = request_data.next().unwrap().split(" ").collect();
        let method = String::from(request_line[0]);
        let url = String::from(request_line[1]);
        let mut body = None;

        // if method is post then get the body from the next line
        if method.eq("POST") {
            let mut line = request_data.next().unwrap().split(' ');
            if !line.next().unwrap().eq("DATA") && line.clone().count() == 2 {panic!("Data must be entered for post requests")};

            body = Some(line.collect::<Vec<_>>().join(" "));
        }

        let expected_code: u16 = request_data.next().unwrap_or("201").parse().unwrap_or(201);
        
        // check if we want to look for the response body
        let expected_body = request_data.next().map(|line| line.to_string()); // If None then the map funciton returns None by default

        Some(Request { title, method, url, expected_code, body, expected_body })
    }).collect();

    Ok(requests_result)
}

fn print_type_of<T>(_: &T) {
    println!("Type is : {}", std::any::type_name::<T>())
}
