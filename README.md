# bench-llvm

Benchmark to compare the performance of LLVM and bytecode VMs.

## Results

| Runtime    | Time (ns/iter) |
| ---------- | -------------: |
| native     |      2,783,106 |
| jit        |      2,830,785 |
| vm         |    129,204,107 |
| vm(unsafe) |    111,182,586 |
| python     |     44,974,225 |

## Test case
```rust
const MOD: i64 = 100000007;
const N: i64 = 1000000;
const ANS: i64 = 59273026;

let mut a = 1;
for i in 1..N {
    a = (a * i) % MOD;
}
assert_eq!(a, ANS);
```

