
I wrote a Rust program that:

- **Starts at position 50** (obviously).
- For each instruction:
  - **Turning right**: I calculate how many multiples of 100 we cross (since every time we pass a multiple of 100, we hit 0).
  - **Turning left**: I just simulated each step because the math was getting messy with the wrapping.
- **Keep a running total of all the zeros**.

The tricky part was making sure I counted correctly when we started exactly at 0 or ended exactly at 0.  
