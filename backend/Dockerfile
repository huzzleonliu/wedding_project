FROM rust:latest
WORKDIR /usr/src/app/backend

# 安装CUPS和必要的工具
RUN apt-get update && apt-get install -y \
    cups-client \
    cups-common \
    printer-driver-all \
    sudo \
    && rm -rf /var/lib/apt/lists/*

# 复制项目文件
COPY . .

# 构建应用
RUN cargo build --manifest-path ./printer/Cargo.toml --release

CMD ["./printer/target/release/backend"]
