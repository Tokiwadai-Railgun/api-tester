use core::panic;
use std::fs;
use ureq::serde_json::{self, Value};

use crate::{
    color::Colorize,
    test_case::{parse_file, Method, TestCase},
};

mod color;
mod test_case;

fn main() {
    let file_argument: Vec<_> = std::env::args().collect();
    if file_argument.len() < 2 {
        panic!("Please precise a file as first argument")
    }

    match fs::read_to_string(format!("./{}", file_argument[1])) {
        Ok(content) => {
            let cases = parse_file(&content);
            for case in cases.iter() {
                let test_result = request(case).unwrap_or_else(|e| {
                    panic!(
                        "Error querrying data for test : {}\n Error : {}",
                        case.name, e
                    )
                });
                match test_result {
                    (true, _) => println!("{} - Status : {}", case.name, "Success".fg_green()),
                    (false, response_data) => {
                        println!("{} - Status : {}", case.name, "Failed".fg_red());
                        if case.expected_status != response_data.status {
                            println!(
                                "\tStatus code mismatch : {} -> {}",
                                case.expected_status,
                                response_data.status.to_string().fg_red()
                            );
                        } else {
                            let string_body =
                                // a bit too much of unwrap here ......
                                serde_json::to_string(&response_data.body.unwrap()).unwrap().fg_red();
                            println!(
                                "\tResponse Mismatch : {} -> {}",
                                case.expected_response.as_ref().unwrap(),
                                string_body
                            )
                        }
                    }
                }
            }
        }
        Err(e) => {
            panic!("An error occured : {}", e);
        }
    };
}

struct Response {
    status: u16,
    body: Option<Value>,
}

fn request(test_case: &TestCase) -> Result<(bool, Response), Box<dyn std::error::Error>> {
    match test_case.method {
        Method::Get => {
            let req = ureq::get(test_case.url).call()?;
            let status = req.status();

            if let Some(expected_response) = &test_case.expected_response {
                let body: Value = req.into_json()?;
                let return_data = Response {
                    status,
                    body: Some(body.clone()),
                };
                Ok((
                    status == test_case.expected_status && body == *expected_response,
                    return_data,
                ))
            } else {
                let return_data = Response { status, body: None };
                Ok((status == test_case.expected_status, return_data))
            }
        }
        Method::Post => {
            let response = match &test_case.body {
                Some(body) => ureq::post(test_case.url)
                    .set("Content-Type", "application/json")
                    .send_json(body)?,
                None => ureq::post(test_case.url).call()?,
            };

            let status = response.status();
            if let Some(expected_response) = &test_case.expected_response {
                let response_body: Value = response.into_json()?;
                let return_data = Response {
                    status,
                    body: Some(response_body.clone()),
                };

                Ok((
                    status == test_case.expected_status && response_body == *expected_response,
                    return_data,
                ))
            } else {
                let return_data = Response { status, body: None };
                Ok((status == test_case.expected_status, return_data))
            }
        }
        _ => panic!("Method not supported"),
    }
}

fn print_type_of<T>(_: &T) {
    println!("Type is : {}", std::any::type_name::<T>())
}
