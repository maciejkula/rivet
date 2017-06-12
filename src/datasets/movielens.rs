use std::fs::File;
use zip::read::ZipArchive;
use std::path::{Path, PathBuf};

use csv;

use traits::{WeightedInteraction, UserID, ItemID};
use datasets::data::{download, get_data_dir};


#[derive(Debug,Deserialize)]
pub struct Interaction {
    user_id: UserID,
    item_id: ItemID,
    rating: usize,
    timestamp: usize,
}


impl WeightedInteraction for Interaction {
    fn get_user_id(&self) -> UserID {
        self.user_id
    }
    fn get_item_id(&self) -> ItemID {
        self.item_id
    }
    fn get_weight(&self) -> f32 {
        1.0
    }
}


fn read_movielens_100k(path: &Path) -> Vec<Interaction> {
    let file = File::open(path).expect("Unable to open Movielens 100K file.");
    let mut archive = ZipArchive::new(file).expect("Invalid archive file.");
    let archive_content = archive.by_name("movielens100k/ml-100k/u.data")
        .expect("No such element.");
    let mut csv_reader = csv::ReaderBuilder::new()
        .delimiter(b'\t')
        .has_headers(false)
        .from_reader(archive_content);

    let mut interactions = Vec::with_capacity(100000);

    for result in csv_reader.deserialize() {
        let interaction = result.expect("Unable to deserialize interaction");
        interactions.push(interaction);
    }

    interactions
}


fn get_movielens_100k() -> Vec<Interaction> {

    let url = "https://github.com/maciejkula/lightfm_datasets/releases/download/v0.1.0/movielens.\
               zip";
    let mut path_buf = get_data_dir();
    path_buf.push("movielens100k");
    path_buf.set_extension("zip");

    let path = path_buf.as_path();

    if !path.exists() {
        download(url, path);
    }

    read_movielens_100k(path)
}


#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn download_100k() {
        get_movielens_100k();
    }
}
