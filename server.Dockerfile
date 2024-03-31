FROM rust:latest as server_builder
WORKDIR /app

COPY . .

RUN cargo install --path thepipelinetool_server --bin server

FROM ghcr.io/cirruslabs/flutter:stable as web_builder
WORKDIR /app

COPY thepipelinetool_ui .
WORKDIR /app/thepipelinetool_ui

RUN flutter pub get
RUN flutter build web --release

FROM rust:latest
WORKDIR /server

# ARG VERSION

# RUN curl -L -o web.zip https://github.com/thepipelinetool/thepipelinetool_ui/releases/download/${VERSION}/web.zip
# RUN unzip web.zip -d temp
# RUN mkdir static && mv temp/* static/
# RUN rm -r temp
# RUN rm web.zip

COPY --from=web_builder /app/thepipelinetool_ui/build/web static/

COPY --from=server_builder /usr/local/cargo/bin/server /usr/local/bin/server

EXPOSE 8000

CMD server