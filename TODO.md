# TODO
- [ ] file size limitations
- [x] authentication
- [x] read url from env
- [x] install instructions
- [x] limit to image files
- [x] check if new file name is unique
- [x] rate limitations
- [x] security
- [ ] delete files
- [x] install script
- [ ] Performance testing
- [x] change routes to mount to / instead of /i
- [x] Update README with installer instructions
- [ ] Support video upload

# Auth
- [x] arg for only printing the raw bearer token

# Installer
- [x] use subdomain i instead of /i
    - [x] nginx configuration for subdomain
        - [x] config file name
        - [x] proxy pass to / instead of /i
    - [x] certificate generation only for subdomain
- [x] Serve install script under `bild.waalrus.xyz/bild.sh`
- [x] More testing
- [x] POSIX compliance
- [x] Ask to input rate limit

# Misc
- [ ] gitlab ci
