use std::env;
use std::process::{Command, exit};

fn main() {
    // Pega todos os argumentos, ignorando o primeiro (nome do binário)
    let mut args = env::args().skip(1);

    // O primeiro argumento deve ser o script
    let script = match args.next() {
        Some(s) => s,
        None => {
            eprintln!("Usage: shell <script> [args...]");
            exit(1);
        }
    };

    // Resto dos argumentos
    let script_args: Vec<String> = args.collect();

    // Detecta o sistema operacional para escolher o shell correto
    #[cfg(target_os = "windows")]
    let mut cmd = {
        let mut c = Command::new("cmd");
        c.arg("/C").arg(&script).args(&script_args);
        c
    };

    #[cfg(not(target_os = "windows"))]
    let mut cmd = {
        let mut c = Command::new("sh");
        c.arg(&script).args(&script_args);
        c
    };

    // Executa o script
    let status = cmd.status().expect("Failed to execute script");

    // Repasse do código de saída
    exit(status.code().unwrap_or(1));
}
