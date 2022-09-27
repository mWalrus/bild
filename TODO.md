# TODO
- [x] file size limitations
- [x] authentication
- [x] read url from env
- [x] install instructions
- [x] limit to image files
- [x] check if new file name is unique
- [x] rate limitations
- [x] security
- [x] delete files
- [x] install script
- [x] change routes to mount to / instead of /i
- [x] Update README with installer instructions
- [x] Support video upload
- [ ] Error handling
    - [x] Update README with env vars
    - [x] Test video upload if it still works
        - [x] Fix video upload
    - [ ] Propagate the error to the catcher
- [ ] Preserve frames in gif uploads
    - [x] basic animated webp conversion
    - [ ] preserve frame delays
- [x] delete /tmp files after handling
- [ ] Testing

# Bugs
- [ ] Throws 422 Unprocessable Entity when 429 would be the correct code
- [ ] Throws "file too large" on video of size 17MiB (limit is 20MiB)

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
- [x] Ask about other configuration
    - [x] garbage collector
        - [x] enable?
        - [x] max age of files?
    - [x] max payload size for uploads
- [x] additional dependencies
    - [x] ffmpeg
- [x] Ask for vid and img limits in installer

# Misc
- [x] gitlab ci
