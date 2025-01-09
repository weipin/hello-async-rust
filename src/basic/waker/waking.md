# Waking, pushing futures to a polling queue

## Question
How can a waker have its future "polled"?

## Answer
A waker calls specific "waking" code when the waker is notified that its data
is ready. The "waking" code locates the future that the waker associates to,
and pushes the future to a "polling queue". A waker's job is finished after
"waking".

With a "polling queue", wakers are decoupled from "external code" which pops
futures from the queue, polls the futures and dispatches data.

Diagram: a waker wakes.
```d2
waking: Waking code\n(shared between wakers)
queue: polling queue {
    grid-columns: 8
    horizontal-gap: 15

    future1: future
    future2: future
    dot1: .. {
        style.stroke-width: 0
        style.fill: transparent
    }
    future3: future
    dot2: .. {
        style.stroke-width: 0
        style.fill: transparent
    }
    future4: future
    future5: future
    dot3: .. {
        style.stroke-width: 0
        style.fill: transparent
    }
}
handling: External code {
    style.stroke-dash: 5
}

waker -> waking: wake(..)
waking -> queue: push future
handling -> queue: pop future {
    style.stroke-dash: 5
}
```