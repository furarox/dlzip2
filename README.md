# DLZIP2

A useless compression algorithm, written in rust and packaged in a python module, which also install a cli

## Installation

To install the python module dlzip2 and the command line dlzip2, please follow those steps.

```
git clone https://github.com/furarox/dlzip2.git your_directory
cd your_directory
pip install .
```

The python module dlzip2 comes with two functions in scope : compress and decompress which respectively compress a bytes collection (or str, or list of int), and decompress the output of compress.

To avoid to install the command line, and just install the python package, please suppres this lines from pyproject.toml file  :
```
[projects.scripts]
dlzip2 = 'dlzip2._cli'
```

### Requirements

Normally, Python (and pip) with version >=3.8 and the Rust toolchain >=1.56 are sufficient to build this module

## Usage

```
usage: dlzip2 [-h] [-d] [-o OUTPUT_PATH] filename

A command line interface to compress file (or directory)

positional arguments:
  filename              file to compress

options:
  -h, --help            show this help message and exit
  -d, --decompress      flag for decompression
  -o OUTPUT_PATH, --output OUTPUT_PATH
                        path to store the compressed file

This project is still under developement
```

## Acknowledgements

First of all, the algorithm used for compression is a slower and less performant version of bzip2.
This algorithm comes from an exercice of one of my teacher, which initial are D.L., thus the name of package.

This project was my first project to learn Rust (god damn linked list in mtf.rs hit hard) and, but also to enhance my skills in python packaging.
I also discovered pyo3, and the possibility to connect python and rust for the best of the two languages.
I do not recommend to use this package in production (because it's still really slow), but it can be a good way to discover 
rust, pyo3 and python packaging
