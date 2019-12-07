## An experimental HL7 library ##

Totally nothing like production ready!

This first cut is intended to parse from a multiline text blob into a tree of strings, representing all the different facets of info.

Interpreting these facets (type conversion, determining which fields they represent etc) is a future problem.

### Intended Features and Design Notes:
- [x] Initially use hl7 default separator chars
- [x] Use separator chars from the message
- [ ] Parse using a from_str() impl rather than a dedicated parser (idiomatic)
- [ ] Add support for sub-field (repeat/component/subcomponent) items
- [ ] Avoid any lifetime complexities, even if it costs perf initially
- [ ] Initially, avoid any per-segment knowledge, requirement to read the spec too much etc.
    - Implementing all the segments, across all the hl7 versions, version-specific parsing etc is tooooo much while we're getting started.
- [ ] Add tighter MSH as an exception to the above
- [ ] The above allows us to parse everything as strings, and provide helper methods for type conversions as required.
- [ ] Add Decoding/Encoding of special chars