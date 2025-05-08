const apiBase = 'http://127.0.0.1:8080';

document.addEventListener('DOMContentLoaded', () => {
    if (document.getElementById('signup-form')) {
        document.getElementById('signup-form').addEventListener('submit', async (e) => {
            e.preventDefault();
            const email = document.getElementById('email').value;
            const password = document.getElementById('password').value;

            const res = await fetch(`${apiBase}/sign_up`, {
                method: 'POST',
                headers: {'Content-Type': 'application/json'},
                body: JSON.stringify({ email, password })
            });

            alert(await res.text());
            window.location.href = 'index.html';
        });
    }

    if (document.getElementById('login-form')) {
        document.getElementById('login-form').addEventListener('submit', async (e) => {
            e.preventDefault();
            const email = document.getElementById('email').value;
            const password = document.getElementById('password').value;

            const res = await fetch(`${apiBase}/login`, {
                method: 'POST',
                headers: {'Content-Type': 'application/json'},
                body: JSON.stringify({ email, password })
            });

            const data = await res.json();
            localStorage.setItem('access_token', data.access_token);
            alert('Logged in!');
            window.location.href = 'index.html';
        });
    }

    if (document.getElementById('post-form')) {
        loadPosts();

        document.getElementById('post-form').addEventListener('submit', async (e) => {
            e.preventDefault();
            const title = document.getElementById('title').value;
            const content = document.getElementById('content').value;
            const author = document.getElementById('author').value;

            await fetch(`${apiBase}/posts`, {
                method: 'POST',
                headers: {'Content-Type': 'application/json'},
                body: JSON.stringify({ title, content, author })
            });

            loadPosts();
        });
    }
});

async function loadPosts() {
    const res = await fetch(`${apiBase}/posts`);
    const posts = await res.json();
    const container = document.getElementById('posts-container');
    container.innerHTML = '';

    posts.forEach(post => {
        const div = document.createElement('div');
        div.className = 'post';
        div.innerHTML = `
            <h3>${post.title}</h3>
            <p>${post.content}</p>
            <p><em>By ${post.author}</em></p>
            <p>Likes: ${post.likes}</p>
            <button onclick="likePost('${post.id}')">Like</button>
        `;
        container.appendChild(div);
    });
}

async function likePost(postId) {
    await fetch(`${apiBase}/posts/${postId}/like`, {
        method: 'POST'
    });
    loadPosts();
}
