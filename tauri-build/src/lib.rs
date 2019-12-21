#[macro_use]
extern crate serde_derive;
extern crate serde_json;

use std::env;
use std::io::Write;

mod config;
#[cfg(not(feature = "dev-server"))]
extern crate tauri_includedir_codegen;
#[cfg(feature = "embedded-server")]
mod tcp;

#[cfg(not(feature = "test"))]
pub fn build() {
  let out_dir = env::var("OUT_DIR").expect("Failed to find out_dir");
  let dest_path = std::path::Path::new(&out_dir).join("tauri_src");
  let mut file = std::fs::File::create(&dest_path).expect("failed to create file");

  let tauri_src: String;
  let config = config::get_tauri_dir();

  #[cfg(not(any(feature = "embedded-server", feature = "no-server")))]
  {
    tauri_src = if config.dev_path.starts_with("http") {
      config.dev_path
    } else {
      let dev_path = std::path::Path::new(&config.dev_path).join("index.tauri.html");
      println!("{}", format!("cargo:rerun-if-changed={:?}", dev_path));
      std::fs::read_to_string(dev_path).expect("failed to build index.tauri.html")
    };
  }

  #[cfg(not(feature = "dev-server"))]
  {
    match env::var("TAURI_DIST_DIR") {
      Ok(dist_path) => {
        // include assets
        tauri_includedir_codegen::start("ASSETS")
          .dir(dist_path, tauri_includedir_codegen::Compression::None)
          .build("/data.rs", config.inlined_assets)
          .expect("failed to build data.rs")
      }
      Err(e) => panic!("Build error: Couldn't find ENV: {}", e),
    }
  }
  #[cfg(feature = "embedded-server")]
  {
    // define URL
    let port;
    let port_valid;
    if config.embedded_server.port == "random" {
      match tcp::get_available_port() {
        Some(available_port) => {
          port = available_port.to_string();
          port_valid = true;
        }
        None => {
          port = "0".to_string();
          port_valid = false;
        }
      }
    } else {
      port = config.embedded_server.port;
      port_valid = crate::tcp::port_is_available(
        port
          .parse::<u16>()
          .expect(&format!("Invalid port {}", port)),
      );
    }
    if port_valid {
      let mut url = format!("{}:{}", config.embedded_server.host, port);
      if !url.starts_with("http") {
        url = format!("http://{}", url);
      }
      tauri_src = url.to_string();
    } else {
      panic!(format!("Port {} is not valid or not open", port));
    }
  }

  #[cfg(feature = "no-server")]
  {
    let index_path = std::path::Path::new(env!("TAURI_DIST_DIR")).join("index.tauri.html");
    println!("{}", format!("cargo:rerun-if-changed={:?}", index_path));
    tauri_src = std::fs::read_to_string(index_path).expect("failed to read string");
  }

  file
    .write_all(tauri_src.as_bytes())
    .expect("failed to write all");
}

#[cfg(feature = "test")]
pub fn build() {
  println!("This is a Test")
}
