# Changelog

## 0.4.0
 - Large change (thanks @sempervictus) to allow querying of message content by both numerical indexer and dot-notation string indexers
    - Note that the string indexers will be replaced with a normal function call in a future release.

## 0.3.0
 - Changes from @sempervictus to expose internal values again

## 0.2.0
- Re-write to avoid excessive string cloning by operating on slices of the source HL7

## 0.1.0
- Initial string.clone() heavy library, nothing to see here...