# Future Tree

When a Future compose with other Futures, these "other Futures" may also be
composed with more Futures, hense constructing a Future "tree".

Diagram: a Future tree that puts A, B and D together.
```d2
caller
a: A {
    continue: Is B ready? {
        shape: diamond
    }
    process: Code processes data
    b: B {
        continue: C and D both ready? {
            shape: diamond
        }
        c: C
        d: D {
            continue: E or F or G ready? {
                shape: diamond
            }
            e: E
            f: F
            g: G
        }
    }
}

poll {
    ready: Ready
    pending: Pending
}

caller -> a: poll(..)
a.continue -> a.b: poll(..) {
    style.stroke-dash: 5
}
a.continue -> a.process: YES
a.continue -> poll.pending: NO
a.process -> poll.ready

a.b.continue -> a.b.c: poll(..) {
    style.stroke-dash: 5
}
a.b.continue -> a.b.d: poll(..) {
    style.stroke-dash: 5
}
a.b.d.continue -> a.b.d.e: poll(..) {
    style.stroke-dash: 5
}
a.b.d.continue -> a.b.d.f: poll(..) {
    style.stroke-dash: 5
}
a.b.d.continue -> a.b.d.g: poll(..) {
    style.stroke-dash: 5
}
```