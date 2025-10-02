use std::env;
use std::process::{Command, exit};

fn main() {
    // Captura todos os argumentos passados ao binário, ignorando o primeiro (nome do binário)
    let mut args = env::args().skip(1);

    // O primeiro argumento agora deve ser o script a executar
    let script = match args.next() {
        Some(s) => s,
        None => {
            eprintln!("Usage: shell <script> [args...]");
            exit(1);
        }
    };

    // Resto dos argumentos
    let script_args: Vec<String> = args.collect();

    // Cria o comando para chamar o script via bash
    let status = Command::new("bash")
        .arg(&script)
        .args(&script_args)
        .status()
        .expect("Failed to execute script");

    // Repasse o código de saída do script
    exit(status.code().unwrap_or(1));
}
