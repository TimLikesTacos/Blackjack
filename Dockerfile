FROM rust:1.56

WORKDIR /usr/src/myapp
COPY . .

RUN apt-get update && apt-get install -y \
 libx11-dev libxext-dev libxft-dev libxinerama-dev \
 libxcursor-dev libxrender-dev libxfixes-dev libpango1.0-dev \
 libpng-dev libgl1-mesa-dev libglu1-mesa-dev  && rm -rf /var/lib/apt/lists/*

RUN cargo build --release && cargo install --path .

CMD ["blackjack"]
