# Bild - A small image uploader and server

A small, self-hosted image uploader and hoster built with Rust, [rocket](https://rocket.rs), [ffmpeg](https://ffmpeg.org/), and
[image](https://github.com/image-rs/image).

## Features
- Authenticated uploads
- File format conversion
- Clutter-free and fast viewing
- Configurable file size limitation
- Configurable request limitations
- Optional configurable periodic file deletion

## Installation
Visit [the project homepage](https://bild.waalrus.xyz) for installation instructions.

## Environment Variables
|Name|Type|Default|Description|
|-|-|-|-|
|ROCKET_SERVER_URL|String|http://localhost:1337|The URL in the returned image link|
|ROCKET_RATE_LIMIT|Integer|2|Number of allowed requests per second|
|ROCKET_FILE_AGE_WEEKS|Integer|2|Number of weeks files are allowed to live for|
|ROCKET_GARBAGE_COLLECTOR|Integer|1|Turn periodic file deletion ON (1) or OFF (0)|
|ROCKET_UPLOAD_MAX_SIZE|Integer|20|Maximum allowed file size, in MiB, the server will allow|

## URLs

### Unauthenticated
- `GET www.<your_domain>.xyz/<file_name>`: view uploaded file

### Authentication required

#### Web interfaces
- `www.<your_domain>.xyz/upload`: upload form
- `www.<your_domain>.xyz/delete/<file_name>`: file deletion form

#### Direct requests
- `www.<your_domain>.xyz/upload`
    - method: `POST`
    - headers:
        - authorization: `Bearer <your_token>`
    - body: `{data: <file>}`
- `www.<your_domain>.xyz/delete/<file_name>`
    - method: `DELETE`
    - headers:
        - authorization: `Bearer <your_token>`

