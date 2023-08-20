# K-lighter

K-lighter is a syntax highlighter for K language.

## Installation

Use cargo to install this package:

```bash
cargo install k-lighter
```

You can also build this package from source. Clone this repo using the following command:

```bash
git clone https://github.com/ondrachwiedziuk/k-lighter.git
```

Then enter main directory and run:

```bash
cargo install --path .
```

To have better performance, you can write alias in your .bashrc file:

```bash
alias k-lighter="$HOME/.cargo/bin/k-lighter"
```

## Usage

To use k-lighter, write:

```bash
k-lighter [OPTIONS] <INPUT> <OUTPUT>
```

### Arguments
| Name | Description |
|------|-------------|
|`INPUT` | Path to your K script.|
|`OUTPUT` |Path to HTML file where highlighted code will be located.|

### Options
| Short | Long        | Description                          |
|-------|-------------|--------------------------------------|
|`-s`   | `--style`   | Style which is used [default: basic] |
| `-h`  | `--help`    | Print help                           |
| `-V`  | `--version` | Print version                        |

### Example

To highlight file `foo.k`, write:
```bash
k-lighter foo.k foo.html
```

### Custom styles
You can also customize styles. Just append your pattern to the end of `$HOME/.cargo/k-lighter.ini` file.
Patterns have the following structure:
```ini
[NAME]
numbers = HEX_COLOR
vars = HEX_COLOR
verbs = HEX_COLOR
adverbs = HEX_COLOR
reserved = HEX_COLOR
pars_0 = HEX_COLOR
pars_1 = HEX_COLOR
pars_2 = HEX_COLOR
pars_3 = HEX_COLOR
comments = HEX_COLOR
string = HEX_COLOR
background = HEX_COLOR
```

Default style is `basic`:

```ini
[basic]
numbers = ffffff
vars = ffff00
verbs = 00ffff
adverbs = ff00ff
reserved = ff0000
pars_0 = 006600
pars_1 = 009900
pars_2 = 00cc00
pars_3 = 00ff00
comments = 0000ff
string = 508050
background = 000000
```
