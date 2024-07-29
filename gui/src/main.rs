// #![windows_subsystem = "windows"]
use std::{fs::File, path::Path};

slint::include_modules!();
use std::rc::Rc;

use query::{self, Funcionario};
use slint::{ComponentHandle, SharedString, VecModel};
use std::str::FromStr;
async fn pegar_dados_nome(nome: &str) -> Vec<Funcionario> {
    let tmp = query::buscar_no_banco_pelo_nome(nome)
        .await
        .unwrap_or_default();
    let mut vec_tmp = Vec::new();
    for i in tmp {
        vec_tmp.push(i);
    }
    vec_tmp
}
async fn pegar_dados_id(nome: i32) -> Funcionario {
    query::buscar_no_banco_pelo_id(&nome).await.unwrap()
}

#[tokio::main]
async fn main() {
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
    let app = Demo::new().unwrap();
    let ui = app.as_weak();
    app.on_change_name(move || {
        let my_ui = ui.unwrap();
        let _ = slint::spawn_local(async move {
            let input: SharedString = my_ui.get_input();
            let data = pegar_dados_nome(&input).await;
            let row_data = Rc::new(VecModel::default());
            for i in data {
                let items = Rc::new(VecModel::default());
                items.push(slint::format!("{}", i.name).into());
                items.push(slint::format!("{}", i.department).into());
                items.push(slint::format!("{}", i.email).into());
                items.push(slint::format!("{}", i.id).into());
                row_data.push(items.into());
            }
            my_ui.global::<User>().set_users(row_data.into());
        });
    });
    let ui = app.as_weak();
    app.on_download(move || {
        let my_ui = ui.unwrap();
        let _ = slint::spawn_local(async move {
            let user = my_ui.global::<User>().get_actual_id();
            let user = FromStr::from_str(user.as_str()).unwrap_or_default();
            let data = pegar_dados_id(user).await;
            let login = &data.login.clone();
            let mut name = data.name;
            let login: Vec<&str> = login.split('.').collect();
            if name.len() > 1 {
                name = std::format!("{} {}", login[0], login[1]);
            }
            let teste = escrever_em_imagem::User::new(
                &name,
                &data.email,
                "41 3084-8900",
                &data.department,
                &data.phone_number,
                &data.extension,
            );
            escrever_em_imagem::Assinatura::gerar_imagem_downloads(teste);
        });
    });
    let ui = app.as_weak();
    // app.on_outlook(move || {
    //     let my_ui = ui.unwrap();
    //     let _ = slint::spawn_local(async move {
    //         let user = my_ui.global::<User>().get_actual_id();
    //         let user = FromStr::from_str(user.as_str()).unwrap_or_default();
    //         let data = pegar_dados_id(user).await;
    //         let teste = escrever_em_imagem::User::new(
    //             &data.name,
    //             &data.email,
    //             "41 3084-8900",
    //             &data.department,
    //             &data.phone_number,
    //             &data.extension,
    //         );
    //         escrever_em_imagem::Assinatura::gerar_imagem_outlook(teste);
    //     });
    // });
    app.run().unwrap();
}
