use std::env;

fn main() {
    // env::args() retorna um iterator com os argumentos da linha de comando.
    // O primeiro item (índice 0) é sempre o nome/caminho do executável.
    let args: Vec<String> = env::args().collect();

    if args.len() == 1 {
        println!("Uso: {} <argumento1> [argumento2] ...", args[0]);
        return;
    }

    let mut fields: Vec<(String, String)> = Vec::new();

    // Default args
    let mut delivery_type = "body";

    println!("Foram passados {} argumento(s):", args.len() - 1);

    // skip(1) ignora o nome do programa; enumerate() numera a partir de 1.
    for arg in args.iter().skip(1) {
        match arg.as_str() {
            "-b" | "--body" => delivery_type = "body",
            "-q" | "--query" => delivery_type = "query",
            _ => {
                if let Some((field, value)) = arg.split_once('=') {
                    fields.push((field.to_string(), value.to_string()));
                } else {
                    println!("Unknown argument: {}", arg);
                }
            }
        }
    }
    println!("Delivery: {}", delivery_type);
    println!("Campos ({}):", fields.len());
    for (campo, valor) in &fields {
        println!("  {} = {}", campo, valor);
    }
}
