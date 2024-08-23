# Term utils

Simple terminal utilities I got tired of scripting.

## Installation

### Building from source:

Mac/Linux (need `Cargo`):

```bash
git clone https://github.com/inwonakng/term-utils ~/Downloads/term-utils
bash scripts/install.sh
```

This creates `~/.term-utils` and adds it to your PATH.

## Functionalities

### Zip current directory contents separately

`zip-dir -p <PATTERN>`

Where the optional `PATTERN` should be a filter to pick which directories to zip. Default is `.*`.
Only zips directories and skips loose files.

### Correctly pad numerical directory names

`fmt-num <PATTERN> -z <NUM>`

Where pattern should include one `\d+` for the number in target. The optional argument `NUM` will be the number of digits to pad to. If any number is already longer than `NUM`, it will override `NUM`.

For example, if you have directories named:

```
test-1
test-2
test-100
```

Running `fmt-num "test-\d+"` will rename them to:

```
test-001
test-002
test-100
```

While running `fmt-num "test-\d+" -z 5` will rename them to:

```
test-00001
test-00002
test-00100
```

### Fix unicode filenames

`fmt-unicode`

Most likely not a problem, but mac makes Korean weird. Run this to fix them all to NFC format.

### Group directories

`grp-dir <PATTERN>`

Where `PATTERN` is a regex pattern for deciding the groups.

For example, if you have directories named:

```
- dummy-1 a
- dummy-1 b
- dummy-1 1
- dummy-2 a
- dummy-2 b
```

You can group them by `dummy-\d+` by running `group-dir dummy-\d+` to get:

```
- dummy-1
  - dummy-1 a
  - dummy-1 b
  - dummy-1 1
- dummy-2
  - dummy-2 a
  - dummy-2 b
```
