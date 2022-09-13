# Bild - A small image uploader and server

The server is made using [rocket](https://rocket.rs) and
[image](https://github.com/image-rs/image) is used for image conversion to webp.

## Disclaimer
This is a young project and im working out the details as I go since I didn't
really have a plan when starting this.
<br>What I'm trying to say is:<br>
Use at your own risk :-)

## Pre-requisites
This guide is intended for ubuntu server 20.04.

<ins>__NOTE__: replace all occurrences of `your-domain.com` with your actual domain name.</ins>
### List of programs that needs to be installed:
- `nginx`
- `certbot` and `python3-certbot-nginx`
- `rust` (easiest with `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`)
- `gcc`

### Misc setup
- Start nginx if not running (check status with `sudo systemctl status nginx`): `sudo systemctl start nginx && sudo systemctl enable nginx`
- Set rustup to nightly toolchain: `rustup default nightly`

## Publicly Available Paths
- `https://your-domain.com/i/<IMAGE_FILE_NAME>`: Gets an image on the server with the specified name.

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

2. Clone repo: `git clone https://gitlab.com/mWalrus/bild.git`
    - make sure the cloned directory is owned by www-data (`sudo chown www-data: bild/`)

3. Enter the cloned directory: `cd bild`

4. Compile: `rustup run nightly cargo build --release`
    - This will create two binaries, `bild-auth` and `bild-server`. `bild-auth` will be explained in the next section.

### Authentication
__Note__: Make sure you have compiled as described in the step above.

The `bild-auth` binary created is a small helper program to generate a secure token for authentication.
This token is saved to `/etc/bild-server/auth.key` which will be read by bild-server when it's running.
The `bild-auth` tool also produces the header string which can be added to chatterino later when setting up the uploader. 

1. Run bild-auth: `./target/release/bild-auth`

Copy the `Authorization: Bearer XXXXXXXXXXXXXX` it outputs.

Thats it for authentication!

### Systemd service
1. Create service file: `touch /etc/systemd/system/bild-server.service`

2. Edit the file and add the following:<br>
<ins>__NOTE__: The `ROCKET_RATE_LIMIT` environment variable is optional, if you omit this variable it will default to allow two (2) requests per second.</ins>
```
[Unit]
Description=My Rocket application for your-domain.com

[Service]
User=www-data
Group=www-data
# The user www-data should probably own that directory
WorkingDirectory=/var/www/bild
Environment="ROCKET_ENV=prod"
Environment="ROCKET_ADDRESS=127.0.0.1"
Environment="ROCKET_PORT=1337"
Environment="ROCKET_LOG=critical"
Environment="ROCKET_SERVER_URL=https://your-domain.com"
# Optional environment variable
# Environment="ROCKET_RATE_LIMIT=2" # default is 2
ExecStart=/var/www/bild/target/release/bild-server

[Install]
WantedBy=multi-user.target
```

3. Start and enable the service: `sudo systemctl start bild-server.service && sudo systemctl enable bild-server.service`

### Chatterino setup
In chatterino settings -> External tools -> Image Uploader, enter in the following:

- Request URL: `https://your-domain.com/i/upload`
- Form field: `data`
- Extra Headers: `Authorization: Bearer XXXXXXXXXXXXXX` (replace this with the output from `bild-auth`)
- Image link: `{url}`

Done! :)
