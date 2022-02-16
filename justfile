release:
    cargo build --release
    find target/release -type f -name init-nodejs-project -exec strip {} +
    find target/release -type f -name init-nodejs-project.exe -exec strip {} +

build:
    cargo build