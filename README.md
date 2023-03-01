/// Create a version 1 UUID# rUUID

I couldn't find a tool that generates uuid's of various formats _conveniently_ and programatically over the cli and rUUID was born. Necessity is the mother of "I can't find it, I'll make it".

This crate only depends on `uuid` and `clap` crates.

# Usage

There are subcommands for each type of supported uuid's. Any uuid formats supported by `uuid` are supported by `rUUID` as well.

For example, to generate a UUID_V1:

```bash
ruuid v1
```

To generate a version 1 UUID with the braced format:

```bash
ruuid v1 -b
```

You can pass a number to tell `ruuid` how many UUIDs you want generated:

```bash
ruuid v1 3
```

The default number of UUIDs generated is 1.

The following UUIDs are supported:

- version 1
- version 3
- version 4
- version 5
- version 6
- version 7
- version 8

Version 1, 3, 5, 6 and 8 need some random bytes to be generated, namely 6 pieces of `u8` (except for version 8 which needs 16 `u8`s).
These are generated with the help of the rand crate. Honestly I have no ideas whether is sufficient, if it's not, let me know.

Version 3 and 5 implicitly use the DNS namespace. I don't know whether it would be preferable to have that configurable. It's static for know.
If you'd like that changed, let me know.
