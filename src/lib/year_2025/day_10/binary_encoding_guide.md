# Binary Encoding Guide for State Packing

A comprehensive guide to packing multiple small integers into a single large integer for efficient storage and manipulation.

---

## Table of Contents

1. [The Problem We're Solving](#the-problem-were-solving)
2. [Binary Number Basics](#binary-number-basics)
3. [Bitwise Operations Reference](#bitwise-operations-reference)
4. [The Encoding Strategy](#the-encoding-strategy)
5. [Implementing Encode/Decode](#implementing-encodedecode)
6. [Operating on Packed Data](#operating-on-packed-data)
7. [Applying to the Joltage Problem](#applying-to-the-joltage-problem)
8. [Common Pitfalls](#common-pitfalls)
9. [Practice Exercises](#practice-exercises)

---

## The Problem We're Solving

We have a **state** represented as a vector of small integers:

```rust
let state: Vec<u16> = vec![3, 5, 12, 7];
```

We want to:
1. Store this in a `HashSet` for fast lookup
2. Perform many insertions and lookups
3. Minimize memory allocations

**The issue:** `HashSet<Vec<u16>>` is slow because:
- Hashing a `Vec` requires iterating through all elements: O(n)
- Each `Vec` is a heap allocation
- Cloning creates new allocations

**The solution:** Pack all values into a single `u128`:
- Hashing a `u128` is O(1)
- No heap allocation — it's just 16 bytes on the stack
- Cloning is just copying 16 bytes

---

## Binary Number Basics

### How Integers Are Stored

Computers store numbers in binary (base 2). Each digit is called a **bit** and can be 0 or 1.

```
Decimal 13 = Binary 1101

  8   4   2   1    (place values: powers of 2)
  ↓   ↓   ↓   ↓
  1   1   0   1    = 8 + 4 + 0 + 1 = 13
```

### Integer Sizes in Rust

| Type   | Bits | Max Value                    | Use Case                |
|--------|------|------------------------------|-------------------------|
| `u8`   | 8    | 255                          | Small counters, bytes   |
| `u16`  | 16   | 65,535                       | Medium values           |
| `u32`  | 32   | 4,294,967,295                | General purpose         |
| `u64`  | 64   | ~18 quintillion              | Large values, packing   |
| `u128` | 128  | ~340 undecillion             | Very large packing      |

### Bit Positions

Bits are numbered from right to left, starting at 0:

```
Binary: 1 0 1 1 0 1 0 0
Bit #:  7 6 5 4 3 2 1 0
        ↑           ↑
    MSB (most      LSB (least
    significant)   significant)
```

The **LSB** (bit 0) represents 2^0 = 1.
The **MSB** (highest bit) represents the largest power of 2.

---

## Bitwise Operations Reference

### AND (`&`) — Masking / Testing Bits

Returns 1 only where **both** inputs have 1.

```
  1011
& 0110
------
  0010
```

**Use case:** Test if a specific bit is set.

```rust
let value = 0b1011u8;  // 11 in decimal
let mask = 0b0100u8;   // bit 2

if value & mask != 0 {
    println!("Bit 2 is set!");
}
```

**Use case:** Extract bits (masking).

```rust
let value = 0b11010110u8;
let last_4_bits = value & 0b00001111;  // = 0b0110 = 6
```

### OR (`|`) — Setting Bits

Returns 1 where **either** input has 1.

```
  1011
| 0110
------
  1111
```

**Use case:** Set a specific bit.

```rust
let mut value = 0b1000u8;
value = value | 0b0010;  // Set bit 1
// value is now 0b1010
```

**Use case:** Combine values when packing.

```rust
let packed = (a << 8) | b;  // Put 'a' in high byte, 'b' in low byte
```

### XOR (`^`) — Toggling Bits

Returns 1 where inputs **differ**.

```
  1011
^ 0110
------
  1101
```

**Use case:** Toggle specific bits.

```rust
let mut lights = 0b1100u8;
let button = 0b0110u8;  // Toggle bits 1 and 2
lights = lights ^ button;
// lights is now 0b1010
```

**Key property:** XOR is self-inverse: `a ^ b ^ b = a`

### NOT (`!`) — Inverting All Bits

Flips every bit.

```rust
let value: u8 = 0b00001111;
let inverted = !value;  // 0b11110000
```

### Left Shift (`<<`) — Multiply by Powers of 2

Shifts all bits left, filling with zeros on the right.

```
0001 << 2 = 0100  (1 becomes 4)
```

**Mathematical meaning:** `x << n` = `x * 2^n`

```rust
1u8 << 0  // = 1   (2^0)
1u8 << 1  // = 2   (2^1)
1u8 << 2  // = 4   (2^2)
1u8 << 3  // = 8   (2^3)
1u8 << 7  // = 128 (2^7)
```

**Use case:** Create a mask for bit N.

```rust
let bit_position = 3;
let mask = 1u8 << bit_position;  // 0b00001000
```

**Use case:** Position a value for packing.

```rust
let value = 5u128;     // We want this in slot 2
let slot = 2;
let positioned = value << (slot * 8);  // Shift to bits 16-23
```

### Right Shift (`>>`) — Divide by Powers of 2

Shifts all bits right, filling with zeros on the left (for unsigned).

```
1000 >> 2 = 0010  (8 becomes 2)
```

**Mathematical meaning:** `x >> n` = `x / 2^n` (integer division)

```rust
16u8 >> 1  // = 8
16u8 >> 2  // = 4
16u8 >> 4  // = 1
```

**Use case:** Extract a value from a packed integer.

```rust
let packed: u128 = 0x0705_0C03;  // Contains [3, 12, 5, 7]
let slot_2_value = (packed >> 16) & 0xFF;  // Extract slot 2 = 12
```

---

## The Encoding Strategy

### The Core Idea

Divide a large integer into fixed-size "slots". Each slot holds one value from our state.

```
u128 (128 bits total)
┌────────┬────────┬────────┬────────┬────────┬─── ... ───┐
│ slot 0 │ slot 1 │ slot 2 │ slot 3 │ slot 4 │           │
│ 8 bits │ 8 bits │ 8 bits │ 8 bits │ 8 bits │           │
└────────┴────────┴────────┴────────┴────────┴─── ... ───┘
  bits     bits     bits     bits     bits
  0-7      8-15     16-23    24-31    32-39
```

### Choosing Slot Size

The slot size determines the **maximum value** each slot can hold.

| Slot Size | Max Value | Slots in u64 | Slots in u128 |
|-----------|-----------|--------------|---------------|
| 4 bits    | 15        | 16           | 32            |
| 6 bits    | 63        | 10           | 21            |
| 8 bits    | 255       | 8            | 16            |
| 16 bits   | 65,535    | 4            | 8             |

**For this problem:** Joltage values appear to max around ~50, so 8 bits (max 255) per slot is safe. With 10 slots maximum, we need 80 bits → use `u128`.

### Visual Example

Encoding state `[3, 5, 12, 7]` with 8-bit slots:

```
State: [3, 5, 12, 7]
        ↓  ↓   ↓  ↓
Slot:   0  1   2  3

Packed u128:
Bits:   0-7     8-15    16-23   24-31   32-127
        ┌───────┬───────┬───────┬───────┬───────────────┐
        │   3   │   5   │  12   │   7   │   (unused)    │
        │0x03   │0x05   │0x0C   │0x07   │               │
        └───────┴───────┴───────┴───────┴───────────────┘

As hex: 0x0000...0007_0C05_03
As binary (last 32 bits): 00000111_00001100_00000101_00000011
```

---

## Implementing Encode/Decode

### Encoding: Vec → Packed Integer

```rust
fn encode(state: &[u16]) -> u128 {
    state
        .iter()
        .enumerate()
        .fold(0u128, |acc, (i, &value)| {
            // Shift value to its slot position and OR it in
            acc | ((value as u128) << (i * 8))
        })
}
```

**Step-by-step for `[3, 5, 12, 7]`:**

```
Initial acc = 0

i=0, value=3:
  3 << (0 * 8) = 3 << 0 = 3 = 0x03
  acc = 0 | 0x03 = 0x03

i=1, value=5:
  5 << (1 * 8) = 5 << 8 = 1280 = 0x0500
  acc = 0x03 | 0x0500 = 0x0503

i=2, value=12:
  12 << (2 * 8) = 12 << 16 = 786432 = 0x0C0000
  acc = 0x0503 | 0x0C0000 = 0x0C0503

i=3, value=7:
  7 << (3 * 8) = 7 << 24 = 117440512 = 0x07000000
  acc = 0x0C0503 | 0x07000000 = 0x070C0503

Final: 0x070C0503
```

### Decoding: Packed Integer → Vec

```rust
fn decode(packed: u128, len: usize) -> Vec<u16> {
    (0..len)
        .map(|i| {
            // Shift right to bring slot to position 0, then mask
            ((packed >> (i * 8)) & 0xFF) as u16
        })
        .collect()
}
```

**Step-by-step for `0x070C0503` with len=4:**

```
i=0:
  0x070C0503 >> 0 = 0x070C0503
  0x070C0503 & 0xFF = 0x03 = 3

i=1:
  0x070C0503 >> 8 = 0x070C05
  0x070C05 & 0xFF = 0x05 = 5

i=2:
  0x070C0503 >> 16 = 0x070C
  0x070C & 0xFF = 0x0C = 12

i=3:
  0x070C0503 >> 24 = 0x07
  0x07 & 0xFF = 0x07 = 7

Result: [3, 5, 12, 7]
```

### Why `& 0xFF`?

The mask `0xFF` (binary `11111111`) isolates just the bottom 8 bits:

```
Value:  0x070C0503
>> 8:   0x00070C05
& 0xFF: 0x00000005  ← Only the last 8 bits remain
```

Without the mask, higher slots would "leak" into our result.

---

## Operating on Packed Data

The real power comes from operating **directly** on packed data without decoding.

### Incrementing a Single Slot

To increment slot `i` by 1:

```rust
fn increment_slot(packed: u128, slot: usize) -> u128 {
    packed + (1u128 << (slot * 8))
}
```

**Why this works:**

```
packed = 0x070C0503  (slots: [3, 5, 12, 7])

To increment slot 1 (currently 5):
  1 << (1 * 8) = 1 << 8 = 0x100

  0x070C0503
+ 0x00000100
-----------
  0x070C0603  (slots: [3, 6, 12, 7])
              slot 1 is now 6!
```

**Caution:** If a slot overflows (>255 with 8-bit slots), it will corrupt the next slot. Make sure your values can't exceed the slot size.

### Reading a Single Slot

```rust
fn read_slot(packed: u128, slot: usize) -> u16 {
    ((packed >> (slot * 8)) & 0xFF) as u16
}
```

### Comparing Slots to Targets

To check if slot `i` exceeds target `t`:

```rust
fn slot_exceeds(packed: u128, slot: usize, target: u16) -> bool {
    read_slot(packed, slot) > target
}
```

Or check all slots at once:

```rust
fn any_slot_exceeds(packed: u128, targets: &[u16]) -> bool {
    targets.iter().enumerate().any(|(i, &t)| {
        read_slot(packed, i) > t
    })
}
```

### The Click Operation (Button Press)

For the joltage problem, pressing a button increments multiple slots.

**Button representation:** A `u16` bitmask where bit `i` means "affect slot `i`".

Wait — there's a twist! Your button parsing reverses bit order. Let me handle both cases:

**If buttons use reversed bit order (your current parsing):**

```rust
fn click_packed(state: u128, button: u16, num_slots: usize) -> u128 {
    let mut result = state;
    for i in 0..num_slots {
        // Check bit (num_slots - 1 - i) due to reversed parsing
        if button & (1 << (num_slots - 1 - i)) != 0 {
            result += 1u128 << (i * 8);  // Increment slot i
        }
    }
    result
}
```

**If buttons use normal bit order (bit i = slot i):**

```rust
fn click_packed_simple(state: u128, button: u16, num_slots: usize) -> u128 {
    let mut result = state;
    for i in 0..num_slots {
        if button & (1 << i) != 0 {
            result += 1u128 << (i * 8);
        }
    }
    result
}
```

### Optimized Click (Advanced)

If you want to avoid the loop entirely, you can precompute an "increment mask" for each button:

```rust
// Precompute: convert button bitmask to packed increment value
fn button_to_increment_mask(button: u16, num_slots: usize) -> u128 {
    let mut mask = 0u128;
    for i in 0..num_slots {
        if button & (1 << (num_slots - 1 - i)) != 0 {
            mask += 1u128 << (i * 8);
        }
    }
    mask
}

// Then clicking is just addition!
fn click_fast(state: u128, increment_mask: u128) -> u128 {
    state + increment_mask
}
```

**Example:**

```
Button (1, 3) with 4 slots → affects slots 1 and 3

increment_mask:
  slot 1: 1 << (1 * 8) = 0x00000100
  slot 3: 1 << (3 * 8) = 0x01000000
  combined: 0x01000100

Clicking:
  state      = 0x070C0503  ([3, 5, 12, 7])
+ increment  = 0x01000100
------------------------------
  new_state  = 0x080C0603  ([3, 6, 12, 8])
```

---

## Applying to the Joltage Problem

### Full Algorithm with Packing

```rust
use std::collections::HashSet;

fn solve_joltage(buttons: &[u16], target: &[u16]) -> usize {
    let num_slots = target.len();
    
    // Encode target
    let target_packed = encode(target);
    
    // Precompute increment masks for each button
    let increment_masks: Vec<u128> = buttons
        .iter()
        .map(|&b| button_to_increment_mask(b, num_slots))
        .collect();
    
    // BFS
    let mut seen = HashSet::new();
    let mut states = HashSet::new();
    
    let initial = 0u128;  // All zeros
    states.insert(initial);
    seen.insert(initial);
    
    let mut clicks = 0;
    
    loop {
        clicks += 1;
        let mut next_states = HashSet::new();
        
        for &state in &states {
            for &inc_mask in &increment_masks {
                let new_state = state + inc_mask;
                
                // Skip if already seen
                if seen.contains(&new_state) {
                    continue;
                }
                
                // Check if we hit target
                if new_state == target_packed {
                    return clicks;
                }
                
                // Check if any slot exceeds its target
                if any_slot_exceeds(new_state, target) {
                    continue;  // Invalid, skip
                }
                
                seen.insert(new_state);
                next_states.insert(new_state);
            }
        }
        
        if next_states.is_empty() {
            panic!("No solution found!");
        }
        
        states = next_states;
    }
}

fn encode(state: &[u16]) -> u128 {
    state.iter().enumerate().fold(0u128, |acc, (i, &v)| {
        acc | ((v as u128) << (i * 8))
    })
}

fn button_to_increment_mask(button: u16, num_slots: usize) -> u128 {
    (0..num_slots).fold(0u128, |acc, i| {
        if button & (1 << (num_slots - 1 - i)) != 0 {
            acc + (1u128 << (i * 8))
        } else {
            acc
        }
    })
}

fn any_slot_exceeds(packed: u128, targets: &[u16]) -> bool {
    targets.iter().enumerate().any(|(i, &t)| {
        ((packed >> (i * 8)) & 0xFF) as u16 > t
    })
}
```

---

## Common Pitfalls

### 1. Overflow Between Slots

If a slot value exceeds its capacity (255 for 8-bit), it overflows into the next slot:

```
State:   0x000000FF  (slot 0 = 255)
+ 1<<0:  0x00000001
-----------------------
Result:  0x00000100  (slot 0 = 0, slot 1 = 1!)  CORRUPTED!
```

**Prevention:** Always check that values won't exceed slot capacity, or use larger slots.

### 2. Signed vs Unsigned Shifts

Always use unsigned types (`u128`, not `i128`). Signed right shifts fill with the sign bit, not zeros.

### 3. Forgetting Type Suffixes

```rust
// WRONG: 1 is i32 by default, can't shift 64+ bits
let mask = 1 << 80;  // Compile error or overflow!

// RIGHT: Explicitly u128
let mask = 1u128 << 80;
```

### 4. Bit Order Confusion

Your button parsing reverses bit order (bit 0 in the mask corresponds to the LAST slot). Keep this consistent!

Document your convention:
```rust
// Convention: bit i in button mask → slot (num_slots - 1 - i)
```

### 5. Off-by-One in Slot Indexing

Slots are 0-indexed. With 4 slots:
- Slot 0: bits 0-7
- Slot 1: bits 8-15
- Slot 2: bits 16-23
- Slot 3: bits 24-31

---

## Practice Exercises

### Exercise 1: Manual Encoding

Encode `[10, 20, 30]` into a u128 with 8-bit slots. Show your work.

<details>
<summary>Solution</summary>

```
slot 0: 10 << 0  = 10      = 0x0A
slot 1: 20 << 8  = 5120    = 0x1400
slot 2: 30 << 16 = 1966080 = 0x1E0000

Combined: 0x0A | 0x1400 | 0x1E0000 = 0x1E140A
```

</details>

### Exercise 2: Manual Decoding

Decode `0x03070205` with 4 slots. What is the state vector?

<details>
<summary>Solution</summary>

```
slot 0: 0x03070205 >> 0  & 0xFF = 0x05 = 5
slot 1: 0x03070205 >> 8  & 0xFF = 0x02 = 2
slot 2: 0x03070205 >> 16 & 0xFF = 0x07 = 7
slot 3: 0x03070205 >> 24 & 0xFF = 0x03 = 3

State: [5, 2, 7, 3]
```

</details>

### Exercise 3: Increment Mask

Button `(0, 2)` with 4 slots (reversed bit order). What's the increment mask?

<details>
<summary>Solution</summary>

```
Button affects positions 0 and 2.
With reversed bit order in 4 slots:
- Position 0 → slot (4-1-0) = slot 3
- Position 2 → slot (4-1-2) = slot 1

Increment mask:
  slot 1: 1 << (1 * 8) = 0x00000100
  slot 3: 1 << (3 * 8) = 0x01000000
  combined: 0x01000100
```

</details>

### Exercise 4: Click Simulation

State `0x070C0503` ([3, 5, 12, 7]), click with increment mask `0x01000100`. What's the new state?

<details>
<summary>Solution</summary>

```
  0x070C0503
+ 0x01000100
-----------
  0x080C0603

New state: [3, 6, 12, 8]
```

</details>

---

## Quick Reference Card

```
ENCODING
  encode:   acc | (value << (slot * 8))
  decode:   (packed >> (slot * 8)) & 0xFF

OPERATIONS
  read slot i:      (packed >> (i * 8)) & 0xFF
  increment slot i: packed + (1 << (i * 8))
  click:            packed + precomputed_increment_mask

BIT OPERATIONS
  &   AND     Mask/test bits
  |   OR      Set/combine bits  
  ^   XOR     Toggle bits
  <<  LSHIFT  Move bits left (multiply by 2^n)
  >>  RSHIFT  Move bits right (divide by 2^n)

SLOT MATH (8-bit slots)
  slot i starts at bit: i * 8
  slot i ends at bit:   i * 8 + 7
  max slots in u128:    16
```

---

Happy coding at 30,000 feet!
