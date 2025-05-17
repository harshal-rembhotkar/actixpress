use actix_web::{web, App, HttpServer, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};


#[derive(Debug, Clone)]
struct AppState {
    users: HashMap<String, String>, // username -> password
    posts: Vec<Post>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
struct User {
    username: String,
    password: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
struct Post {
    id: usize,
    title: String,
    content: String,
    likes: usize,
    author: String,
}

async fn sign_up(data: web::Data<Arc<Mutex<AppState>>>, user: web::Json<User>) -> impl Responder {
    let mut state = data.lock().unwrap();

    if state.users.contains_key(&user.username) {
        return HttpResponse::Conflict().body("User already exists");
    }

    state.users.insert(user.username.clone(), user.password.clone());
    HttpResponse::Ok().body("User signed up successfully")
}

async fn login(data: web::Data<Arc<Mutex<AppState>>>, user: web::Json<User>) -> impl Responder {
    let state = data.lock().unwrap();

    match state.users.get(&user.username) {
        Some(password) if password == &user.password => HttpResponse::Ok().body("Login successful"),
        _ => HttpResponse::Unauthorized().body("Invalid username or password"),
    }
}

async fn create_post(data: web::Data<Arc<Mutex<AppState>>>, post: web::Json<Post>) -> impl Responder {
    let mut state = data.lock().unwrap();
    let mut new_post = post.into_inner();
    new_post.id = state.posts.len() + 1;
    state.posts.push(new_post.clone());

    HttpResponse::Ok().json(new_post)
}

async fn like_post(data: web::Data<Arc<Mutex<AppState>>>, post_id: web::Path<usize>) -> impl Responder {
    let mut state = data.lock().unwrap();

    // Dereference post_id to avoid moving it
    let post_id_value = post_id.into_inner();

    if let Some(post) = state.posts.iter_mut().find(|p| p.id == post_id_value) {
        post.likes += 1;
        HttpResponse::Ok().json(post)
    } else {
        HttpResponse::NotFound().body("Post not found")
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let data = web::Data::new(Arc::new(Mutex::new(AppState {
        users: HashMap::new(),
        posts: Vec::new(),
    })));

    HttpServer::new(move || {
        App::new()
            .app_data(data.clone())
            .route("/signup", web::post().to(sign_up))
            .route("/login", web::post().to(login))
            .route("/posts", web::post().to(create_post))
            .route("/posts/{id}/like", web::post().to(like_post))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
