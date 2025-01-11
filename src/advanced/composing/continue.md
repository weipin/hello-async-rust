# Example: continue

Future A waits for Future B, and continues with data obtained from B.
```d2
caller
a: A {
    continue: Is B ready? {
        shape: diamond
    }
    process: Code processes data
    b: B
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
```

