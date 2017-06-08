use std::fs::File;
use std::io::prelude::*;
use std::io::BufWriter;
use std::io::copy;

use std::path::{Path, PathBuf};

use app_dirs;
use reqwest;


pub fn get_data_dir() -> PathBuf {
    app_dirs::data_root(app_dirs::AppDataType::UserData).expect("Unable to get data directory")
}


pub fn download(url: &str, path: &Path) {

    let mut file = File::create(path).expect("Unable to create target file");
    let mut writer = BufWriter::new(file);

    let mut data = reqwest::get(url).expect("Unable to download data");

    copy(&mut data, &mut writer);
}


#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn it_works() {
        get_data_dir();
    }

    #[test]
    fn download_100k() {
        get_data_dir();
    }
}
