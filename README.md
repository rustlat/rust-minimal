# Meetup: Introducción a la programación con Rust

Este repositorio está dedicado a documentar proyectos introductorios en lenguaje Rust. Dirigido a la comunidad Rust Latam y cualquier interesado en formar parte de alguno de nuestros futuros talleres, meetups y hackathons.

El repositorio está estructurado en los siguientes `crates`:
- quadrant: Ejemplo introductorio al lenguaje Rust
- opeanapi: Hola Mundo en un Web Server
- openai: Integración de un LLM con Rust

## Indicaciones generales para ejecutar un `crate`
Mediante la interfaz de línea de comandos ubicarse en el directorio raiz, luego seguir las siguientes instrucciones para cada crate.

### Quadrant

```rust
cargo run --bin quadrant
```

### Open API

```rust
cargo run --bin openapi
```

Una vez que el servidor esté siendo ejecutado, realizar una petición HTTP mediante un cliente (e.g. Postman, Google Chrome) de la siguiente manera: `http://localhost:3000/api/hello?name=john`

### OpenAI
Crear un archivo `.env` que contenga las siguientes variables de entorno:

```bash
OPENAI_API_KEY=<api-key>
OPENAI_MODEL_NAME=<llm-model-name>
```

Finalmente, ejecutar el crate de manera similar a los casos anteriores.

## Additional Resources:
- [Rust Language Cheat Sheet](https://cheats.rs/)
- [A half-hour to learn Rust](https://fasterthanli.me/articles/a-half-hour-to-learn-rust)
- [The Rust Programming Language](https://doc.rust-lang.org/book/title-page.html)
- [Rust by Example](https://doc.rust-lang.org/rust-by-example/index.html)

