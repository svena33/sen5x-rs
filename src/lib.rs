//! This library provides an embedded `no_std` driver for the [Sensirion SEN5x series](https://developer.sensirion.com/sensirion-products/sen5x-environmental-sensor-node/).
//! This driver was built using [embedded-hal](https://docs.rs/embedded-hal/) traits.
//! The implementation is based on [scd4x-rs](https://github.com/hauju/scd4x-rs) and [sgpc3-rs](https://github.com/mjaakkol/sgpc3-rs).
//! This driver aims to be compatible with `embedded-hal` v1.0.
//! This driver reimplements some common functions of [sensirion-i2c-rs](https://github.com/Sensirion/sensirion-i2c-rs) since that library supports only `embedded-hal` `<=0.2.7`.

mod sen5x;
pub use crate::sen5x::Sen5x;

pub mod commands;

pub mod types;
