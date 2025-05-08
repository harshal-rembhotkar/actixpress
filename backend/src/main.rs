use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use reqwest;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct AuthData {
    pub email: String,
    pub password: String,
}

#[derive(Serialize, Deserialize)]
pub struct Post {
    pub id: String,
    pub title: String,
    pub content: String,
    pub likes: i32,
}

// Signup handler
async fn signup(data: web::Json<AuthData>) -> impl Responder {
    let url = "https://YOUR_PROJECT_ID.supabase.co/auth/v1/signup"; // Replace with actual Supabase URL
    let client = reqwest::Client::new();
    
    // Request to create a user
    let res = client
        .post(url)
        .header("apikey", "YOUR_SUPABASE_API_KEY") // Replace with your actual API key
        .header("Authorization", format!("Bearer {}", "YOUR_SUPABASE_API_KEY"))
        .json(&*data) // Use json() instead of body() for direct serialization
        .send()
        .await;

    match res {
        Ok(response) => {
            if response.status().is_success() {
                HttpResponse::Ok().body("User signed up successfully")
            } else {
                HttpResponse::BadRequest().body("Failed to sign up user")
            }
        },
        Err(_) => HttpResponse::InternalServerError().body("Error during signup"),
    }
}

// Like post handler
async fn like_post(web::Path(id): web::Path<String>) -> impl Responder {
    let url = format!("https://YOUR_PROJECT_ID.supabase.co/rest/v1/posts?id=eq.{}", id); // Replace with actual Supabase URL
    let client = reqwest::Client::new();
    
    // Request to fetch post by ID
    let res = client
        .get(&url)
        .header("apikey", "YOUR_SUPABASE_API_KEY") // Replace with your actual API key
        .header("Authorization", format!("Bearer {}", "YOUR_SUPABASE_API_KEY"))
        .send()
        .await;

    match res {
        Ok(response) => {
            if let Ok(mut posts) = response.json::<Vec<Post>>().await {
                if let Some(mut post) = posts.pop() {
                    post.likes += 1;
                    
                    // Update the post with a new like count
                    let update_res = client
                        .put(&url)
                        .header("apikey", "YOUR_SUPABASE_API_KEY") // Replace with your actual API key
                        .header("Authorization", format!("Bearer {}", "YOUR_SUPABASE_API_KEY"))
                        .header("Content-Type", "application/json")
                        .json(&post) // Serialize the post into JSON
                        .send()
                        .await;

                    if let Ok(_) = update_res {
                        return HttpResponse::Ok().body("Post liked");
                    } else {
                        return HttpResponse::InternalServerError().body("Error updating post");
                    }
                } else {
                    return HttpResponse::NotFound().body("Post not found");
                }
            } else {
                return HttpResponse::InternalServerError().body("Failed to fetch post");
            }
        },
        Err(_) => HttpResponse::InternalServerError().body("Error during like request"),
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/signup", web::post().to(signup))
            .route("/like/{id}", web::post().to(like_post))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
