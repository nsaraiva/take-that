use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    // Se só passou o nome do programa, mostra o uso
    if args.len() == 1 {
        println!("Uso: {} [-X <METODO>]", args[0]);
        println!("Métodos: GET, POST, PUT, PATCH, DELETE");
        return;
    }

    // Valor padrão: GET
    let mut request_type = "GET";

    let mut url = "";
    let mut path = "";
    let mut header = "";
    let mut data = "";

    // Começa no índice 1 para pular o nome do programa (índice 0)
    let mut i = 1;

    while i < args.len() {
        match args[i].as_str() {
            "-X" => {
                // Verifica se existe um próximo argumento após o -X
                if i + 1 < args.len() {
                    request_type = &args[i + 1];
                    i += 2; // pula o -X e o método
                } else {
                    println!("Erro: -X requer um método (POST, PUT, PATCH, DELETE)");
                    return;
                }
            }
            "-u" => {
                if i + 1 < args.len() {
                    url = &args[i + 1];
                    i += 2; // pula o -u e o valor da URL
                } else {
                    println!("Erro: -u requer uma URL");
                    return;
                }
            }
            "-w" => {
                if i + 1 < args.len() {
                    path = &args[i + 1];
                    i += 2; // pula o -w e o valor do path
                } else {
                    println!("Erro: -w requer um path");
                    return;
                }
            }
            "-H" => {
                if i + 1 < args.len() {
                    header = &args[i + 1];
                    i += 2; // pula o -H e o valor do header
                } else {
                    println!("Erro: -H requer um header");
                    return;
                }
            }
            "-d" => {
                if i + 1 < args.len() {
                    data = &args[i + 1];
                    i += 2; // pula o -d e o valor do data
                } else {
                    println!("Erro: -d requer um data");
                    return;
                }
            }
            _ => {
                println!("Erro: argumento desconhecido {}", args[i]);
                return;
            }
        }
    }

    println!("Request: {}", request_type);
    println!("URL: {}", url);
    println!("Header: {}", header);
    println!("Data: {}", data);
    println!("Wordlist: {}", path);
}
