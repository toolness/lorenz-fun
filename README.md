This repository contains a very simple Rust implementation of the
[Lorenz system][], which I was inspired to explore after first listening to
[Roguelike Radio Episode 149: Chaos Theory][rr], and then reading James Gleick's
[Chaos: Making a New Science][gleick].

This system consists of three equations that are sensitive to initial conditions.
It's most easily explored by running this project's
[3D visualization of the Lorenz system][web].

Here you can see the basic behavior of the system over time; once you've seen
enough, you can press <kbd>2</kbd> to see a visualization of two systems
running concurrently: the second has almost identical initial conditions
as the first, except that one of its coordinates is altered by
0.0000001. Both systems appear to behave identically at first, but after
several seconds diverge from one another, illustrating the Chaos
principle at work (well, at least insofar as I understand it).

## Quick start

You will need [Rust](https://www.rust-lang.org/).

```
cargo run --bin svg > lorenz.svg
```

Then open `lorenz.svg` in a browser.

You can also view a 3D visualization:

```
cargo run --bin three
```

## Building for the web

To run the 3D visualization on the web, you will also need
[node](https://nodejs.org).

Run the following:

```
cargo install -f cargo-web
npm install
npm run build
npm start
```

Then visit http://localhost:3000 in a browser.

### Deploying for the web

Run the following:

```
npm run build:release
```

This will create an optimized build in the `dist` directory, which
you can deploy to any static web host. Make sure it serves
`.wasm` files with the MIME type `application/wasm`.

If you want to host the site on GitHub Pages, you can deploy with:

```
npm run deploy
```

[Lorenz system]: https://en.wikipedia.org/wiki/Lorenz_system
[rr]: http://www.roguelikeradio.com/2018/12/episode-149-chaos-theory.html
[gleick]: https://en.wikipedia.org/wiki/Chaos:_Making_a_New_Science
[web]: https://toolness.github.io/lorenz-fun/
