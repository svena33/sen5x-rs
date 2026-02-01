# Sensirion I2C SEN5x Driver

This library provides an embedded `no_std` driver for the [Sensirion SEN5x series](https://developer.sensirion.com/sensirion-products/sen5x-environmental-sensor-node/).
This driver was built using [embedded-hal](https://docs.rs/embedded-hal/) traits.
The implementation is based on [scd4x-rs](https://github.com/hauju/scd4x-rs) and [sgpc3-rs](https://github.com/mjaakkol/sgpc3-rs).

This driver is compatible with `embedded-hal v1.0`.

## Features

- **`async`** - Enables async I2C support via `embedded-hal-async`. When enabled, async versions of all methods are available with an `_async` suffix (e.g., `measurement_async()`).

```toml
[dependencies]
sen5x-rs = { version = "0.3", features = ["async"] }
```

## Sensirion SEN5x

The SEN5x is a environmental sensor node for HVAC and air quality applications. It measures
particulate matter (e.g. PM 2.5), VOC, NOx, humidity, and temperature.

![SEN5x](https://github.com/svena33/sen5x-rs/raw/main/img/sen5x.png)

Further information: [Datasheet SEN5x](https://sensirion.com/media/documents/6791EFA0/62A1F68F/Sensirion_Datasheet_Environmental_Node_SEN5x.pdf)

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or
  http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or
  http://opensource.org/licenses/MIT) at your option.

### Contributing

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.
