# syscaller-wrap-macro

## Example usage

```rust
wrap_syscall! {
    1 : ssize_t write(int fd, void *buf, size_t count),
    57 : int fork(),
    59 : int execve(const char *path, char *const *argv, char *const *envp),
    319 : int memfd_create(const char *name, unsigned int flags),
}
```
