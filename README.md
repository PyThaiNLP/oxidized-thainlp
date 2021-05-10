
# BETA Release

```bash
pip install pythainlp-rust-modules
```


# Feature

- newmm word tokenization with default dict, ultra fast speed.
- support custom dict.
------------------------------------------------------------------------------------------------------------------

------------------------
# How To Build 

## Requirements

- Rust 2018 Edition [Download here](https://www.rust-lang.org/tools/install)
- Python 3.6+
- PyO3 - already included in Cargo.toml
- *** For Linux *** sudo apt install python3-dev python-dev
- [Maturin](https://github.com/PyO3/maturin)
Linux

At pythainlp root directory

```bash

maturin build --release -i python --manylinux off  
# Or 
maturin build --release -i python

```

Windows Powershell
```shell
path\\to\\maturin.exe build --release -i python

```

MacOS
```zsh
maturin build --release -i python
```


This should generate a wheel file which can be installed by pip

-------------------
# How To Use Built module

```python
from oxidized_thainlp import segment,load_dict


```

Just that!

--------------------------------------------------------------------------------------------------------------------------------------------------------
# TODO

- Distribute module on pip or whatever, I really do not know much about Python env.

# oxidized-thainlp
