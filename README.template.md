# About

alpha-counter-cli is a command line utility using the
[`alpha-counter`](https://crates.io/crates/alpha-counter) library to provide an
alphabetic counter as would be used for numbering appendices.

It provides the `--kind` option to create counters using upper and lower ASCII
alphabets as well as any alphabet via the `--alphabet` option.
Specify an alternate starting point via the `--start` option.
Stop printing after 100 iterations, or define an alternate number via the
`--take` option.

# Examples

```
$ alpha-counter -V
!run:./target/release/alpha-counter -V
```

```
$ alpha-counter -h
!run:./target/release/alpha-counter -h
```

```
$ alpha-counter
A
B
C
...
CT
CU
CV
```

```
$ alpha-counter --kind lower
a
b
c
...
ct
cu
cv
```

```
$ alpha-counter --start 23 --take 6
!run:./target/release/alpha-counter --start 23 --take 6
```

```
$ alpha-counter --alphabet abc --take 10
!run:./target/release/alpha-counter --alphabet abc --take 10
```

