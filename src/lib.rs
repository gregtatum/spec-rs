// Enable benchmarks, which are unstable.
#![feature(test)]

extern crate icu;
extern crate icu_provider_fs;
mod atomics;
mod calendar;
mod floats;
mod icu_test;
mod pin;
mod pointers;
mod smallstr_test;
