# Rust HTTP Server

This is simple implementation of HTTP server in rust, following the requirements from [CodeCrafters Challenge](https://app.codecrafters.io/courses/http-server/overview).

## Functionality:
- parsing HTTP/1.1 request
- sending HTTP/1.1 response
- multithreading
- using async operations for I/O
- accepting files in a request 
- sending files as a resposne
- encoding (currently only `gzip`)
- few basic endpoints

Most functionality is written from scratch. Two most used crates are `tokio` (multithreading + async operations) and `flate2` (`gzip` encoding). 