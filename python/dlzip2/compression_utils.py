from dlzip2._dlzip2 import _compress, _decompress


def compress(content: list[int] | bytes | str) -> bytes:
    """Compress content using dlzip2 algorithm

    Parameters
    -----------
    content : list[int] | bytes | str
        the element to be compressed. It will commpress the utf8
        representation of the element, so in case of list[int], all
        element should be in [0, 256[

    Returns
    ---------
    bytes
        bytes representation of the compressed element
    """

    if isinstance(content, str):
        content = bytes(content, "utf8")
        return bytes(_compress(content))
    elif isinstance(content, bytes):
        return _compress(content)
    elif isinstance(content, list):
        if min(content) < 0 or max(content) > 255:
            raise ValueError(
                "list element should be between 0 and 255 (included),"
                "for utf8 representation")
        return bytes(_compress(bytes(content)))
    else:
        raise TypeError(
            f"content should be either list[int] | bytes | str, not"
            f" {type(content)}")


def decompress(content: bytes, return_type='bytes') -> bytes | str:
    """Decompress the bytes of data using the inverse transformation of
    compress

    Parameters
    -----------
    content : bytes
        bytes of data to decrompress
    return_type : str = 'bytes'
        {'str', 'bytes'} type of the return element
    """
    #
    if not isinstance(content, bytes):
        raise TypeError(f"content should be bytes, not {type(content)}")

    if return_type == 'bytes':
        return bytes(_decompress(content))
    elif return_type == 'str':
        return bytes(_decompress(content)).decode()
    else:
        raise ValueError("return_type should be in {'bytes', 'str'}")