use base64::Engine;
use image::{open, DynamicImage, ImageBuffer, Rgba};
use imageproc::drawing::draw_text;
use regex::Regex;
use rusttype::{Font, Scale};
use std::fs;
use std::fs::File;
use std::io::{Read, Write};
use std::ops::Not;
use std::path::Path;

pub struct User {
    name: String,
    email: String,
    telefone: String,
    setor: String,
    celular: String,
    ramal: String,
}

impl User {
    pub fn new(
        name: &str,
        email: &str,
        telefone: &str,
        setor: &str,
        celular: &str,
        ramal: &str,
    ) -> Self {
        Self {
            name: name.to_string(),
            email: email.to_string(),
            telefone: telefone.to_string(),
            setor: setor.to_string(),
            celular: celular.to_string(),
            ramal: ramal.to_string(),
        }
    }
}

const AZUL_ESCURINHO: Rgba<u8> = Rgba([23u8, 34u8, 82u8, 1u8]);
const INICIO_ESCRITA_W: i32 = 240;
const VERDE_CLARINHO: Rgba<u8> = Rgba([29u8, 35u8, 65u8, 1u8]);

const IMAGEM_BASE_DIRETORIO: &str = "\\\\srv1-ibl02\\Wallpaper\\assinatura_base.png";
const TAMANHO_FONTE: u32 = 16;
pub struct Assinatura {
    img: ImageBuffer<Rgba<u8>, Vec<u8>>,
    font: Font<'static>,
    signature_name: String,
    signature_number: String,
}

impl Assinatura {
    fn new() -> Self {
        let path = Path::new(&IMAGEM_BASE_DIRETORIO);
        let img = open(path).expect("deu ruim").into_rgba8();
        // let font = Vec::from(include_bytes!("../asset/Carlito-Bold.ttf") as &[u8]);
        let font = Vec::from(include_bytes!("../asset/LiberationSans-Bold.ttf") as &[u8]);
        let font = Font::try_from_vec(font).unwrap();
        Self {
            img,
            font,
            signature_name: String::from("teste_imagem"),
            signature_number: String::new(),
        }
    }

    fn carregar_font(&self) -> Font<'static> {
        // let font = Vec::from(include_bytes!((ACTUAL_FONT as &[u8])));
        let font = Vec::from(include_bytes!("../asset/LiberationSans-Regular.ttf") as &[u8]);
        Font::try_from_vec(font).unwrap()
    }
    fn escrever_email(&mut self, texto: &str) {
        self.signature_name = texto.split_once('@').unwrap_or_default().0.to_string();
        self.signature_name = format!("{} ({})", self.signature_name, texto);
        self.img = draw_text(
            &self.img,
            AZUL_ESCURINHO,
            INICIO_ESCRITA_W,
            103,
            self.font_scaling(TAMANHO_FONTE),
            &self.font,
            texto,
        );
    }

    fn diminuir_imagem(&mut self, width: u32, height: u32) {
        self.img = DynamicImage::ImageRgba8(self.img.clone())
            .thumbnail(width, height)
            .to_rgba8();

        let nova_altura = self.img.height() + 15; // Ajuste conforme necessário

        let nova_img = ImageBuffer::from_fn(self.img.width(), nova_altura, |x, y| {
            if y < self.img.height() {
                *self.img.get_pixel(x, y)
            } else {
                Rgba([255, 255, 255, 0]) // Fundo branco para o espaço do texto
            }
        });

        self.img = nova_img;
    }
    fn escrever_nome(&mut self, texto: &str) {
        self.img = draw_text(
            &self.img,
            AZUL_ESCURINHO,
            INICIO_ESCRITA_W,
            7,
            self.font_scaling(TAMANHO_FONTE + 9),
            &self.font,
            texto.to_uppercase().as_str(),
        );
    }

    fn escrever_setor(&mut self, texto: &str) {
        self.img = draw_text(
            &self.img,
            VERDE_CLARINHO,
            INICIO_ESCRITA_W,
            33,
            self.font_scaling(TAMANHO_FONTE - 2),
            &self.carregar_font(),
            // &self.font,
            texto.to_uppercase().as_str(),
        );
    }

    fn font_scaling(&self, height: u32) -> Scale {
        Scale {
            x: height as f32,
            y: height as f32,
        }
    }

    fn escrever_site(&mut self) {
        let texto = "www.integrabrasil.com";
        self.img = draw_text(
            &self.img,
            AZUL_ESCURINHO,
            INICIO_ESCRITA_W,
            122,
            self.font_scaling(TAMANHO_FONTE),
            &self.font,
            texto,
        );
    }

    fn escrever_telefone(&mut self, texto: &str, ramal: &str) {
        let mut tmp = texto.to_string();

        if ramal.is_empty().not() {
            tmp = format!("{}   -   Ramal {}", texto, ramal);
        }
        self.img = draw_text(
            &self.img,
            AZUL_ESCURINHO,
            INICIO_ESCRITA_W,
            83,
            self.font_scaling(TAMANHO_FONTE),
            &self.font,
            tmp.as_str(),
        );
    }
    fn escrever_celular(&mut self, texto: &str) {
        self.signature_number = texto.to_string();
        let re = Regex::new(r"(\d{2})(\d)(\d{4})(\d{4})").unwrap();
        let formatted_number = re.replace(texto, "$1 $2 $3-$4").to_string();
        self.img = draw_text(
            &self.img,
            AZUL_ESCURINHO,
            INICIO_ESCRITA_W,
            63,
            self.font_scaling(TAMANHO_FONTE),
            &self.font,
            formatted_number.as_str(),
        );
    }

    fn escrever_mensagem(&mut self) {
        let mensagem =
            "Notifique se recebido por engano. Responsabilidade ambiental, pense antes de imprimir";
        self.img = draw_text(
            &self.img,
            AZUL_ESCURINHO,
            1,
            174,
            self.font_scaling(15),
            &self.font,
            mensagem.to_uppercase().as_str(),
        );
    }

    pub fn gerar_imagem_outlook(user: User) {
        let h = 200;
        let w = 700;
        let mut ass = Assinatura::new();
        ass.diminuir_imagem(w, h);
        ass.escrever_nome(&user.name);
        ass.escrever_email(&user.email);
        ass.escrever_setor(user.setor.trim());
        ass.escrever_telefone(&user.telefone, &user.ramal);
        ass.escrever_celular(&user.celular);
        ass.escrever_mensagem();
        ass.escrever_site();
        ass.escrever_html();
    }
    pub fn gerar_imagem_downloads(user: User) {
        let h = 200;
        let w = 700;
        let mut ass = Assinatura::new();
        ass.diminuir_imagem(w, h);
        ass.escrever_nome(&user.name);
        ass.escrever_email(&user.email);
        ass.escrever_setor(&user.setor);
        ass.escrever_telefone(&user.telefone, &user.ramal);
        ass.escrever_celular(&user.celular);
        ass.escrever_mensagem();
        ass.escrever_site();
        ass.salvar_imagem(&user.name);
    }
    fn salvar_imagem(&self, name: &str) {
        let mut path_download_image = dirs::download_dir().unwrap_or_default();
        path_download_image.push(format!("{}.jpg", name));

        let _ = self.img.save(path_download_image);
        // let _ = self
        //     .img
        //     .save_with_format(path_download_image, image::ImageFormat::Jpeg);
    }
    fn escrever_html(&self) {
        let mut path_img = dirs::config_dir().unwrap();
        path_img.push(Path::new("Microsoft"));
        path_img.push(Path::new("Signatures"));
        let mut path_html = path_img.clone();
        path_html.push(format!("{}.htm", self.signature_name));
        path_img.push(format!("{}_arquivos", self.signature_name));
        let _ = fs::create_dir_all(&path_img);
        path_img.push("image.jpg");

        let _ = self.img.save(&path_img);
        let mut arquivo = File::open(&path_img).unwrap();
        let mut buffer = Vec::new();
        let _ = File::read_to_end(&mut arquivo, &mut buffer).unwrap();
        let img_base64 = base64::engine::general_purpose::STANDARD.encode(buffer);

        let html = maud::html! {
            body{
                // div.container
                // {

                    a href={"https://api.whatsapp.com/send?phone=55"(self.signature_number.replace(" ","").replace("-",""))};


                    img src={"data:image/jpg;base64," (img_base64)};
                // };
                }
        };
        let html = html.into_string();

        let mut file = match File::create(&path_html) {
            Ok(file) => {
                log::info!("arquivo criado");
                log::debug!("{:?}", &path_html);
                file
            }
            Err(err) => {
                log::error!("Erro ao criar o arquivo: {}\n{}", err, &path_html.display());
                return;
            }
        };

        match file.write_all(html.as_bytes()) {
            Ok(_) => println!(
                "HTML salvo com sucesso em {}",
                path_html.to_str().unwrap_or_default()
            ),
            Err(err) => log::error!("Erro ao escrever o HTML no arquivo: {}", err),
        }
    }
}
