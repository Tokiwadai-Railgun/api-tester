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
pub struct TestCase<'a> {
    pub name: &'a str,
    pub method: Method,
    pub url: &'a str,
    pub body: Option<serde_json::Value>,
    pub expected_status: u16,
    pub expected_response: Option<serde_json::Value>,
    pub headers: HashMap<&'a str, &'a str>,
    pub save_cookies: bool, // wether to store response cookies or not
    pub use_cookies: bool,
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
            save_cookies: Default::default(),
            expected_status: Default::default(),
            use_cookies: Default::default(),
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
            self.save_cookies
        )
    }
}

enum ReadingMode {
    Json((usize, usize), (usize, usize)),
    Data,
}

pub fn parse_file(content: &'_ str) -> Vec<TestCase<'_>> {
    let mut reading_mode = ReadingMode::Data;

    let mut cases_vec: Vec<TestCase> = Vec::new();
    let mut current_case = TestCase::default();
    let mut json_array: Vec<&str> = Vec::new();

    let mut line_index = 0;

    for line in content.split("\n") {
        if line.starts_with("###") {
            if let ReadingMode::Json(_, _) = reading_mode {
                panic!(
                    "LINE {} : Invalid JSON data given. \n Data : {:?}",
                    line_index, json_array
                )
            }

            if !current_case.name.is_empty() {
                cases_vec.push(current_case);
                current_case = TestCase::default();
            }

            current_case.name = &line[4..] // remove the "### " from the line to get the name
        }

        if line.starts_with("{") {
            reading_mode = ReadingMode::Json((0, 0), (0, 0));
        }
        match reading_mode {
            ReadingMode::Data => {
                if line.is_empty() || line == "\n" {
                    line_index += 1;
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
                    line_index += 1;
                    continue;
                }

                // annotation
                if line.starts_with("#") {
                    if let Some(data) = handle_comment(line) {
                        match data {
                            Anotation::ExpectedStatus(status) => {
                                current_case.expected_status = status
                            }
                            Anotation::ExpectedResponse(first_line) => {
                                let opening_bracket = line.chars().filter(|c| *c == '[').count();
                                let closing_bracket = line.chars().filter(|c| *c == ']').count();

                                let opening_brace = line.chars().filter(|c| *c == '}').count();
                                let closing_brace = line.chars().filter(|c| *c == '}').count();

                                // clear the current json value
                                json_array = Vec::new();
                                json_array.push(first_line);

                                if line.ends_with('}')
                                    && opening_bracket == closing_bracket
                                    && opening_brace == closing_brace
                                {
                                    current_case.expected_response = Some(
                                        serde_json::from_str(json_array.join("").as_str())
                                            .unwrap_or_else(|_| {
                                                panic!(
                                                    "LINE {} : Unable to parse json data for test : {}, {}",
                                                    line_index,
                                                    current_case.name,
                                                    json_array.join("")
                                                )
                                            }),
                                    );
                                } else {
                                    reading_mode = ReadingMode::Json(
                                        (opening_brace, closing_brace),
                                        (opening_bracket, closing_bracket),
                                    );
                                }
                            }
                            Anotation::UseCookies => current_case.use_cookies = true,
                            Anotation::SaveCookies => current_case.save_cookies = true,
                        }
                    }

                    line_index += 1;
                    continue;
                }

                // Headers
                let mut line_split = line.split(':');
                let header_name = line_split.next().unwrap();
                if let Some(header_value) = line_split.next() {
                    current_case.headers.insert(header_name, header_value);
                } else {
                    panic!("LINE {} : Invalid header form on {}, please use : <headerName>: <HeaderValue>", line_index, line)
                }

                line_index += 1;
            }
            ReadingMode::Json(
                (nb_opened_brace, nb_closed_brace),
                (nb_opened_bracket, nb_closed_bracket),
            ) => {
                json_array.push(line.trim_matches('#'));
                let opening_bracket =
                    line.chars().filter(|c| *c == '[').count() + nb_opened_bracket;
                let closing_bracket =
                    line.chars().filter(|c| *c == ']').count() + nb_closed_bracket;

                let opening_brace = line.chars().filter(|c| *c == '}').count() + nb_opened_brace;
                let closing_brace = line.chars().filter(|c| *c == '}').count() + nb_closed_brace;

                reading_mode = ReadingMode::Json(
                    (opening_brace, closing_brace),
                    (opening_bracket, closing_bracket),
                );

                // Checking if the ended the json
                if line.ends_with("}")
                    && opening_bracket == closing_bracket
                    && opening_brace == closing_brace
                {
                    reading_mode = ReadingMode::Data;
                    let full_json = Some(
                        serde_json::from_str(json_array.join("").as_str()).unwrap_or_else(|_| {
                            panic!(
                                "LINE {} : Unable to parse json data for test : {}",
                                line_index, current_case.name
                            )
                        }),
                    );

                    if line.starts_with('#') {
                        current_case.expected_response = full_json
                    } else {
                        current_case.body = full_json
                    }

                    json_array = Vec::new();
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
    SaveCookies,
    UseCookies,
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
                        panic!("Invalid anotation, usage : @expect-status <status>")
                    }
                }
                "@expect-response" => {
                    if let Some(rest) = line.splitn(3, ' ').nth(2) {
                        Some(Anotation::ExpectedResponse(rest))
                    } else {
                        panic!("Invalid anotation, usage : @expect-status <status>")
                    }
                }
                "@save-cookies" => Some(Anotation::SaveCookies),
                "@use-cookies" => Some(Anotation::UseCookies),
                _ => None, // considering a simple regular comment with an @ here
            }
        }
        None => None, // considering regular comment with @ here
    }
}
