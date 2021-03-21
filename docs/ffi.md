# FFI

I'm not sure if I want good ffi.

The main problem is it makes things like gc, and (esp) debuging a pain.

I might go the Go route of making it a pain, and disowning it.

The need for FFI is

1. Call existing code
2. Call fast code

An approach would be "officialy sacntioned" fast code embeded in the language.
But this is controlling and I dont like it

## Things you want to be fast, in a slow language

- Regex
- Sqlite
- Hashmaps/HashSets