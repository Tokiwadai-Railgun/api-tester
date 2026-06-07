use std::{collections::HashMap, fmt::Display};

use ureq::serde_json::{self, from_str};

#[derive(Debug)]
pub enum Method {
    Get,
    Post,
    Put,
    Delete,
}

impl Display for Method {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            Method::Get => write!(f, "GET"),
            Method::Post => write!(f, "POST"),
            Method::Put => write!(f, "PUT"),
            Method::Delete => write!(f, "DELETE"),
        }
    }
}

impl From<&str> for Method {
    fn from(value: &str) -> Self {
        match value {
            "GET" => Method::Get,
            "POST" => Method::Post,
            "PUT" => Method::Put,
            "DELETE" => Method::Delete,
            e => panic!("Invalid method provided : {}", e),
        }
    }
}

#[derive(Debug)]
pub struct TestResult {
    success: bool,
    content: String,
}

impl Display for TestResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

#[derive(Debug)]
pub struct TestCase<'a> {
    pub name: &'a str,
    pub method: Method,
    pub url: &'a str,
    pub body: Option<serde_json::Value>,
    pub expected_status: u16,
    pub expected_response: Option<serde_json::Value>,
    pub headers: Option<HashMap<&'a str, &'a str>>,
    pub store_cookies: bool, // wether to store response cookies or not
    pub result: Option<TestResult>,
}

impl Default for TestCase<'_> {
    fn default() -> Self {
        Self {
            name: Default::default(),
            method: Method::Get,
            url: Default::default(),
            body: Default::default(),
            expected_response: Default::default(),
            headers: Default::default(),
            store_cookies: Default::default(),
            result: Default::default(),
            expected_status: Default::default(),
        }
    }
}

impl Display for TestCase<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "
            Name: {}
            Method: {}
            Url: {}
            Body: {:?}
            Expected Status: {}
            Expected Response: {:?}
            Headers: {:?}
            Store Cookies ?: {}
            ",
            self.name,
            self.method,
            self.url,
            self.body,
            self.expected_status,
            self.expected_response,
            self.headers,
            self.store_cookies
        )
    }
}

enum ReadingMode {
    Json(usize, usize),
    Data,
}

pub fn parse_file(content: &'_ str) -> Vec<TestCase<'_>> {
    // Split lignes par lignes
    // un test commence par "["
    // Ensuite trier par premier mot de la ligne
    // <METHOD> <URL>
    // DATA <DATA>
    // RESPONSE <RESPONSE>
    // HEADERS <HEADERS>

    let mut reading_mode = ReadingMode::Data;

    let mut cases_vec: Vec<TestCase> = Vec::new();
    let mut current_case = TestCase::default();
    let mut json_array: Vec<&str> = Vec::new();

    for line in content.split("\n") {
        if line.starts_with("###") {
            if let ReadingMode::Json(_, _) = reading_mode {
                panic!("Invalid JSON data given")
            }

            if !current_case.name.is_empty() {
                cases_vec.push(current_case);
                current_case = TestCase::default();
            }

            current_case.name = &line[4..] // remove the "### " from the line to get the name
        }

        if line.starts_with("{") {
            reading_mode = ReadingMode::Json(0, 0);
        }
        match reading_mode {
            ReadingMode::Data => {
                if line.is_empty() || line == "\n" {
                    continue;
                }

                // Method + URL
                if line.starts_with("GET")
                    || line.starts_with("POST")
                    || line.starts_with("PUT")
                    || line.starts_with("DELETE")
                {
                    let (method, url) = extract_name_url(line);
                    current_case.method = method;
                    current_case.url = url;
                }

                // Headers

                // annotation
                if line.starts_with("#") {
                    if let Some(data) = handle_comment(line) {
                        match data {
                            Anotation::ExpectedStatus(status) => {
                                current_case.expected_status = status
                            }
                            Anotation::ExpectedResponse(_) => {}
                        }
                    }
                }

                // TODO: Check for @notation to know which property to set
            }
            ReadingMode::Json(nb_opened, nb_closed) => {
                json_array.push(line);
                let opening = line.chars().filter(|c| *c == '{').count() + nb_opened;
                let closing = line.chars().filter(|c| *c == '}').count() + nb_closed;

                reading_mode = ReadingMode::Json(opening, closing);

                // Checking if the ended the json
                if line.ends_with("}") && opening == closing {
                    reading_mode = ReadingMode::Data;
                    current_case.body = Some(
                        serde_json::from_str(json_array.join("").as_str()).unwrap_or_else(|_| {
                            panic!("Unable to parse json data for test : {}", current_case.name)
                        }),
                    );
                }
            }
        }
    }

    // Pushing the last case
    cases_vec.push(current_case);

    cases_vec
}

/// Extract the name and the method from the line, returning the two in a tuple
fn extract_name_url(line: &str) -> (Method, &str) {
    let mut iter = line.split(" ");
    let method_name = match iter.next() {
        Some(name) => name,
        None => panic!("Invalid URL structure, please usr <METHOD> <URL>"),
    };
    let url = match iter.next() {
        Some(url) => url,
        None => panic!("Invalid URL structure, please usr <METHOD> <URL>"),
    };

    let method = Method::from(method_name);

    (method, url)
}

enum Anotation<'a> {
    ExpectedStatus(u16),
    ExpectedResponse(&'a str),
}

fn handle_comment(line: &'_ str) -> Option<Anotation<'_>> {
    if !line.contains("@") {
        return None;
    }; // skip regular comments
    let mut iter = line.split(" ");
    let _ = iter.next();
    let anotation = iter.next();

    match anotation {
        Some(value) => {
            match value {
                "@expect-status" => {
                    if let Some(value) = iter.next() {
                        let number_status: u16 =
                            from_str(value).expect("Invalid status provided : not a number");
                        Some(Anotation::ExpectedStatus(number_status))
                    } else {
                        panic!("Invalid anotation, usage : @expected-status <status>")
                    }
                }
                "@expect-response" => {
                    todo!("Still need to be implemented")
                }
                _ => None, // considering a simple regular comment with an @ here
            }
        }
        None => None, // considering regular comment with @ here
    }
}
