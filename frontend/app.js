document.addEventListener("DOMContentLoaded", function() {
    fetch("http://127.0.0.1:8080/posts")
        .then(response => response.json())
        .then(posts => {
            const postsContainer = document.getElementById("posts");
            posts.forEach(post => {
                const postDiv = document.createElement("div");
                postDiv.classList.add("post");
                postDiv.innerHTML = `
                    <h2>${post.title}</h2>
                    <p>${post.content}</p>
                    <button onclick="likePost('${post.id}')">Like</button>
                    <button onclick="deletePost('${post.id}')">Delete</button>
                `;
                postsContainer.appendChild(postDiv);
            });
        });
});

function likePost(postId) {
    fetch(`http://127.0.0.1:8080/posts/${postId}/like`, {
        method: 'PUT',
    })
    .then(response => response.json())
    .then(data => {
        alert('Post liked!');
    })
    .catch(error => console.error('Error liking post:', error));
}

function deletePost(postId) {
    fetch(`http://127.0.0.1:8080/posts/${postId}`, {
        method: 'DELETE',
    })
    .then(response => response.json())
    .then(data => {
        alert('Post deleted!');
    })
    .catch(error => console.error('Error deleting post:', error));
}
