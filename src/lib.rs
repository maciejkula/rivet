extern crate reqwest;
extern crate rand;

#[macro_use]
extern crate serde_derive;

extern crate serde;
extern crate serde_json;
extern crate app_dirs;
extern crate csv;
extern crate zip;

use app_dirs::*;

const APP_INFO: AppInfo = AppInfo {
    name: "Rivet",
    author: "Maciej Kula",
};

pub mod datasets;
pub mod traits;
pub mod models;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {}
}
