# Image Server - An image uploader for chatterino

A small image uploader and hoster written in rust.
The server is made using [rocket](https://rocket.rs) and
[image](https://github.com/image-rs/image) is used for image conversion to webp.

## Disclaimer
This is a young project and im working out the details as I go since I didn't
really have a plan when starting this.
What I'm trying to say is:<br>
Use at your own risk :-)

## Pre-requisites
This guide is intended for ubuntu server 20.04.

__NOTE__: replace all occurrences of `your-domain.com` should be replaced with your actual domain name. 
### List of programs that needs to be installed:
- `nginx`
- `certbot` and `python3-certbot-nginx`
- `rust` (easiest with `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`)
- `gcc`

### Misc setup
- Start nginx if not running (check status with `sudo systemctl status nginx`): `sudo systemctl start nginx && sudo systemctl enable nginx`
- Set rustup to nightly toolchain: `rustup default nightly`

## Install Steps

### Nginx config
1. Create a new nginx config file:
`touch /etc/nginx/sites-available/your-domain.com.conf`

2. Edit the same file and add:
```
  server {
    server_name your-domain.com www.your-domain.com
    
    location /i/ {
      proxy_pass http://127.0.0.1:1337;
    }
  }
```

3. Symlink the file to sites-enabled:
`ln -s /etc/nginx/sites-available/your-domain.com.conf /etc/nginx/sites-enabled/`

4. Generate certificates for your domain: `certbot --nginx`
  - Choose the (sub)domains you want to generate certs for.
  - Choose redirect (option 2) when prompted.
  
5. Reload nginx: `sudo systemctl reload nginx`


### Compiling
1. Go to www directory: `cd /var/www/`

2. Clone repo: `git clone https://gitlab.com/mWalrus/image-server`
  - make sure the cloned directory is owned by www-data (`sudo chown www-data: image-server/`)

3. Enter the cloned directory: `cd image-server`

4. Compile: `rustup run nightly cargo build --release`

### Systemd service
1. Create service file: `touch /etc/systemd/system/your-domain.com.service`

2. Edit the file and add the following:
```
[Unit]
Description=My Rocket application for your-domain.tld

[Service]
User=www-data
Group=www-data
# The user www-data should probably own that directory
WorkingDirectory=/var/www/image-server
Environment="ROCKET_ENV=prod"
Environment="ROCKET_ADDRESS=127.0.0.1"
Environment="ROCKET_PORT=1337"
Environment="ROCKET_LOG=critical"
Environment="ROCKET_SERVER_URL=https://your-domain.com"
ExecStart=/var/www/image-server/target/release/image-server

[Install]
WantedBy=multi-user.target
```

3. Start and enable the service: `sudo systemctl start your-domain.com.service && sudo systemctl enable your-domain.com.service`
