# Filtered images generator (made with love and photon-rs)

This project is based on [photon-rs]() and is a part of e-monkeys.tech website building project. Is still under development.

The purpose is 

(For pokemon-api-date) is freely-inspired by opinionated template [rust-rocket-template]()

Refers to TODO section to see futures improvements.

## General usage


## Examples

```bash
time cargo run img/in img/out
    Finished dev [unoptimized + debuginfo] target(s) in 1.58s
     Running `target/debug/image-generator img/in img/out`
Generate img/out/space_saturate_hsl.jpg image.
Generate img/out/space_desaturate.jpg image.
Generate img/out/space_lighten.jpg image.
Generate img/out/space_darken.jpg image.
Generate img/out/space_shift_hue.jpg image.
Generate img/out/space_solarize.jpg image.
Generate img/out/space_colorize.jpg image.
Generate img/out/space_halftone.jpg image.
Generate img/out/space_inc_brightness.jpg image.
Generate img/out/space_vertical_strips.jpg image.
Generate img/out/space_horizontal_strips.jpg image.
Generate img/out/space_tint.jpg image.
Generate img/out/space_offset.jpg image.
Generate img/out/space_offset_blue.jpg image.
Generate img/out/space_offset_red.jpg image.
Generate img/out/space_offset_green.jpg image.
Generate img/out/space_multiple_offsets.jpg image.
Generate img/out/space_primary.jpg image.
You can compare outputs images with the original in DirEntry("img/in/space.png")

real    1m44.852s
user    1m42.384s
sys     0m0.161s
```