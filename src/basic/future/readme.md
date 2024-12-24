# Future

## What's a Future
A future is an object which represents a value, a value that becomes ready
"some time later."


## Future as function result
Therefore, futures can be used to represent function results when those results
can only become ready "some time later." This usage of future separates results
from functions.

Diagram 1: Represented by a future, the result `bar` is separated from its
function `foo`.
```d2
direction: right

future.bar
"foo(...)" -> future
```

This separation provides new ways to process ready-some-time-later results,
making async Rust possible.

Diagram 2: Obtaining `bar` through a future.
```d2
code: Code processes bar {
    getting_value: Obtaining bar
    handling_value: Processing bar
}
foo: "foo(...)"

foo -> future
code.getting_value -> future
future.ready: Is bar ready?
future.ready.shape: diamond
future.ready -> code.getting_value: {
    source-arrowhead.label: NO
}
future.ready-> code.handling_value: {
    source-arrowhead.label: YES
}
```