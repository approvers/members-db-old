# Build

FROM alpine:3.12 AS build

COPY . /src
WORKDIR /src
RUN apk add --no-cache --update curl gcc musl-dev && \
    curl https://sh.rustup.rs -sSf | sh -s -- --default-toolchain nightly -y && \
    source $HOME/.cargo/env && \
    cargo build --release


# Run

FROM alpine:3.12

WORKDIR /root
COPY database.yaml /root/database.yaml
COPY --from=build /src/target/release/members-db /usr/local/bin/members-db
EXPOSE 8080
ENTRYPOINT ["/usr/local/bin/members-db"]
CMD ["serve"]
