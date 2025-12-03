How to run :

1. Build the binary of day1 or next day2 if already available. -p -> to tell the compiler which project/binary to build

```
cargo build -p day1 --release
```


2. Run the app with the file_path followed by the input file path

```
./target/release/day1 ./day1/input.txt
```



Benchmarking using Hyperfine :

1. Install with apt or pacman or get it from the github repo

```
pacman -S hyperfine
```


2. Run the benchnark with --shell=none to make the result accurate (no shell noise)

```
hyperfine --shell=none "./target/release/day1 ./day1/input.txt"
```

