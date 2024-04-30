use pyo3::prelude::*;

mod bwt;
pub mod content;
mod huffman;
mod mtf;
mod z_rle;

#[pyfunction]
fn _compress(text_content: Vec<usize>) -> PyResult<Vec<u8>> {
    let code = bwt::bwt_encode(text_content);
    let code = mtf::mtf_encode(code);
    let code = z_rle::zrle_encode(code);
    Ok(huffman::huffman_encode(code))
}

#[pyfunction]
fn _decompress(text_content: Vec<u8>) -> PyResult<Vec<u8>> {
    let decode = huffman::huffman_decode(text_content);
    let decode = z_rle::zrle_decode(decode);
    let decode = mtf::mtf_decode(decode);
    Ok(bwt::bwt_decode(decode))
}

#[pymodule]
fn _dlzip2(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(_compress, m)?)?;
    m.add_function(wrap_pyfunction!(_decompress, m)?)?;
    Ok(())
}

#[cfg(test)]
mod test {
    use crate::huffman;

    use super::{bwt, mtf, z_rle};

    #[test]
    fn test_bwt_mtf_zrle_chain() {
        let text_content = vec![15, 15, 15, 15, 16, 16, 231, 231, 192, 255];

        let mut code = bwt::bwt_encode(text_content.clone());
        code = mtf::mtf_encode(code);
        code = z_rle::zrle_encode(code);

        println!("{:?}", code);

        let mut decode = z_rle::zrle_decode(code);
        decode = mtf::mtf_decode(decode);
        let _decode = bwt::bwt_decode(decode);
    }

    #[test]
    fn test_all() {
        let text_content = vec![15, 15, 15, 15, 16, 16, 231, 231, 192, 255];
        let text_len = text_content.len();

        let mut code = bwt::bwt_encode(text_content.clone());
        code = mtf::mtf_encode(code);
        code = z_rle::zrle_encode(code);
        let code = huffman::huffman_encode(code);

        println!(
            "Compression ratio: {} %",
            (code.len() as f64 / text_len as f64) * 100 as f64
        );

        let mut decode = huffman::huffman_decode(code);
        decode = z_rle::zrle_decode(decode);
        decode = mtf::mtf_decode(decode);
        let _decode = bwt::bwt_decode(decode);
    }
}
