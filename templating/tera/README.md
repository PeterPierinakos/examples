# template_tera

Minimal example of using the template [tera](https://github.com/Keats/tera) that displays a form.

## Usage

### server

```sh
cd templating/tera
cargo run (or ``cargo watch -x run``)
# Started http server: 127.0.0.1:8080
```

### web client

- [http://localhost:8080](http://localhost:8080)
- [http://localhost:8080/non-existing-page](http://localhost:8080/non-existing-page) - 404 page rendered using template
