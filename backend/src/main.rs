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
    let url = "https://frkqxwqaveertytosczc.supabase.co/auth/v1/signup"; // Replace with actual Supabase URL
    let client = reqwest::Client::new();
    
    // Request to create a user
    let res = client
        .post(url)
        .header("apikey", "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJpc3MiOiJzdXBhYmFzZSIsInJlZiI6ImZya3F4d3FhdmVlcnR5dG9zY3pjIiwicm9sZSI6InNlcnZpY2Vfcm9sZSIsImlhdCI6MTc0NjY4MDk1NCwiZXhwIjoyMDYyMjU2OTU0fQ.HQuN3QAcuydlTJjTY6wUE-ud8M_YWillmvRUz_uSBd0") // Replace with your actual API key
        .header("Authorization", format!("Bearer {}", "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJpc3MiOiJzdXBhYmFzZSIsInJlZiI6ImZya3F4d3FhdmVlcnR5dG9zY3pjIiwicm9sZSI6InNlcnZpY2Vfcm9sZSIsImlhdCI6MTc0NjY4MDk1NCwiZXhwIjoyMDYyMjU2OTU0fQ.HQuN3QAcuydlTJjTY6wUE-ud8M_YWillmvRUz_uSBd0"))
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
async fn like_post(path: web::Path<String>) -> impl Responder {
    let id = path.into_inner();
    let url = format!("https://frkqxwqaveertytosczc.supabase.co/rest/v1/posts?id=eq.{}", id);
    let api_key = "YOUR_SUPABASE_API_KEY";
    let bearer_token = format!("Bearer {}", api_key);

    let client = reqwest::Client::new();

    // Fetch the post
    let res = client
        .get(&url)
        .header("apikey", api_key)
        .header("Authorization", &bearer_token)
        .send()
        .await;

    match res {
        Ok(response) => {
            match response.json::<Vec<Post>>().await {
                Ok(mut posts) => {
                    if let Some(mut post) = posts.pop() {
                        post.likes += 1;

                        let update_res = client
                            .put(&url)
                            .header("apikey", api_key)
                            .header("Authorization", &bearer_token)
                            .header("Content-Type", "application/json")
                            .json(&post)
                            .send()
                            .await;

                        match update_res {
                            Ok(_) => HttpResponse::Ok().body("Post liked"),
                            Err(_) => HttpResponse::InternalServerError().body("Error updating post"),
                        }
                    } else {
                        HttpResponse::NotFound().body("Post not found")
                    }
                }
                Err(_) => HttpResponse::InternalServerError().body("Failed to parse post response"),
            }
        }
        Err(_) => HttpResponse::InternalServerError().body("Failed to fetch post"),
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
