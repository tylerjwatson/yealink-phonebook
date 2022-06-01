FROM rust:1.61 as builder
WORKDIR /usr/src/yealink-phonebook
COPY . .
RUN cargo install --path .

FROM debian:bullseye
RUN apt-get update && apt-get install -y ca-certificates
COPY --from=builder /usr/local/cargo/bin/yealink-phonebook /usr/local/bin/yealink-phonebook
CMD ["yealink-phonebook"]