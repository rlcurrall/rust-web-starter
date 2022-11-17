################################################################################
# Development Builder                                                          #
################################################################################
FROM rustlang/rust:nightly AS api_builder

RUN apt-get update && apt-get install libssl-dev -y

# create a new empty shell project
RUN USER=root cargo new --bin app
WORKDIR /app
COPY Cargo.toml Cargo.lock ./
RUN cargo build
RUN rm src/*.rs

# copy app code
COPY src src
COPY migrations migrations
COPY ./sqlx-data.json .

# build source
RUN rm ./target/debug/deps/app*
RUN cargo build


################################################################################
# Development Server                                                           #
################################################################################
FROM api_builder AS dev_server

WORKDIR /app
RUN cargo install cargo-watch

COPY conf conf
COPY migrations migrations
COPY docker/wait-for-it ./wait-for-it
COPY .env .

EXPOSE 3000
VOLUME [ "/usr/local/cargo" ]


################################################################################
# Production Builder                                                           #
################################################################################
FROM rustlang/rust:nightly AS prod_builder

RUN apt-get update && apt-get install libssl-dev -y

# create a new empty shell project
RUN USER=root cargo new --bin app
WORKDIR /app
COPY Cargo.toml Cargo.lock ./
RUN cargo build --release
RUN rm src/*.rs

# copy app code
COPY src src
COPY migrations migrations
COPY ./sqlx-data.json .

# build source
RUN rm ./target/release/deps/app*
RUN cargo build --release


################################################################################
# Production Server                                                            #
################################################################################
FROM debian:buster-slim AS prod_server

WORKDIR /app
RUN apt-get update && apt-get install libssl-dev -y

COPY --from=prod_builder /app/target/release/server ./server
COPY migrations migrations
COPY docker/wait-for-it ./wait-for-it

VOLUME [ "/app/conf" ]
