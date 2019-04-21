Fun with the [Lorenz system][].

## Quick start

```
cargo run --bin svg > lorenz.svg
```

Then open `lorenz.svg` in a browser.

You can also view a 3D visualization:

```
cargo run --bin three
```

## Building for the web

To run the 3D visualization on the web, run the following:

```
cargo install -f cargo-web
npm install
npm run build
npm start
```

Then visit http://localhost:3000 in a browser.

[Lorenz system]: https://en.wikipedia.org/wiki/Lorenz_system
