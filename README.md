# shuffle

This is an awesome tool solving a problem I never had. It's a simple command line tool for shuffling. It shuffles the arguments given to it.

## Usage

  $ shuffle [options] [values ...]

Where options can be one of:
-h: Print usage and help.
-q (--quote-style): set quote style. Can be one of the following:
  * none: No arguments get surrounded by quotes
  * all: All arguments get surrounded by quotes
  * spaces: Only arguments containing whitespace get surrounded by quotes (default)

### Examples

  $ shuffle foo bar baz
  baz foo bar

  $ shuffle foo bar "baz rust"
  bar "baz rust" foo

  $ shuffle -q all foo bar "baz rust"
  "baz rust" "foo" "bar"

  $ shuffle -q none foo bar "baz rust"
  baz rust bar foo

  $ shuffle -q spaces foo bar "baz rust"
  "baz rust" bar foo

Everything after a -- will be interpreted as values, not options. In case you want to shuffle something starting with -.

  $ shuffle -q spaces hello -- foo -p "-q lol"
  foo "-q lol" -p hello

