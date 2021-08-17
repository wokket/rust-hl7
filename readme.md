## An experimental HL7 library ##

[![CI Ubunutu](https://github.com/wokket/rust-hl7/actions/workflows/ci.yml/badge.svg)](https://github.com/wokket/rust-hl7/actions/workflows/ci.yml)
[![Crates IO](https://img.shields.io/crates/v/rust-hl7.svg)](https://crates.io/crates/rust-hl7)

Totally kind of like production ready!

This second cut provides consistent structure down to the sub-sub-field, efficient accessors to shared string reference data, with standardized implementations of common functionality.

Interpreting these facets (type conversion, determining which fields they represent etc) is a future problem... there is **no plan whatsoever** for message conformance checks or anything of that nature.

This library is trying to provide the _tooling_ you need to build robust HL7 based systems, without dictating _how_ you go about it.  There's no one-size-fits-all here, so we try to provide a box of separate tools rather than a full framework.

### Intended Features and Design Notes:
- [x] Initially use hl7 default separator chars
- [x] Use separator chars from the message
- [X] Add support for sub-field (component/subcomponent) items
    - [ ] Field repeats (via `~`) are currently missing ([#26](https://github.com/wokket/rust-hl7/issues/26))
- [X] Initially, avoid any per-segment knowledge, requirement to read the spec too much etc.
    - Implementing all the segments, across all the hl7 versions, version-specific parsing etc is tooooo much while we're getting started.
- [-] Add support for [HL7 escape sequences](https://www.lyniate.com/knowledge-hub/hl7-escape-sequences/) ([#22](https://github.com/wokket/rust-hl7/issues/22))
    - [x] Decoding of the most common escape sequences including `\E\`, `\R\`, `\S\` & `\T\`
    - [x] Correctly passes through `\H\`, `\N\` and custom `\Z..\` sequences unchanged
    - [X] Decodes `\X..\` sequences for hex-encoded chars
    - [ ] Support for various unicode sequences (`\C..\`, `\M..\`).  These are lower priority as [HL7 Australia considers them deprecated](https://confluence.hl7australia.com/display/OO/3+Datatypes#id-3Datatypes-3.1.1.6EscapesequencessupportingmultiplecharactersetsforFT,ST,andTXdatatypes)
- [ ] Add tighter MSH as an exception to the above
- [ ] The above allows us to parse everything as strings, and provide helper methods for type conversions as required.
- [x] Parse a message using a `TryFrom<&str>` impl rather than a dedicated parser
- [x] Index into messages using HL7 string index notation and binary methods
    - [x] Index into sub-fields using HL7 string index notation and binary methods
    - [X] Index into the segment enum using HL7 string index notation and binary methods
- [ ] Implement buffer-copy-free generic indexing into MSH