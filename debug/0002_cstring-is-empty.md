commitid f1f4779b50d7aedb7bd4f4b1ad385086ee1464ae

The child_main functions seems to receive an empty `root_directory` string.

### Notes

-   The `child_main_impl` receives `args=NULL`.

-   It appears, that dereferencing a pointer to call a method does not create
    a copy.
    https://play.rust-lang.org/?version=stable&mode=debug&edition=2018&gist=3625a313e5fe5711b17261dcb2f96149

### Ideas

### Theories

-   I've already discovered before that Rust can't quite deal with `clone()` correctly,
    maybe this is another side effect?

### Actions

-   Don't try to call a method on a pointer.  This is a bit of a workaround, but
    the code is cleaner than before...
