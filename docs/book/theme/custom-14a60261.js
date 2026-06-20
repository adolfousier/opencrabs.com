// Make the book title link back to the main site
document.addEventListener('DOMContentLoaded', function() {
    var title = document.querySelector('h1.menu-title');
    if (title) {
        title.style.cursor = 'pointer';
        title.addEventListener('click', function() {
            window.location.href = 'https://opencrabs.com';
        });
    }
});
