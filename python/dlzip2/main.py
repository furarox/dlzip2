import argparse
import pathlib
import sys
import time
from dlzip2 import compress, decompress


def dlzip2_cli():
    """Command line function for dlzip2 package"""

    parser = argparse.ArgumentParser(
        prog="dlzip2",
        description="A command line interface to compress file",
        epilog="This project is still under developement"
    )

    parser.add_argument('filename', help="filepath to the file de/compress")
    parser.add_argument('-d', '--decompress', dest="decompress",
                        action='store_true', help="flag for decompression")
    parser.add_argument('-o', '--output',
                        dest="output_path",
                        help="path to store the de/compressed file, "
                             "if not specified, it will add/remove .dlz2 "
                             "extension from {filepath} and use it to store "
                             "the output")
    parser.add_argument('-v', '--verbose', dest='verbose',
                        action='store_true',
                        help="Output the characteristics of the compression "
                             "or decompression, time and de/compression ratio")
    parser.add_argument('-p', '--paste', dest='paste',
                        action='store_true',
                        help='Output the de/compression to the terminal to '
                             'pipe into other command. If no output path are '
                             'specified, it will not generate a default '
                             'output file')

    args = parser.parse_args()
    file_path = pathlib.Path(args.filename)

    if args.output_path is not None:
        output_path = pathlib.Path(args.output_path)
    elif not args.paste:
        output_path = file_path
    else:
        output_path = None

    if not file_path.is_file():
        print(f"{file_path} is not a path to an existing file")
        sys.exit(1)

    if args.decompress and file_path.suffix != '.dlz2':
        print(f"Cannot decompress {file_path} because it doesn't have the "
              f"correct extension, expected '.dlz2' extension")
        sys.exit(1)

    if output_path:
        if not args.decompress and output_path.suffix != '.dlz2':
            output_path = output_path.with_suffix(output_path.suffix + '.dlz2')

        if args.decompress and output_path.suffix == '.dlz2':
            output_path = output_path.with_suffix("")

        if output_path.is_file() or output_path.is_dir():
            print(f"{output_path} is already a path to an existing file or "
                  f"directory, operation aborted")
            sys.exit(1)

    with open(file_path, 'rb') as f:
        file_content = f.read()
        deb = time.perf_counter()
        if args.decompress:
            output = decompress(file_content, return_type='bytes')
        else:
            output = compress(file_content)
        time_elapsed = time.perf_counter() - deb

    if args.verbose:
        print(f"Time elapsed : {time_elapsed:.3f}s, compression/decompression "
              f"ratio : {len(output) / len(file_content) * 100:.2f}%")

    if args.paste:
        print(output)

    if output_path:
        with open(output_path, "wb") as f:
            f.write(output)

    sys.exit(0)
