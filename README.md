# Lerp and InverseLerp for float values
## lerp 
Find the value between two floats using a 0-1 fraction:
```rust
// Returns 1.5 because 1.5 is 0.5 distance between 1.0 and 2.0
lerp(1.0, 2.0, 0.5) 
```
## inverse_lerp
Find the fraction a given value lies at between two floats 
```rust
// Returns 0.5 because 1.5 is 0.5 of the distance between 1.0 and 2.0
inverse_lerp(1.0, 2.0, 1.5)
```
