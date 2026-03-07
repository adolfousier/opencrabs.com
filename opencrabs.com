# HTTP redirect to HTTPS
server {
    if ($host = www.opencrabs.com) {
        return 301 https://$host$request_uri;
    } # managed by Certbot

    if ($host = opencrabs.com) {
        return 301 https://$host$request_uri;
    } # managed by Certbot

    listen 80;
    server_name opencrabs.com www.opencrabs.com;

    # Redirect all HTTP traffic to HTTPS
    return 301 https://$host$request_uri;
}

# Redirect HTTPS www to non-www
server {
    listen 443 ssl;
    server_name www.opencrabs.com;
    ssl_certificate /etc/letsencrypt/live/opencrabs.com/fullchain.pem; # managed by Certbot
    ssl_certificate_key /etc/letsencrypt/live/opencrabs.com/privkey.pem; # managed by Certbot
    include /etc/letsencrypt/options-ssl-nginx.conf;
    ssl_dhparam /etc/letsencrypt/ssl-dhparams.pem;

    return 301 https://opencrabs.com$request_uri;
}

# Main HTTPS Server Block
server {
    listen 443 ssl;
    server_name opencrabs.com;

    # SSL settings (managed by Certbot)
    ssl_certificate /etc/letsencrypt/live/opencrabs.com/fullchain.pem; # managed by Certbot
    ssl_certificate_key /etc/letsencrypt/live/opencrabs.com/privkey.pem; # managed by Certbot
    include /etc/letsencrypt/options-ssl-nginx.conf;
    ssl_dhparam /etc/letsencrypt/ssl-dhparams.pem;

    # Security headers
    add_header X-Frame-Options "SAMEORIGIN";
    add_header X-XSS-Protection "1; mode=block";
    add_header X-Content-Type-Options "nosniff";
    add_header Strict-Transport-Security "max-age=31536000; includeSubDomains" always;

    # Proxy to Docker container on port 19283
    location / {
        proxy_pass http://localhost:19283;
        proxy_http_version 1.1;
        proxy_set_header Host $host;
        proxy_set_header X-Real-IP $remote_addr;
        proxy_set_header X-Forwarded-For $proxy_add_x_forwarded_for;
        proxy_set_header X-Forwarded-Proto $scheme;
    }

    # Gzip compression
    gzip on;
    gzip_vary on;
    gzip_min_length 1000;
    gzip_proxied any;
    gzip_types text/plain text/css application/json application/javascript text/xml application/xml application/xml+rss text/javascript application/wasm;
    gzip_disable "msie6";
}
