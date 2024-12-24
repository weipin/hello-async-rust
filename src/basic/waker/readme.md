# Waker

A waker is an object which associates to a given future. The waker can be used
to have its associated future "polled".

Diagram: A waker which leads to its associated future being "polled" when the
data is ready.
```d2
code: Code to process the data
future {
    near: top-left
}

code -> future: poll(..)
waker -> code
future.ready: Is data ready?
future.ready.shape: diamond
future.ready-> waker: {
    source-arrowhead.label: YES
}
```

The "Waker" pattern provides a solution to the problems that "polling in a
loop" raises.

* Problem: "polling in a loop" wastes CPU time.

    Solution: instead of polling the future repeatedly, we can rely on a waker
    to have the future polled when the data is ready.

* Problem: "polling in a loop" doesn't scale.

    Solution: for a given future, when its data is ready, the waker associates
    to the future will have the exact future polled "soon enough", if not
    immediately.