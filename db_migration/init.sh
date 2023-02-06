#!/usr/bin/env bash

sleep 10
sqlx database create
sqlx migrate run
rm ./migrations/*