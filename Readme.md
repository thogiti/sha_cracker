# SHA1 cracker in Rust

## How to use

```
# sha_cracker wordlist.txt sha1_hash
# for example
$ ./target/debug/sha_cracker wordlist.txt 7c6a61c68ef8b9b6b061b28c348bc1ed7921cb53
# or
$ ./target/debug/sha_cracker wordlist.txt 95401a269a87015f41c501aa2bfff8428713b848
```


## How to Run

```
$ cargo run -- wordlist.txt 95401a269a87015f41c501aa2bfff8428713b848
```


## How to Build

```
$ cargo build
```