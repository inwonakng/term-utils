# Term utils

Simple terminal utilities I got tired of scripting.

## Installation

### Building from source:

Mac/Linux:

```bash
git clone https://github.com/inwonakng/term-utils ~/Downloads/term-utils
bash scripts/install.sh
```

This creates `~/.term-utils` and adds it to your PATH.

## Functionalities

### Zip current directory contents separately

`zip-dir`

### Correctly pad numerical directory names

`fmt-num <PATTERN>`

Where pattern should include one `\d+` for the number in target.

### Fix unicode filenames

`fmt-unicode`

Most likely not a problem, but mac makes korean weird. Run this to fix them all to NFC format.

### Group directories

`grp-dir <PATTERN>`

Where pattern is a regex pattern for deciding the groups.

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
