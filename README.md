# Intro

Forked form Luca Palmieri's [Zero To Production In Rust](https://github.com/LukeMathWalker/zero-to-production) and exploring the following:

- Use Rust's default linker
- Postgres to MySQL
- Replace bash script by docker compose

## Remarks

- env file has been removed, please create your own

```
DATABASE_URL="mysql:user:password@localhost:3306/newsletter"

DB_Root_Password=super_user_password
DB_User=user
DB_Password=password
DB_Name=newsletter

App_Base_Url="http://127.0.0.1:8000" # make sure no "/" at the end
App_Port=8000
App_Hmac_Secret="super-long-and-secret-random-key-needed-to-verify-message-integrity_production"
```

- local configuration file has been removed, please create your own

```
application:
  host: 127.0.0.1
  base_url: "http://127.0.0.1"
database:
  require_ssl: false
```

- production configuration file has been removed, please create your own

```
application:
  host: 0.0.0.0
database:
  require_ssl: true
email_client:
  base_url: "https://api.postmarkapp.com"
```

## How to use

```
docker-compose build
docker-compose up -d

## License

Refer to the licence requirement of Luca Palmieri's [Zero To Production In Rust](https://github.com/LukeMathWalker/zero-to-production).