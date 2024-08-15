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
alpha-counter 0.2.3
```

```
$ alpha-counter -h
Alphabetic Counter (A, B, C, ..., X, Y, Z, AA, AB, AC, ...)

Usage: alpha-counter [OPTIONS]

Options:
  -k, --kind <KIND>          Kind (upper, lower) [default: upper]
  -s, --start <START>        Start [default: 0]
  -t, --take <TAKE>          Take [default: 100]
  -a, --alphabet <ALPHABET>  Custom alphabet
  -h, --help                 Print help
  -V, --version              Print version
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
X
Y
Z
AA
AB
AC
```

```
$ alpha-counter --alphabet abc --take 10
a
b
c
aa
ab
ac
ba
bb
bc
ca
```

