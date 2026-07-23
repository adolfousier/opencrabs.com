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

// User Stories — Hermes-style category filter pills.
// Guarded so it only runs on the user-stories page.
document.addEventListener('DOMContentLoaded', function() {
    var bar = document.querySelector('.stories-filters');
    var grid = document.querySelector('.stories-grid');
    if (!bar || !grid) return;

    // Map each card's displayed category tag to a broad filter group.
    var groupOf = {
        'Accounting': 'business-ops',
        'Finance': 'business-ops',
        'Real Estate': 'business-ops',
        'Automation': 'business-ops',
        'Dev Workflow': 'product-building',
        'Mobile': 'product-building',
        'App Building': 'product-building',
        'Product building': 'product-building',
        'Research': 'research-content',
        'Reporting': 'research-content',
        'Creative · A2A': 'research-content',
        'Meta · QA': 'meta-dogfooding',
        'The thesis': 'meta-dogfooding',
        'Dogfooding': 'meta-dogfooding',
        'Team · Prod': 'devops-infra',
        'DevOps · Infra': 'devops-infra',
        'GitHub ops': 'devops-infra'
    };

    var cards = Array.prototype.slice.call(grid.querySelectorAll('.story-card'));
    function cardGroup(card) {
        var tag = card.querySelector('.story-cat');
        var name = tag ? tag.textContent.trim() : '';
        return groupOf[name] || 'other';
    }

    var pills = Array.prototype.slice.call(bar.querySelectorAll('.filter-pill'));

    // Keep the counts honest — compute them from the cards on load.
    pills.forEach(function(pill) {
        var f = pill.getAttribute('data-filter');
        var n = (f === 'all')
            ? cards.length
            : cards.filter(function(c) { return cardGroup(c) === f; }).length;
        var b = pill.querySelector('b');
        if (b) b.textContent = n;
    });

    pills.forEach(function(pill) {
        pill.addEventListener('click', function() {
            var f = pill.getAttribute('data-filter');
            pills.forEach(function(p) { p.classList.toggle('active', p === pill); });
            cards.forEach(function(card) {
                var show = (f === 'all') || (cardGroup(card) === f);
                card.classList.toggle('is-hidden', !show);
            });
        });
    });
});
