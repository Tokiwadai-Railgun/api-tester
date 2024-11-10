use std::fs;
use regex::Regex;

struct Request {
    title: String,
    method: String,
    url: String,
    expected_code: u16
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

fn request(request_data: &Request) -> Result<bool, Box<ureq::Error>>{
    let status = ureq::get(&request_data.url).call()?.status();
    Ok(status == request_data.expected_code)
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
        let expected_code: u16 = request_data.next().unwrap_or("201").parse().unwrap_or(201);

        Some(Request { title, method, url, expected_code})
    }).collect();

    Ok(requests_result)
}

fn print_type_of<T>(_: &T) {
    println!("Type is : {}", std::any::type_name::<T>())
}
