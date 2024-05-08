use std::{fs::File, path::Path};

fn main() {
    let user = escrever_em_imagem::User::new(
        "Daniel inacio",
        // "Rozilaine mlynarczuk",
        // "Wellington Diefenthaeler",
        // "wellington.Diefenthaeler@integrabrasil.com",
        // "Kaylane   Diefenthaeler",
        // "DENISIANA KAZIMIERCZAK"
        // "Wellington Wellington"
        "ti@integrabrasil.com",
        // "rozilaine.mlynarczuk@integrabrasil.com",
        // "comprovante.entrega@integrabrasil.com",
        "3084-8900",
        "Tecnologia da Informação",
        "41 9 9711-4825",
        "80014",
    );
    let mut config = dirs::config_local_dir().unwrap();
    config.push(Path::new("Temp"));
    config.push("assinatura.log");
    let file_log = Box::new(File::create(config).expect("não pode criar o arquivo"));

    env_logger::Builder::new()
        .format_timestamp(None)
        .format_module_path(false)
        .parse_filters("debug") // Nível de log desejado
        .write_style(env_logger::WriteStyle::Never)
        .target(env_logger::Target::Pipe(file_log)) // Definir o arquivo de log
        .init();
    escrever_em_imagem::Assinatura::gerar_imagem_outlook(user);
    // escrever_em_imagem::Assinatura::gerar_imagem_downloads(user);
}
