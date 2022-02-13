release:
    cargo build --release
    test -f target/release/init-nodejs-project.exe && strip target/release/init-nodejs-project.exe
    test -f target/release/init-nodejs-project && strip target/release/init-nodejs-project
