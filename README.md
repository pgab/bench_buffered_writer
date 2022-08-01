To run this, you need to be on nightly:
```
cargo +nightly bench
```
Adjust `DATA_SIZE` as you like. I get the following results:

DATA_SIZE = 100_000_000:
========================
```
running 3 tests
test tests::write_all             ... bench:  89,937,856 ns/iter (+/- 5,922,548)
test tests::write_one_by_one      ... bench: 1,063,806,295 ns/iter (+/- 43,490,031)
test tests::write_to_mem_then_all ... bench: 606,946,848 ns/iter (+/- 67,702,694)

test result: ok. 0 passed; 0 failed; 0 ignored; 3 measured; 0 filtered out
```
DATA_SIZE = 100_000:
====================
```
running 3 tests
test tests::write_all             ... bench:     158,004 ns/iter (+/- 28,055)
test tests::write_one_by_one      ... bench:   1,115,747 ns/iter (+/- 227,329)
test tests::write_to_mem_then_all ... bench:     595,559 ns/iter (+/- 12,098)

test result: ok. 0 passed; 0 failed; 0 ignored; 3 measured; 0 filtered out
```