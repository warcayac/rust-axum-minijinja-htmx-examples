use std::io::Write;

use axum::{extract::{Multipart, State}, response::{Html, IntoResponse}};
use minijinja::context;

use crate::app::{presentation::helpers::*, TAppStateState, APP_ROUTES};


const FILE_EXTS: &str = ".jpeg,.png,.jpg,.csv,.pdf,.json,.mp4,.mov,.avi,.wmv,.flv,.webm,.3gp,.3g2";

pub async fn root_hdlr(State(state): TAppStateState) -> impl IntoResponse {
    let template = state.env.get_template("pages/file_upload/index.html");
    let context = context! {
        target1 => APP_ROUTES["fileupload"].full_pathname(0),
        target2 => APP_ROUTES["fileupload"].full_pathname(1),
    };

    HtmlTemplate(template, context).into_response()
}

pub async fn js_hdlr(State(state): TAppStateState) -> impl IntoResponse {
    let template = state.env.get_template("pages/file_upload/jsversion.html");
    let context = context! {
        target => APP_ROUTES["fileupload"].full_pathname(2),
        file_exts => FILE_EXTS,
    };

    HtmlTemplate(template, context).into_response()
}

pub async fn hs_hdlr(State(state): TAppStateState) -> impl IntoResponse {
    let template = state.env.get_template("pages/file_upload/hsversion.html");
    let context = context! {
        target => APP_ROUTES["fileupload"].full_pathname(2),
        file_exts => FILE_EXTS,
    };

    HtmlTemplate(template, context).into_response()
}

pub async fn upload_hdlr(mut multipart: Multipart) -> impl IntoResponse {
    let file = multipart.next_field().await.unwrap().unwrap();
    let file_name = file.file_name().unwrap().to_string();
    let file_type = file.content_type().unwrap().to_string();
    let file_raw = file.bytes().await.unwrap();
    println!("File uploaded:");
    println!("->> Name: {}", file_name);
    println!("->> Size: {} KB", file_raw.len() / 1024);
    println!("->> Type: {}", file_type);

    let upload_dir = "./uploads";
    if !std::path::Path::new(upload_dir).exists() {
        std::fs::create_dir(upload_dir).expect("Failed to create upload directory");
    }
    let file_path = format!("{}/{}", upload_dir, file_name);
    let mut dest_file = std::fs::File::create(&file_path).expect("Failed to create file");
    dest_file.write_all(&file_raw).expect("Failed to write file");

    Html("Successfully uploaded file!")
}
