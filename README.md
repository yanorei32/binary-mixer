# Binary Mixer

![GitHub](https://img.shields.io/github/license/yanorei32/binary-mixer)

A simple binary mix utility.
This created for decode a ROM of 8-bit EPROM x2 used as 16-bit EPROM.

![image](https://user-images.githubusercontent.com/11992915/231426589-7beeb5bd-6bda-4486-8f01-0d73654dd917.png)

## Installation

```bash
cargo install --git https://github.com/yanorei32/binary-mixer
```

## How to

### Mix mode

```bash
$ binary-mixer mix <(echo -n "Hlo") <(echo -n "el!") out0
$ xxd out0
00000000: 4865 6c6c 6f21                           Hello!
$ 
```

### Split mode

```bash
$ binary-mixer split <(echo -n "Hello!") out0 out1
$ xxd out0
00000000: 486c 6f                                  Hlo
$ xxd out1
00000000: 656c 21                                  el!
$ 
```
