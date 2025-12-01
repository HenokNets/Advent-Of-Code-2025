# Advent of Code – Day 1 Part 1 Solution



## How it works

The program:

* Reads all input from `stdin`
* Starts the dial at position **50**
* Applies each rotation (`L` or `R`)
* Counts how many times the dial ends up at **0**
* Prints the final count

## Run

```bash
cargo run < input.txt
```

## Example

Input:

```
L68
L30
R48
...
```

Output:

```
3
```

## Notes

* Uses `rem_euclid(100)` for correct wrap-around behavior.
* Straightforward and efficient implementation in Rust.
