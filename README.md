# Observable

Provides an `Observable` type used to model push-based data sources. It always uses atomic references, allowing it to be passed to other threads. It is based on a TC39 proposal.

The `observer!` macro constructs an opaque `Observer`. You can also implement your own observer type if desired.

Requirements:

- The Rust standard library (`std`).

```rust
use std::sync::Arc;
use rust_observable::*;

fn my_observable() -> Observable<String> {
    Observable::new(Arc::new(|observer| {
        // send initial data
        observer.next("initial value".into());

        // return a cleanup function that runs on unsubscribe.
        Arc::new(|| {
            println!("cleanup on unsubscribe");
        })
    }))
}

let _ = my_observable()
    .subscribe(observer! {
        next: |value| {},
        error: |error| {},
        complete: || {},
        start: |subscription| {},
    })
    .unsubscribe();

// you can also use functional methods such as `filter` and `map`.
let _ = my_observable()
    .filter(|value| true)
    .map(|value| value);
```