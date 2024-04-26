use std::env;
use std::error::Error;
use std::fs::File;
use std::io::Write;
use actix_multipart::{Multipart,MultipartError};
use futures_util::StreamExt;

pub struct Upload{}

impl Upload {
    pub fn new()->Self{
        Self{}
    }
    pub async fn upload(&self,mut file:Multipart) -> Result<String, Box<dyn Error>>{
        //获取到要保存的文件路径
        let path = env::var("UPLOAD_PATH")?;
        let mut path_url = String::from("");
        while let Some(part) = file.next().await {
            let mut val = part?;
            let content_disposition = val.content_disposition();
            let file_name = content_disposition.get_filename().ok_or_else(||{
                Box::<dyn Error>::from(MultipartError::MissingField(
                    "🔥 Failed to get the file name".to_string(),
                ))
            })?;
            let file_path = format!("{}{}",path,file_name);
            path_url = file_path.clone();
            //创建文件
            let mut f = File::create(&file_path)?;
            //写入文件
            while let Some(chunk) = val.next().await {
                let data = chunk?;
                f.write_all(&data)?;
            }
        }
        Ok(path_url)
    }
}


