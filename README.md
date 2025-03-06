# anyrun-qalculate

A simple [anyrun](https://github.com/anyrun-org/anyrun) plugin adding [Qalculate!](https://qalculate.github.io/index.html) support.
Start a query with `=` and have the power of Qalculate! at your fingertips.

NOTE: multiple statements are not supported.

## There is already an official Rink plugin. Why Qalculate! ?

Qalculate offers may [builtin functions](https://qalculate.github.io/manual/qalculate-functions.html) that Rink does not such as the ability to solve equations, or calculate derivatives.
For a full rundown of what Qalculate! has to offer, see the [features page](https://qalculate.github.io/features.html) or the [manual](https://qalculate.github.io/manual/index.html).

## Future plans

- [ ] Custom commands: Allow to specify custom commands to run before the main query.
      This gives users the option to define custom variables and functions.
- [ ] Color support: Translate the colors from the `qalc` cli output to pango markup.

Suggestions can be submitted by creating a new issue (prefer gitlab over github if possible).

## Installing

The plugin requires the `qalc` command from [libqalculate](https://github.com/Qalculate/libqalculate) to work.
To install `qalc` check your distro's repository.
The package will likely be called `libqalculate`.
Otherwise check the [Qalculate!](https://qalculate.github.io/index.html) site for installation instructions.

Clone the repo and run `just install`:

```shell
# clone and enter the repo
git clone https://gitlab.com/udragg/anyrun-qalculate.git && cd anyrun-qalculate

# build the plugin and copy it to ~/.config/anyrun/plugins/
# if just is not installed run the commands in the install recipe manually (or install just)
just install
```

Lastly add `"libqalculate.so"` to your plugins list in the main anyrun `config.toml`.
