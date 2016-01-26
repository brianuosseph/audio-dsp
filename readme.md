# rasp
[![Build Status](https://travis-ci.org/brianuosseph/rasp.svg?branch=master)](https://travis-ci.org/brianuosseph/rasp)
[![Coverage Status](https://coveralls.io/repos/brianuosseph/rasp/badge.svg?branch=master&service=github)](https://coveralls.io/github/brianuosseph/rasp?branch=master)

An audio signal processing library in Rust.

The design, and general usage, of this library is based on the [Synthesis Toolkit](https://ccrma.stanford.edu/software/stk/index.html) which is hosted by CCRMA and written in C++.


## Usage

All objects intended to process samples of an audio signal implement the `Processor` trait. Samples are passed into the object using `process()` and returns an output sample.


### Future Additions

Features and components I'd like to add in the future.

- Add general `EnvelopeDetector`?
  - Must warn in documentation that input must be absolute values
  - An example of when this is used is for gain changing in compressors
    - This is where the attack and release of the gain comes from
- `AllpassDelay`, an all-pass interpolating delay-line (see `stk::DelayA`)
- FFI
- More filters (Based on DSPFilters by vinniefalco, all optional)
  - `filter::butterworth`
  - `filter::chebyshev1`
  - `filter::chebyshev2`
  - `filter::elliptic`
  - `filter::bessel`
  - `filter::legendre`
- Common effects? (All optional, or just as examples)
  - CombFilter
  - Chorus
  - Flanger
  - Phaser
  - PitchShifter
  - Echo
  - Simple reverb?
- `mod envelope`
  - `Adsr`
  - `Ahdsr`
  - Others?
- Formant filter
- `mod generator` (see `stk::Generator`)
- Pluck-string model (see `stk::Twang`)
