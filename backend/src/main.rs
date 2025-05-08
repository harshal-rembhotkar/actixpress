use actix_web::{web, App, HttpServer, HttpResponse, Responder, post, put, delete};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use dotenv::dotenv;
use std::env;
use supabase::SupabaseClient;  // Assuming you're using Supabase client or an HTTP client to interact with Supabase API
use uuid::Uuid;

#[derive(Serialize, Deserialize)]
struct Post {
    id: String,
    user_id: String,
    title: String,
    content: String,
    likes: i32,
    shared: bool,
}

#[derive(Serialize, Deserialize)]
struct User {
    email: String,
    password: String,
}

#[derive(Clone)]
struct AppState {
    supabase: Arc<SupabaseClient>,  // You need to integrate Supabase for DB interaction
}

#[post("/sign_up")]
async fn sign_up(data: web::Data<AppState>, user: web::Json<User>) -> impl Responder {
    // Simulating user signup - You should interact with Supabase to create the user
    let new_user = data.supabase.create_user(user.into_inner()).await;
    HttpResponse::Created().json(new_user)
}

#[post("/posts")]
async fn create_post(data: web::Data<AppState>, post: web::Json<Post>) -> impl Responder {
    // Create a post in the Supabase database
    let new_post = data.supabase.create_post(post.into_inner()).await;
    HttpResponse::Created().json(new_post)
}

#[put("/posts/{id}/like")]
async fn like_post(data: web::Data<AppState>, post_id: web::Path<String>) -> impl Responder {
    // Simulate liking the post, interact with Supabase to update
    let updated_post = data.supabase.like_post(&post_id).await;
    HttpResponse::Ok().json(updated_post)
}

#[delete("/posts/{id}")]
async fn delete_post(data: web::Data<AppState>, post_id: web::Path<String>) -> impl Responder {
    // Delete post using Supabase API
    let result = data.supabase.delete_post(&post_id).await;
    HttpResponse::Ok().json(result)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();  // Load environment variables

    let supabase_url = env::var("SUPABASE_URL").expect("SUPABASE_URL not set");
    let supabase_key = env::var("SUPABASE_KEY").expect("SUPABASE_KEY not set");

    let supabase_client = Arc::new(SupabaseClient::new(&supabase_url, &supabase_key));

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(AppState { supabase: supabase_client.clone() }))
            .service(sign_up)
            .service(create_post)
            .service(like_post)
            .service(delete_post)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
