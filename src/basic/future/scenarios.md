# Scenarios

Futures, while they can represent any type of values, are often used in
specific scenarios.

One of the scenarios is I/O operation, such as reading from the Internet, or
writing to a local file system, etc.

Diagram: Function `foo` obtains data through the BSD socket interface `read` in
nonblocking mode.
```d2
grid-rows: 1
horizontal-gap: 140

userspace: User space
kernal: Kernal space
userspace.foo: "foo(...)"
kernal.read: "read(...) in nonblocking mode"

userspace.foo -> userspace.future
userspace.future -> kernal.read: when polled

kernal.read.ready: Is data available to read?
kernal.read.ready.shape: diamond
kernal.read.ready -> userspace.poll.Ready: YES
kernal.read.ready -> userspace.poll.Pending: NO

userspace {
    grid-rows: 4
    vertical-gap: 100

    style: {
        stroke-dash: 5
        font-size: 40
        italic: true
    }
}
kernal {
    grid-rows: 3
    vertical-gap: 100

    style: {
        stroke-dash: 5
        font-size: 40
        italic: true
    }
}
kernal.read.ready.height: 300
```