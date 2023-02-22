# gef-file-to-map

[![PyPI version](https://badge.fury.io/py/gef-file-to-map.svg)](https://pypi.org/project/gef-file-to-map/0.1.0/)

Utility library for parsing of .gef files, written for [pygef](https://github.com/cemsbv/pygef).

### Run locally

```sh
# Install maturin inside a Python environment
python3 -m venv .env
source .env/bin/activate
pip install maturin

# Create a Python package from the Rust code
maturin develop

# Open a GEF file locally
python

>>> from gef_file_to_map import gef_to_map
>>> gef_to_map(open('./tests/test.gef').read())
```
