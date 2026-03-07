# Changelog

<div id="changelog-content">Loading changelog from GitHub...</div>

<script>
(function() {
    fetch('https://api.github.com/repos/adolfousier/opencrabs/contents/CHANGELOG.md', {
        headers: { 'Accept': 'application/vnd.github.v3.raw' }
    })
    .then(function(r) { return r.text(); })
    .then(function(md) {
        // Strip the "# Changelog" header and preamble lines
        var lines = md.split('\n');
        var start = 0;
        for (var i = 0; i < lines.length; i++) {
            if (lines[i].match(/^## \[/)) { start = i; break; }
        }
        var body = lines.slice(start).join('\n');

        // Simple markdown to HTML
        body = body
            .replace(/^### (.+)$/gm, '<h3>$1</h3>')
            .replace(/^## (.+)$/gm, '<h2>$1</h2>')
            .replace(/^\*\*(.+?)\*\*/gm, '<strong>$1</strong>')
            .replace(/^- (.+)$/gm, '<li>$1</li>')
            .replace(/`([^`]+)`/g, '<code>$1</code>')
            .replace(/\[([^\]]+)\]\(([^)]+)\)/g, '<a href="$2">$1</a>')
            .replace(/^> (.+)$/gm, '<blockquote>$1</blockquote>')
            .replace(/(<li>.*<\/li>\n?)+/g, function(m) { return '<ul>' + m + '</ul>'; })
            .replace(/\n\n/g, '<br/>');

        document.getElementById('changelog-content').innerHTML = body;
    })
    .catch(function() {
        document.getElementById('changelog-content').innerHTML =
            '<p>Failed to load changelog. <a href="https://github.com/adolfousier/opencrabs/blob/main/CHANGELOG.md">View on GitHub</a></p>';
    });
})();
</script>
