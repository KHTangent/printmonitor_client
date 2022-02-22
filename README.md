# PrintMonitor client

## Setup on Debian-derived operating systems
1. Install dependencies: 
```
sudo apt install git build-essential clang libopencv-video-dev libopencv-dev libopencv-imgproc-dev libopencv-imgcodecs-dev libopencv-videoio-dev
```
2. Install Rust (for example using [RustUp](https://rustup.rs/))
3. Clone the project
```
git clone https://github.com/KHTangent/printmonitor_client
cd printmonitor_client
```
4. (optional) Create a file named `.env` for configuration options, see below for available options.
5. Build and run the client: 
```
cargo run
```

## Configuration options
Configuration parameters can be put in a file titled `.env`, or set as environment variables. The `.env` file uses the format `VARIABLE_NAME=value`, one variable on each line. 
Example: 
```
CAMERA_INDEX=1
SERVER_ADDRESS=192.168.1.43
SERVER_PORT=8089
```
### Available options
`SERVER_ADDRESS`, IP address or domain for printmonitor_server. Required  
`SERVER_PORT`, integer, default 8089  
`CAMERA_INDEX`, integer, default 0. Index of camera to use.  
