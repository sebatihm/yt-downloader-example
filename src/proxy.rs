use rusty_dl::{self, errors::DownloadError, youtube::YoutubeDownloader, Downloader};

pub trait Youtube {
    fn download_video_to(&mut self, link : &str, ubicacion: &str) -> Result<(), DownloadError>;
    fn download_video(&mut self, link : &str) -> Result<(), DownloadError>;
}




pub struct YoutubeProxy{
    youtube : Box<YoutubeDownloader>,
    urls: Vec<String>
}

impl YoutubeProxy{
        
    pub fn new() -> Self{
        return Self{
            urls: vec![],
            youtube: Box::new(YoutubeDownloader::new("https://www.youtube.com/watch?v=fa337WWpz58").expect("Url invalida"))
        };
    }

    fn check_url(&self, link : &str) -> bool{
        let mut state = false;

        for url in &self.urls{
            if url == link{
                state = true;
            }
        }

        state
    }
}

impl Youtube for YoutubeProxy{


    fn download_video(&mut self, link : &str) -> Result<(), DownloadError>{

        if self.check_url(link){
            println!("The video has been already downloaded!");
        }else{
            println!("Getting video...");
            self.urls.push(String::from(link));
            self.youtube = Box::new(YoutubeDownloader::new(link).expect("Url invalida"));
            self.youtube.blocking_download()?;
            println!("Video Downloaded succesfully!");
        }

        
        Ok(())
    }

    fn download_video_to(&mut self, link : &str, ubicacion : &str) -> Result<(), DownloadError>{
        if self.check_url(link){
            println!("The video has been already downloaded!");
        }else{
            println!("Getting video...");
            self.urls.push(String::from(link));
            self.youtube = Box::new(YoutubeDownloader::new(link).expect("Url invalida"));
            self.youtube.blocking_download_to(ubicacion)?;
            println!("Video Downloaded succesfully!");
            println!("Saved in: {}", ubicacion);
        }
        Ok(())
    }


}
