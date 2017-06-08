extern crate reqwest;

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

mod datasets;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {}
}
