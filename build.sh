#!/bin/bash -e
git submodule update --init --recursive
cd pomodoro-react
npm run build
cd ../
cargo clean
cargo run

# Note create release docker/podman image with 
# .
# └── simple-rust-server/
#     └── pomodoro-react/
#         └── build/
#             ├── index.html
#             ├── satic/
#             │   ├── css
#             │   ├── js
#             │   └── media
#             ├── favicon.ico
#             └── ....