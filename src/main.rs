use actix_web::{web, App, HttpServer};
// use serde_json::Result;
use listenfd::ListenFd;

use std::io;
use std::io::{Read, Write};
use std::fs;
use std::fs::File;
// use rand::{thread_rng, Rng};
// use rand::distributions::Alphanumeric;

pub mod handles;
pub mod config;
pub mod types;

use types::dataset;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=info");

    let config = config::Config {
        app_name: String::from("Actix-web"),
        storage_path: String::from("./storage/"),
        dataset_path: String::from("./datasets/"),
    };

    std::fs::create_dir_all(&config.storage_path).unwrap();
    let mut listenfd = ListenFd::from_env();

    let config_clone = config.clone();
    let mut server = HttpServer::new(move || {
        App::new()
            .data(config_clone.clone())
            .route("/", web::get().to(handles::_index))
            .route("/", web::post().to(handles::save_file))
            .route("/{dataset}/{commit}/{filename}", web::get().to(handles::retrieve_file))
        });

    let dataset1 = dataset::Dataset {
        name: String::from("Cat photos"),
        path: String::from("cat_photos"),
        backend: dataset::Backend::LocalBackend(
            dataset::LocalBackend { 
                path: String::from("./storage/")
            }
        ),
        description: String::from("Very important cat photos.")
    };

    let dataset_string = serde_json::to_string_pretty(&dataset1).unwrap();
    let path = format!("{}{}.json", &config.dataset_path, dataset1.path);
    write_string_to_file(&path, &dataset_string).unwrap();

    let entries = fs::read_dir(&config.dataset_path).unwrap();

    // let mut datasets: Vec<types::dataset::Dataset>;

    let datasets: Vec<types::dataset::Dataset> = fs::read_dir(&config.dataset_path)?
        .filter_map(Result::ok)
        .map(|e| e.path())
        .map(|path| {
            let file = fs::read_to_string(path).unwrap();
            let dataset: types::dataset::Dataset = serde_json::from_str(&file).unwrap();
            dataset
        }).collect();

    // entries.filter_map(|entry| {
    //     entry.unwrap().path().extension()

    // });

    // for entry in entries {
    //     let entry = entry?;
    //     let path = entry.path();
    //     match path.extension() {
    //         Some(dataset) => println!("This is a database file"),
    //         None => println!("This is not a database file")
    //     }
    //     let filename = path.file_name().unwrap();
    //     println!("Filename: {}", &filename.to_str().unwrap());

    //     // println!("Name: {}", path.unwrap().path().display())
    //     // println!("Name: {}", path.unwrap().path().display())
    // }
    // let commit_hash: String = thread_rng()
    //     .sample_iter(&Alphanumeric)
    //     .take(32)
    //     .collect();

    // let commit = types::Commit {
    //     hash: commit_hash,
    //     parent: String::from(""),
    //     branch: String::from("master"),
    //     name: String::from("eerste_commit"),
    //     desc: String::from("Dit is een commit van een dataset"),
    //     diff: vec![],
    //     deprecated: false
    // };

    // let commit_string = serde_json::to_string_pretty(&commit).unwrap();
    // write_file(&String::from("./storage/versions/test.json"), &commit_string).unwrap();

    
    server = if let Some(l) = listenfd.take_tcp_listener(0).unwrap() {
        server.listen(l)?
    } else {
        server.bind("127.0.0.1:3000")?
    };

    server.run().await

}

pub fn read_file_to_string(filepath: &String) -> Result<String, std::io::Error> {
    let mut file = File::open(filepath)?;

    let mut data = String::new();
    file.read_to_string(&mut data)?;
    Ok(data)
}


pub fn write_string_to_file(filepath: &String, content: &String) -> Result<(), io::Error> {
    let mut file = File::create(filepath)?;
    file.write_all(content.as_bytes())?;
    Ok(())
}