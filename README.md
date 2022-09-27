# Bild - A small image uploader and server

A small, self-hosted image uploader and hoster built with Rust, [rocket](https://rocket.rs), and
[image](https://github.com/image-rs/image).

## Disclaimer
This is a young project and im working out the details as I go since I didn't
really have a plan when starting this.
<br>What I'm trying to say is:<br>
Use at your own risk :-)

## Installation
Visit [the project homepage](https://bild.waalrus.xyz) for installation instructions.

## Features

#### Authenticated uploads
In order to upload a file to your Bild instance, you are required to supply an authorization token.
Authorization tokens are generated with the `bild-auth` tool in this repository and are cryptographically strong.
When you first install Bild, using the installer, a token is automatically generated, stored, and
communicated to you at the end of the installer's run, so no need to worry about that there.

You can always generate a new token by simply running the `bild-auth` tool again. This will replace
the old token, making it invalid.<br>
As with any sensitive authentication related information, keep this token safe and do __NOT__ share it
with anyone you don't want to have access.

####  Format conversion
With Bild you can upload both images and videos. Bild supports a wide variety of [image formats](https://github.com/image-rs/image/blob/master/README.md#supported-image-formats)
and [video formats](https://github.com/bojand/infer#video).

##### Images
When you upload an image to Bild, it will take the image, decode its contents and converts it to
[WebP](https://en.wikipedia.org/wiki/WebP) in order to make the file's footprint on your server
smaller.
Bild also supports converting animated `.gif` files into animated `.webp` files.

##### Videos
When you upload videos to Bild, Bild makes sure to convert the video to `.mp4` in order to support
the wider range of browsers out there. For this Bild uses `ffmpeg` under the hood.

##### Failures
If an upload fails the server responds with a [`500 Internal Server Error`](https://developer.mozilla.org/en-US/docs/Web/HTTP/Status/500) error.
This is usually because the file being uploaded isn't supported, which means Bild won't be able to
convert the file into the final format.
If the file being uploaded is of a supported format and it still fails, it could be because the
file is corrupted.

#### Clutter-free and fast viewing
Bild serves images, and only images. There is no need for fancy a fancy landing page slowing down response times.
The fact that Bild converts the uploaded images to WebP also helps improve said response time, since WebP usually
shrinks the file size.

#### Size limitation
Bild currently limits the accepted size of an uploaded file to be 20 MiB at max. If you attempt
to upload a file larger than 20 MiB, the server will respond with a
[`413 Payload Too Large`](https://developer.mozilla.org/en-US/docs/Web/HTTP/Status/413) error.

#### Request limitations
Bild is configured to allow a maximum of 2 uploads per second. This default should be enough, but
you can of course configure this to your liking. The install script will ask if you want to change
the rate limit when you run it.

If the server is detecting too many requests it will abort any new requests, returning a
[`429 Too Many Requests`](https://developer.mozilla.org/en-US/docs/Web/HTTP/Status/429) error.

#### Garbage collection
In order to conserve storage space on your server, Bild performs a periodic deletion of uploads
older than __2 weeks__. This garbage collection runs every 2 hours from the time the server starts.

This will be configurable in the future.

## Information

#### Environment Variables
|Name|Type|Default|Description|
|-|-|-|-|
|ROCKET_SERVER_URL|String|http://localhost:1337|The URL in the returned image link|
|ROCKET_RATE_LIMIT|Integer|2|Number of allowed requests per second|
|ROCKET_FILE_AGE_WEEKS|Integer|2|Number of weeks files are allowed to live for|
|ROCKET_GARBAGE_COLLECTOR|Integer|1|Turn old file deletion ON (1) or OFF (0)|
|ROCKET_VIDEO_UPLOAD_MAX_SIZE|Integer|20|Maximum allowed video file size, in MiB, the server will allow|
|ROCKET_IMAGE_UPLOAD_MAX_SIZE|Integer|5|Maximum allowed image file size, in MiB, the server will allow|
