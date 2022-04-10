# ECE421_group_project3
A web application for Connect-4 and TOOT-OTTO using rust.

### Server Installation
1. Navigate to the 'server/' directory.
2. Install rust nightly
```
rustup install nightly
```
3. Set up nightly as the default toolchain in this directory.
```
rustup override set nightly
```

4. Install MongoDB.
```
Instructions are on MongoDB website: https://www.mongodb.com/try/download/community
```

4. Build and Run Server.
```
cargo build
cargo run
```

### Frontend Installation
1. Navigate to the 'frontend/' directory.
2. Install 'Trunk'.
```
cargo install trunk wasm-bindgen-cli
```

### Running the application
1. In one terminal navigate to '/server' and run the server code.
```
cd server
cargo run
```
2. In a second terminal navigate to '/frontend' and run the frontend code.
```
cd frontend
trunk serve --release
```
