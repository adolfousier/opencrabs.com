# HTTP redirect to HTTPS
server {
    listen 80;
    server_name docs.opencrabs.com;
    return 301 https://$host$request_uri;
}

# Main HTTPS Server Block
server {
    listen 443 ssl;
    server_name docs.opencrabs.com;

    # SSL settings (managed by Certbot)
    ssl_certificate /etc/letsencrypt/live/docs.opencrabs.com/fullchain.pem;
    ssl_certificate_key /etc/letsencrypt/live/docs.opencrabs.com/privkey.pem;
    include /etc/letsencrypt/options-ssl-nginx.conf;
    ssl_dhparam /etc/letsencrypt/ssl-dhparams.pem;

    # Security headers
    add_header X-Frame-Options "SAMEORIGIN";
    add_header X-XSS-Protection "1; mode=block";
    add_header X-Content-Type-Options "nosniff";
    add_header Strict-Transport-Security "max-age=31536000; includeSubDomains" always;

    # Serve static mdBook output
    root /srv/rs/opencrabs.com/docs/book;
    index index.html;

    location / {
        try_files $uri $uri/ =404;
    }

    # Gzip compression
    gzip on;
    gzip_vary on;
    gzip_min_length 1000;
    gzip_proxied any;
    gzip_types text/plain text/css application/json application/javascript text/xml application/xml application/xml+rss text/javascript;
    gzip_disable "msie6";
}
