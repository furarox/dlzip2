import pytest
from dlzip2.compression_utils import compress, decompress


def test_compress_argument():
    with pytest.raises(TypeError):
        compress({})
    with pytest.raises(ValueError):
        compress([1111])


def test_decompress_argument():
    with pytest.raises(TypeError):
        decompress("blabla")
    with pytest.raises(ValueError):
        decompress(b'blabla', "test")


def test_compress_decompress():
    content = ("Some useless text that should be compressed and decompressed,"
               " hopefully, at the end, we should have content")

    content_compressed = compress(content)
    content_decompressed = decompress(content_compressed, return_type='str')

    assert content_decompressed == content
