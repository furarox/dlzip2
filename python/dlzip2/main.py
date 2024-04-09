import argparse


def dlzip2_cli():
    """Command line function for dlzip2 package"""

    parser = argparse.ArgumentParser(
        prog="dlzip2",
        description="A command line interface to compress file (or directory)",
        epilog="This project is still under developement"
    )

    parser.add_argument('filename', help="file to compress")
    parser.add_argument('-d', '--decompress', dest="decompress",
                        action='store_true', help="flag for decompression")
    parser.add_argument('-o', '--output',
                        dest="output_path",
                        help="path to store the compressed file")

    args = parser.parse_args()

    print(args.filename, args.decompress, args.output_path)


if __name__ == '__main__':
    dlzip2_cli()
