pub mod proxy;

use proxy::*;
use std::io::stdin;
fn main() {
    display_menu();
}

fn input_choice() -> bool{
    let mut choice = String::new();
    stdin()
        .read_line(&mut choice)
        .expect("Algo salio mal");

    choice = choice.to_uppercase();

    let choice = match choice.chars().next() {
            Some(thing) => thing,
            None => 'N' 
    };

    choice == 'Y'
}

fn input_link() -> String{
    let mut link = String::new();
    stdin()
        .read_line(&mut link)
        .expect("Algo salio mal");


    link = link.trim().to_string();
    link

}

fn display_menu(){
    let mut downloader = YoutubeProxy::new();
    loop {
        println!(" ..:: Video Youtube Downloader :::.. ");
        println!("Ingresa el link del video a descargar: ");
        let link = input_link();

        println!("Desea poner el video en una ubicacion especifica? (Y | N): ");
        let option = input_choice();

        if option{
            println!("Ingrese la ubicacion: ");
            let ubication = input_link();

            let _a = downloader.download_video_to(&link, &ubication);
        }else {
            let _b = downloader.download_video(&link);
        }

        println!("\n\nDesea continuar? (Y | N): ");
        let bucle = input_choice();

        if !bucle{
            break;
        }

    }

}