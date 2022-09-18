# Bild - A small image uploader and server

The server is made using [rocket](https://rocket.rs) and
[image](https://github.com/image-rs/image) is used for image conversion to webp.

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
Authorization tokens are generated with the `bild-auth` tool in this repository.
When you first install Bild, using the installer, a token is automatically generated, stored, and
communicated to you at the end of the installer's run, so no need to worry about that there.

You can always generate a new token by simply running the `bild-auth` tool again. This will replace
the old token, making it invalid.<br>
As with any sensitive authentication related information, keep this token safe and do NOT share it
with anyone you don't want to have access.

####  Format conversion
With Bild you can upload image files in any of the more popular image formats.
Bild will take the file you upload, guess the format from the raw uploaded data or file extension,
and then convert it to [WebP](https://en.wikipedia.org/wiki/WebP) for faster load times
when viewing the image. If an upload would fail to be converted, the server responds with a
[`500 Internal Server Error`](https://developer.mozilla.org/en-US/docs/Web/HTTP/Status/500) error.
This is usually because the file type being uploaded could not be converted to WebP due to the file
either not being an image file or being corrupted.

#### Size limitation
Bild currently limits the accepted size of an uploaded file to be 5 MiB at max. If you attempt
to upload a file larger than 5 MiB, the server will respond with a
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


