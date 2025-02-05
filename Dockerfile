FROM rust:1.82.0 AS build

WORKDIR /usr/src/weather-bot
COPY . .
COPY .env .env

RUN cargo install --path .

FROM gcr.io/distroless/cc-debian12


# Application files

COPY --from=build /usr/local/cargo/bin/weather-bot /usr/local/bin/weather-bot

COPY --from=build /usr/src/weather-bot/.env /.env

EXPOSE 4000

CMD ["weather-bot"]
