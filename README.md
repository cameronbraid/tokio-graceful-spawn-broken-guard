
Issue is when the guard is upgraded in a spawned task


```sh
cargo run
```

produces :

```
Doing work
Ok(8.147µs)
```

expected :

```
Doing work
Work done
Ok(1.s)
```
