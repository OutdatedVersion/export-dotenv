# export dotenv

I've caught myself prefixing a `local.env`'s lines with `export ` more
times than I'd care to admit ... which this prevents!

## Usage

Put binary on your path from

- Releases
- `git clone` and `cargo build --release`

```console
$ echo 'HELLO=friend' > example.env
$ export-dotenv example.env | source /dev/stdin
```
