# Scales

[![Build Status](https://api.travis-ci.com/bbmsoft/scales.svg?branch=develop)](https://travis-ci.com/github/bbmsoft/frost)
[![Current Crates.io Version](https://img.shields.io/crates/v/scales.svg)](https://crates.io/crates/scales)

A utility library for converting values between scales.

## What does it do?

It provides reusable converters that make translating ranged values from one scale to another easy. Imagine you have a slider in your UI that has a linear internal value but it controls a logarithmic parameter. Normally you would be doing something like this:

```
let relative = (slider.value() - slider.min()) / (slider.max() - slider.min());
let log_range = parameter.max().log10() - parameter.min().log10();
let exp = parameter.min().log10() + relative * log_range;
let new_value = 10f64.powf(exp);
parameter.set(new_value);
```

Using scales you can reduce that to:

```
let new_value = (&slider_scale, &parameter_scale).convert(slider.value());
parameter.set(new_value);
```

where `slider_scale` and `parameter_scale` are reusable components that look like this:

```
let slider_scale = LinearScale::new(slider.min(), slider.max());
let parameter_scale = LogarithmicScale::new(parameter.min(), parameter.max());
```
