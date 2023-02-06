FROM rust:slim-bullseye
RUN cargo install --version='~0.6' sqlx-cli --no-default-features --features rustls,mysql
WORKDIR /
COPY ./migrations ./migrations
COPY ./db_migration/init.sh ./db_migration/init.sh
CMD ["/bin/bash", "./db_migration/init.sh"]