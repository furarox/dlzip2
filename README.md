# DLZIP2

A useless compression algorithm, written in rust and packaged in a python module.

## Installation

To install the python module dlzip2 and the command line dlzip2, please follow those steps.
It is reccomanded to install in a dedicated environnements to test before.

```
git clone https://github.com/furarox/dlzip2.git your_directory
cd your_directory
pip install .
```
To avoid to install the command line, and just install the python package, please suppres this lines from pyproject.toml file before pip install . :
```
[projects.scripts]
dlzip2 = 'dlzip2._cli'
```

### Requirements

Normally, Python (and pip) with version >=3.8 and the Rust toolchain >=1.56 are sufficient to build this module

## Usage

### CLI dlzip2

```
usage: dlzip2 [-h] [-d] [-o OUTPUT_PATH] [-v] [-p] filename

A command line interface to compress file

positional arguments:
  filename              filepath to the file de/compress

options:
  -h, --help            show this help message and exit
  -d, --decompress      flag for decompression
  -o OUTPUT_PATH, --output OUTPUT_PATH
                        path to store the de/compressed file, if not specified, it will add/remove .dlz2 extension from {filepath} and use it to store the
                        output
  -v, --verbose         Output the characteristics of the compression or decompression, time and de/compression ratio
  -p, --paste           Output the de/compression to the terminal to pipe into other command. If no output path are specified, it will not generate a
                        default output file

This project is still under developement
```

### Python module dlzip2

The python module dlzip2 installed procures two functions : compress and decompress. 
Here is an example of how to use them :
```
>>> import dlzip2
>>> long_text = "Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum."
>>> len(long_text)
445
>>> text_compressed = dlzip2.compress(long_text)
>>> len(text_compressed)
358
>>> assert dlzip2.decompress(text_compressed, return_type='str') == long_text
```

For more information, you can check those functions documentations.

(Note that compression is not really efficient on small text.
Indeed, compression aims to recognize common pattern and remplace them by fewer characters.
But there are less common patterns in small text rather than in big texts)

## Acknowledgements

First of all, the algorithm used for compression is a slower and less performant version of bzip2.
This algorithm comes from an exercice of one of my teacher, which initial are D.L., thus the name of package.

This project was my first project to learn Rust (god damn linked list in mtf.rs hit hard) and, but also to enhance my skills in python packaging.
I also discovered pyo3, and the possibility to connect python and rust for the best of the two languages.
I do not recommend to use this package in production (because it's still really slow), but it can be a good way to discover 
rust, pyo3 and python packaging
