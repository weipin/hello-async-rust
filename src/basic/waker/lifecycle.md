# Lifecycle

A waker's life begins before a future is polled, and ends when the waker wakes.
Details:

1. Creating.

    Before a future can be polled, a waker which associates to the future is
    created.

1. Passing.

    When the future is polled, the waker is passed to it.

1. Registering.

    During the polling, if the data is returned (as its ready), nothing is done
    to the waker and its life ends.

    If the data isn't ready, the waker is "registered". Once registered:
    * The waker is stored.
    * The waker starts waiting for data through a given reactor.

    ---

    **NOTE**

    The registering repeats for each poll of the future, meaning that:
    * The stored waker will be replaced.
    * Multiple polls lead to a single waking of the "current waker".

    ---

1. Waking.

    When the reactor notifies the waker that its data is ready, the waker
    wakes.