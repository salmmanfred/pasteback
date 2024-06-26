#[macro_use]
extern crate lazy_static;
use axum::{ http::StatusCode, response::Html, routing::{get, post}, Router};
mod image_hand;
mod paste_hand;
#[tokio::main]
async fn main() {
    //This one doesnt make sense at first but there is a really good reason for it 
    //This line at the start of the program writes the favicon file to the root directory of the server
    //and since im too lazy to find the actual path 
    // I use "include_bytes" which startpoint (./) is where the file (main.rs) is.
    // it then writes it to the local directory of the execution which is god knows where
    openfile::write_file_bytes("./favicon.ico", include_bytes!("../favicon.ico").to_vec()).unwrap();
    println!("Starting server");
    //Removes information thats only suppose to be in the experimental version and so on
    process_html();
    let app = Router::new()
        
        .route("/", get(root))
        .route("/favicon", get(image_hand::faivcon))
        .route("/new", post(paste_hand::post_token))
        .route("/ret/:token", get(paste_hand::ret_token))
        .route("/health", get(health));
    //Checks if its in offline mode or experimental mode or if its live 
    //if its live it sets correct ports and so on
    //otherwise it adds the experimental marker and sets the port to 3000
    let port = option_env!("PORT").unwrap_or_else(offline_mode);
    match option_env!("EXP"){
        Some(a)=>{
            if a == "EXP"{
                offline_mode();
            }

        }
        None=>{
            offline_mode();
        }
    };
    //Starts the server
    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}",port)).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
//These 2 are only really for the devs to not have to worry about so many things 
fn process_html(){
    //Removes the EXPERIMENTAL VERSION sticker from the live website
    let new_html = include_str!("../index.html").replace("<h3>THIS IS AN EXPERIMENTAL VERSION</h3>", "");
    openfile::write_file("../index_internal.html", &new_html).unwrap();

} 
fn offline_mode()->&'static str{
    let new_html = include_str!("../index.html");

    openfile::write_file("../index_internal.html", &new_html).unwrap();

    return "3000"
}
async fn health()->StatusCode{
    StatusCode::OK
}

async fn root() -> Html<String>{
    Html(openfile::read_file("../index_internal.html").unwrap())
}

