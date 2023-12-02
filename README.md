Expected results :

```sh
cargo run --bin main
```

produces :

```
guard is set
Err(TimeoutError(1s))
```


Issue is when the guard is upgraded in a spawned task


```sh
cargo run --bin spawn
```

produces :

```
guard is set
Ok(14.999µs)
```