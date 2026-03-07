// Make the sidebar book title link back to the main site
document.addEventListener('DOMContentLoaded', function() {
    var title = document.querySelector('.sidebar .sidebar-scrollbox a.sidebar-logo, .sidebar h1.menu-title');
    if (title && !title.closest('a[href*="opencrabs.com"]')) {
        title.style.cursor = 'pointer';
        title.addEventListener('click', function(e) {
            e.preventDefault();
            window.location.href = 'https://opencrabs.com';
        });
    }
});
