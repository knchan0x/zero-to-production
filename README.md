# Intro

Forked form Luca Palmieri's [Zero To Production In Rust](https://github.com/LukeMathWalker/zero-to-production) and exploring the following:

- Use Rust's default linker
- Postgres to MySQL

## Remarks

- env file has been removed, please create your own

```
DATABASE_URL="mysql:user:password@localhost:3306/newsletter"
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

Refer to instructions on Luca Palmieri's [Zero To Production In Rust](https://github.com/LukeMathWalker/zero-to-production).

## License

Refer to the licence requirement of Luca Palmieri's [Zero To Production In Rust](https://github.com/LukeMathWalker/zero-to-production).