# Example: select

Future D selects future E, F, and G, waiting for any of the three to complete.
```d2
caller
d: D {
    continue: E or F or G ready? {
        shape: diamond
    }
    e: E
    f: F
    g: G
}

poll {
    ready: Ready
    pending: Pending
}

caller -> d: poll(..)
d.continue -> d.e: poll(..) {
    style.stroke-dash: 5
}
d.continue -> d.f: poll(..) {
    style.stroke-dash: 5
}
d.continue -> d.g: poll(..) {
    style.stroke-dash: 5
}
d.continue -> poll.pending: NO
d.continue -> poll.ready: YES
```

