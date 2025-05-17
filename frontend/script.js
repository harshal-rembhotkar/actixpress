const API_BASE = "http://127.0.0.1:8080";

async function signUp() {
    const username = document.getElementById("signup-username").value;
    const password = document.getElementById("signup-password").value;

    const response = await fetch(`${API_BASE}/signup`, {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify({ username, password }),
    });

    alert(await response.text());
}

async function logIn() {
    const username = document.getElementById("login-username").value;
    const password = document.getElementById("login-password").value;

    const response = await fetch(`${API_BASE}/login`, {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify({ username, password }),
    });

    if (response.ok) {
        document.getElementById("auth").style.display = "none";
        document.getElementById("post-section").style.display = "block";
    }

    alert(await response.text());
}

async function createPost() {
    const title = document.getElementById("post-title").value;
    const content = document.getElementById("post-content").value;

    const response = await fetch(`${API_BASE}/posts`, {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify({ title, content, author: "User" }),
    });

    fetchPosts();
    alert(await response.text());
}

async function fetchPosts() {
    const response = await fetch(`${API_BASE}/posts`);
    const posts = await response.json();

    const postsDiv = document.getElementById("posts");
    postsDiv.innerHTML = "";

    posts.forEach(post => {
        const postDiv = document.createElement("div");
        postDiv.innerHTML = `
            <h3>${post.title}</h3>
            <p>${post.content}</p>
            <p>Likes: ${post.likes}</p>
            <button onclick="likePost(${post.id})">Like</button>
        `;
        postsDiv.appendChild(postDiv);
    });
}

async function likePost(postId) {
    await fetch(`${API_BASE}/posts/${postId}/like`, { method: "POST" });
    fetchPosts();
}