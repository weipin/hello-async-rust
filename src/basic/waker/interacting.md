# Channel, passing results

## Question
How to interact with the "Waker" pattern?

### Blocking
Execution of "normal Rust code" blocks. It blocks until data is returned, and
continues with the obtained data.

Diagram 1: "Polling in a loop" blocks.
```d2
code1: Code obtains data
code2: Code processes data
future {
    ready: Is data ready? {
        shape: diamond
    }
}

code1 -> future: poll(..) {
    style.animated: true
}
future.ready -> code1: NO {
    style.animated: true
}
future.ready -> code2: YES
```

### Nonblocking
Execution of the "Waker" pattern does not block.

Diagram 2: Wakers put code execution in nonblocking mode.
```d2
current: Current thread {
    near: center-left

    code1: Code obtains data
    code2: Code processes data
    future {
        ready: Is data ready? {
            shape: diamond
        }
    }
    unknown: what to do? {
        shape: circle
        style.stroke-dash: 5
    }
}
data_source: Data source {
    near: top-center
}
another: Another thread {
    near: center-right

    waking: Waking code
    waker -> waking: wake(..)
}
poll_queue: polling queue {
    near: bottom-right
}

current.code1 -> current.future: poll(..)
current.future.ready -> current.unknown: {
    source-arrowhead.label: NO
}
current.future.ready -> current.code2 {
    source-arrowhead.label: YES
}
current.future.ready -- data_source {
    style.stroke-dash: 5
}
data_source -> another.waker {
    source-arrowhead.label: ready
}
another.waking -> poll_queue: push future
```

### The question, again
How to interact with the "Waker" pattern, as the code execution is nonblocking?

## Answer

### Solution 1: blocks the current thread until data is obtained

[Parks][1] the current thread, runs wakers and futures in dedicated threads,
and [unpark][2] the suspended thread when data is ready.

### Solution 2: interacts with data using the "channel" mechanism

"Channel" is a communication mechanism, allowing data to be sent and received
across threads. The mechanism can often work in both blocking and nonblocking
modes.

Using the "channel" mechanism, data can be sent across threads when it's ready
and obtained, and will be received in a specific thread, where the data
processing starts.

[1]: https://doc.rust-lang.org/std/thread/fn.park.html
[2]: https://doc.rust-lang.org/std/thread/struct.Thread.html#method.unpark