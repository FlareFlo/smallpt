# Run with optimal settings
```shell
RUSTFLAGS="--emit=asm -Ctarget-cpu=native" cargo run --release $SAMPLE_COUNT
```

# View image
Image viewers like gwenview support the PPM format, otherwise, converting using tools like imagemacick are required
```shell
gwenview image.ppm
```


# Convert PPM to PNG using imagemacick
```shell
convert image.ppm image.png
```