# Lart

Lart is a library for generating 2D vector art in Rust meant to be plotted.

It provides a set of primitives geometric types and operations on them alongside
a "canvas" to draw on.

Here's an example that showcases:

- automatic command line generation and parsing via the `sketch_parms` macro
- boolean operations on geometries (union, intersection, difference)
- polygon buffering

```rust
# use lart::*;

sketch_parms! {
    lines: u8 = 2,
    points: u16 = 10,
}

fn main() {
    let parms = Parms::from_cli();
    let mut doc = Sketch::new("example").with_page(Page::A6);

    let bbox = doc.page_bbox();

    let mut drawn = Geometry::new();

    for _ in 0..parms.lines {
        let mut p = Path::new();
        for _ in 0..parms.points {
            p.push(V::in_rect(&mut doc, &bbox));
        }
        let g = Geometry::from(p).buffer(-2.0);
        let g = g - &drawn;
        drawn = drawn | &g;
        doc.geometry(g);
    }

    doc.fit_to_page(20.0);
}
```

To run a sketch you can run it from the command line and the svg is
automatically saved in a `wip/<sketch_name>/` directory. The parameters defined
with the `sketch_parms!` macro are automatically parsed as long command line
options.

```shell
$ cargo run --example example
$ cargo run --example example --lines 20 --points 42
```

Check the examples directory for more usage patterns.

## skv - Sketch Viewer

lart also provides an interactive viewer in Python3 that allows to quickly
explore the parameter space and play around with the parameters while also
automatically recompiling the project if need be.

The preferred way to install it is via `pipx` with `pipx install skv`.

To use it run it from the command line run it providing it the command to run

```shell
$ skv run -- cargo run --example example --
```
