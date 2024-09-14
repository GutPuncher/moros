# MOROS Syscalls

This list is unstable and subject to change between versions of MOROS.

## EXIT (0x1)

```rust
pub fn exit(code: usize) -> usize
```

## SPAWN (0x2)

```rust
pub fn spawn(path: &str) -> isize
```

## READ (0x3)

```rust
pub fn read(handle: usize, buf: &mut [u8]) -> isize
```

## WRITE (0x4)

```rust
pub fn write(handle: usize, buf: &mut [u8]) -> isize
```

## OPEN (0x5)

```rust
pub fn open(path: &str, flags: usize) -> isize
```

## CLOSE (0x6)

```rust
pub fn close(handle: usize)
```

## INFO (0x7)

```rust
pub fn info(path: &str, info: &mut FileInfo) -> isize
```

## DUP (0x8)

```rust
pub fn dup(old_handle: usize, new_handle: usize) -> isize
```

## DELETE (0x9)

```rust
pub fn delete(path: &str) -> isize
```

## STOP (0xA)

```rust
pub fn stop(code: usize)
```

The system will reboot with `0xCAFE` and halt with `0xDEAD`.

## SLEEP (0xB)

```rust
pub fn sleep(seconds: f64)
```

## POLL (0xC)

```rust
pub fn poll(list: &[(usize, IO)]) -> isize
```

## CONNECT (0xD)

```rust
pub fn connect(handle, usize, addr: &str, port: u16) -> isize
```

## LISTEN (0xE)

```rust
pub fn listen(handle, usize, port: u16) -> isize
```

## ACCEPT (0xF)

```rust
pub fn accept(handle, usize, addr: &str) -> isize
```

## ALLOC (0x10)

```rust
pub fn alloc(size: usize, align: usize) -> *mut u8
```

## FREE (0x11)

```rust
pub fn free(ptr: *mut u8, size: usize, align: usize)
```

## KIND (0x12)

```rust
pub fn kind(handle: usize) -> isize
```
