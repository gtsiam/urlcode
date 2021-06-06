# Url coding utility

This is a convinience tool for managing urls from the command line, inspired by the base\* family of
tools from `coreutils`.

Examples:

```sh
$ echo -ne "hello there/ asd23@#%23" | urlcode
hello%20there%2F%20asd23%40%23%2523
$ echo -ne "hello%20there%2F%20asd23%40%23%2523" | urlcode -d
hello there/ asd23@#%23
```
