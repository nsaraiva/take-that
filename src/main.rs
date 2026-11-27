use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    // Se só passou o nome do programa, mostra o uso
    if args.len() == 1 {
        println!("Uso: {} [-X <METHOD>]", args[0]);
        println!("Métodos: GET, POST, PUT, PATCH, DELETE");
        return;
    }

    // Default request method: GET
    let mut method = "GET";

    let mut url = "";
    let mut path = "";
    let mut header = "";
    let mut data = "";

    // Start at index 1 to skip the program name (index 0)
    let mut i = 1;

    while i < args.len() {
        match args[i].as_str() {
            "-X" => {
                // Verify if there is a next argument after -X
                if i + 1 < args.len() {
                    method = &args[i + 1];

                    i += 2; // skip the -X and the method
                } else {
                    println!("Error: -X requires a method (POST, PUT, PATCH, DELETE)");
                    return;
                }
            }
            "-u" => {
                if i + 1 < args.len() {
                    url = &args[i + 1];

                    i += 2; // skip the -u and the URL value
                } else {
                    println!("Error: -u requires a URL");
                    return;
                }
            }
            "-w" => {
                if i + 1 < args.len() {
                    path = &args[i + 1];

                    i += 2; // skip the -w and the path value
                } else {
                    println!("Error: -w requires a path");
                    return;
                }
            }
            "-H" => {
                if i + 1 < args.len() {
                    header = &args[i + 1];

                    i += 2; // skip the -H and the header value
                } else {
                    println!("Error: -H requires a header");
                    return;
                }
            }
            "-d" => {
                if i + 1 < args.len() {
                    data = &args[i + 1];

                    i += 2; // skip the -d and the data value
                } else {
                    println!("Error: -d requires data");
                    return;
                }
            }
            _ => {
                //println!("Error: unknown argument {}", args[i]);
                return;
            }
        }
    }

    println!("Request: {}", method);
    println!("URL: {}", url);
    println!("Header: {}", header);
    println!("Data: {}", data);
    println!("Word list: {}", path);

    let words = read_wordlist(path);
    for word in words {
        let new_data = data.replace("TAKE", &word);

        let response = match method {
            "GET" => ureq::get(url).call(),
            "POST" => ureq::post(url).send_string(&new_data),
            "PUT" => ureq::put(url).send_string(&new_data),
            "PATCH" => ureq::patch(url).send_string(&new_data),
            "DELETE" => ureq::delete(url).call(),
            _ => {
                println!("Method not supported: {}", method);
                return;
            }
        };

        match response {
            Ok(res) => {
                println!("Status: {}", res.status());
            }
            Err(e) => println!("Request error: {}", e),
        }
    }
}

#[allow(clippy::lines_filter_map_ok)]
fn read_wordlist(path: &str) -> Vec<String> {
    use std::fs::File;
    use std::io::{self, BufRead};
    use std::path::Path;

    let mut words: Vec<String> = Vec::new();

    if let Ok(file) = File::open(Path::new(path)) {
        for word in io::BufReader::new(file).lines().flatten() {
            words.push(word);
        }
    }
    words
}
