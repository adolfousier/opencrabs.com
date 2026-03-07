// Inject centered crab logo into the menu bar
(function() {
    var menuBar = document.querySelector('.menu-bar');
    if (!menuBar || document.querySelector('.center-logo')) return;

    // Get path prefix from favicon (e.g. "../" on subpages, "" on root)
    var fi = document.querySelector('link[rel="shortcut icon"]');
    var base = fi ? fi.getAttribute('href').replace(/favicon[^/]*$/, '') : '';

    var logo = document.createElement('a');
    logo.className = 'center-logo';
    logo.href = 'https://opencrabs.com';
    logo.innerHTML = '<img src="' + base + 'img/logo.png" alt="" /> OpenCrabs';

    var right = menuBar.querySelector('.right-buttons');
    menuBar.insertBefore(logo, right || null);
})();
