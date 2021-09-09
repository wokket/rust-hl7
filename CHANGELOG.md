# Changelog

## 0.6.0
- Breaking Change ([#25](https://github.com/wokket/rust-hl7/issues/25)): Moved the core structs to the top-level module to avoid the noisy using statements.

## 0.5.0
 - Add `query` functions to replace the string based `Index` impls in the version version.  These are functionally identical to the string `Index` implementations, but avoid some lifetime issues (returning `&&str`) and have visible documentation.
 - Add `EscapeSequence` struct to support decoding [escape sequences](https://www.lyniate.com/knowledge-hub/hl7-escape-sequences/) back to their original values.

## 0.4.0
 - Large change (thanks @sempervictus) to allow querying of message content by both numerical indexer and dot-notation string indexers
    - Note that the string indexers will be replaced with a normal function call in a future release.

## 0.3.0
 - Extensive work by @sempervictus to expose the segments/fields as collections (which I hadn't got back to after the re-write to slices.)

## 0.2.0
-  Re-Write to not expose cloned/copied vecs of vecs everywhere.  We have all the data in a single string slice to begin with so lets return slices from that.

## 0.1.0
- Initial string.clone() heavy library, nothing to see here...
