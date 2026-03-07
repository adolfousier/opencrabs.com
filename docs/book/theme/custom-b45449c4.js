// Inject centered crab logo into the menu bar
(function() {
    var menuBar = document.querySelector('.menu-bar');
    if (menuBar && !document.querySelector('.center-logo')) {
        var logo = document.createElement('a');
        logo.className = 'center-logo';
        logo.href = 'https://opencrabs.com';
        logo.target = '_blank';
        logo.innerHTML = '<img src="/img/logo.png" alt="OpenCrabs" /> OpenCrabs';
        menuBar.appendChild(logo);
    }
})();
