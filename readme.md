## An experimental HL7 library ##

[![CI Ubunutu](https://github.com/wokket/rust-hl7/actions/workflows/ci.yml/badge.svg)](https://github.com/wokket/rust-hl7/actions/workflows/ci.yml)
[![Crates IO](https://img.shields.io/crates/v/rust-hl7.svg)](https://crates.io/crates/rust-hl7)

Totally kind of like production ready!

The first cut was intended to parse from a multiline text blob into a tree of string slices, representing all the different facets of info.
This second cut provides consistent structure down to the sub-sub-field, efficient accessors to shared string reference data, with standardized implementations of common functionality.

Interpreting these facets (type conversion, determining which fields they represent etc) is a future problem.

### Intended Features and Design Notes:
- [x] Initially use hl7 default separator chars
- [x] Use separator chars from the message
- [X] Add support for sub-field (repeat/component/subcomponent) items
- [X] Initially, avoid any per-segment knowledge, requirement to read the spec too much etc.
    - Implementing all the segments, across all the hl7 versions, version-specific parsing etc is tooooo much while we're getting started.
- [ ] Add Decoding/Encoding of special chars
- [ ] Add tighter MSH as an exception to the above
- [ ] The above allows us to parse everything as strings, and provide helper methods for type conversions as required.
- [ ] Parse using a from_str() impl rather than a dedicated parser (idiomatic but no lifetimes)
- [x] Index into messages using HL7 string index notation and binary methods
    - [x] Index into sub-fields using HL7 string index notation and binary methods
    - [X] Index into the segment enum using HL7 string index notation and binary methods
- [ ] Implement buffer-copy-free generic indexing into MSH