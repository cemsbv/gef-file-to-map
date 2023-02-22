# gef-file-to-map

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

>>> from gef_file_to_map import parse
>>> gef_to_map(open('./tests/test.gef').read())
```
