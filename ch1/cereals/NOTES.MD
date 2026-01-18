# Rust Memory Model: Stack, Heap, Vec, `mut`, and Bits

This document explains how Rust manages memory with `Vec<T>`, `mut` variables, and ownership, down to stack/heap layout and bits.

---

## 1. Stack vs Heap in Memory

In a running program:

High Addresses
│
│ Stack (grows downward)
│ ┌───────────────┐
│ │ grains.ptr │ ──► heap buffer
│ │ grains.len │
│ │ grains.cap │
│ └───────────────┘
│
│ … other local variables …
│
│ Heap (grows upward)
│ ┌───────────────┐
│ │ Cereal::Millet │
│ │ (uninitialized)│
│ │ ... │
│ └───────────────┘
Low Addresses



- **Stack**: fast, structured, LIFO memory for local variables and function calls
- **Heap**: dynamic memory allocated at runtime, grows/shrinks arbitrarily, managed by allocator

---

## 2. `Vec<T>` Layout (64-bit)

A vector on the stack occupies **24 bytes**:

Vec<T> (on stack) = 24 bytes
┌───────────┬───────────┬───────────┐
│ ptr (8B)  │ len (8B)  │ cap (8B)  │
└───────────┴───────────┴───────────┘

ptr: address of the heap buffer
Binary example (64-bit):

01111111 10101100 01001001 00000000 ... (64 bits)


len: number of elements initialized (Cereal::Millet → 1)

0000 0000 0000 0001  ... 64-bit representation of 1


cap: how many elements the buffer can hold before reallocating

0000 0000 0000 0100  ... for example capacity 4

3. Heap memory

When you push(Cereal::Millet):

Heap buffer (ptr points here)
┌─────────────┐
│ Cereal::Millet │ 0b000? (whatever enum value)
└─────────────┘
(unused slots for future pushes)

(unused slots for future pushes)


Vec owns this heap memory

Ownership = sole authority to free/reallocate