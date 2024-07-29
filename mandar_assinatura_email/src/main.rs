use base64::Engine;
use escrever_em_imagem;
use lettre::message::MultiPart;
use lettre::message::{header::ContentType, Attachment};
use lettre::{
    transport::smtp::authentication::Credentials, AsyncSmtpTransport, AsyncTransport, Message,
    Tokio1Executor,
};

use rpassword::read_password;
use std::io::{self, Write};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    print!("Digite o nome/email de usuário do SMTP: ");
    io::stdout().flush()?;
    let mut nome = String::new();
    io::stdin().read_line(&mut nome)?;
    let nome = nome.trim();

    print!("Digite a senha do SMTP: ");
    io::stdout().flush()?;
    let smtp_credentials = Credentials::new(nome.to_string(), read_password()?);

    print!("digite a URL do servidor SMTP: ");
    io::stdout().flush()?;
    let mut servidor = String::new();
    io::stdin().read_line(&mut servidor)?;

    let mailer = AsyncSmtpTransport::<Tokio1Executor>::relay(servidor.trim())?
        .credentials(smtp_credentials)
        .build();

    let from = "<ti@integrabrasil.com>";
    let subject = "Assinatura Atualizada";

    let usuarios = query::buscar_no_banco_pelo_nome("").await?;
    for user in usuarios.iter() {
        let tmp = user.login.replace(".", " ");
        let user_final = escrever_em_imagem::User::new(
            tmp.as_str(),
            &user.email,
            "41 3084-8900",
            &user.department,
            &user.phone_number,
            &user.extension,
        );

        if "paulo.yure".contains(&user.login) {
            println!("pulado paulo yure");
            continue;
        }
        if "monitoramento@integrabrasil.com".contains(&user.email) {
            println!("pulado monitoramento");
            continue;
        }
        println!("{}", tmp);
        let body: String = escrever_em_imagem::Assinatura::gerar_imagem_base64(user_final);
        let _ = send_email_smtp(&mailer, from, &user.email, subject, body).await;
        // let _ = send_email_smtp(&mailer, from, "ti@integrabrasil.com", subject, body).await;
    }

    Ok(())
}

async fn send_email_smtp(
    mailer: &AsyncSmtpTransport<Tokio1Executor>,
    from: &str,
    to: &str,
    subject: &str,
    body: String,
) -> Result<(), Box<dyn std::error::Error>> {
    let tipo_arquivo = ContentType::parse("imagem/png").unwrap();
    let body = base64::engine::general_purpose::STANDARD.decode(&body)?;
    let anexo = Attachment::new("assinatura.png".to_string()).body(body, tipo_arquivo);

    let email = Message::builder()
        .from(from.parse()?)
        .to(to.parse()?)
        .subject(subject)
        .multipart(MultiPart::related().singlepart(anexo).singlepart(
            lettre::message::SinglePart::plain(
                "Olá,  segue em anexo assinatura de email atualizada\n a anterior estava com o numero da integra errado, e resolução baixa".to_string(),
            ),
        ))?;

    mailer.send(email).await?;

    Ok(())
}
