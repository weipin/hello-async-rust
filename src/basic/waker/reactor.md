# Reactor, handling I/O event notifications

## Question
How can a waker **effectively** knows when its data is ready?

## Answer
"Effectively" means no loop. In other words, a waker should be *notified* when
its data is ready. Such effectiveness is often achieved by using the
notification mechanisms provided by the operating systems.

## Case

For futures which represent results of I/O operations, the "I/O event
notification mechanisms" provided by the operating systems can be used to
notify the wakers.

In this case, the wakers interact with some specific code that waits for the
I/O event notifications. This specific code is often called "reactor".

Diagram: A waker driven by a reactor.
```d2
classes: {
    invisible: {
        style.opacity: 0
        label: ""
    }
}

code: Code processes the data {
    near: top-right
}
kernal: Kernal space {
    grid-columns: 1
    grid-rows: 2
    vertical-gap: 80
    near: bottom-left

    notification_mechanisms: Notification mechanisms {
        epoll: epoll\n(Linux, Android, ...)
        kqueue: kqueue\n(macOS, iOS, ...)
        iocp: IOCP\n(Windows)
    }
    data: Data source

    data -> notification_mechanisms: when ready
}
future {
    near: top-left
}
reactor {
    near: center-left
}

reactor -> waker
waker -> code
code -> future: poll(..)
reactor -- future.data {
    style.stroke-dash: 5
}
kernal.notification_mechanisms -> reactor
```