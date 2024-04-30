const ZRLE_ONE: usize = 257;
const ZRLE_TWO: usize = 258;

fn from_int_to_zrle(length: usize) -> Vec<usize> {
    let mut result: Vec<usize> = Vec::new();
    let mut q_n = length;
    let mut q_np1;

    while q_n > 0 {
        q_np1 = (q_n + 2 - 1) / 2 - 1;
        if q_n - 2 * q_np1 == 1 {
            result.push(ZRLE_ONE);
        } else if q_n - 2 * q_np1 == 2{
            result.push(ZRLE_TWO);
        }
        q_n = q_np1;
    }

    result.reverse();
    result
}

fn from_zrle_to_int(zrle: &[usize]) -> usize {
    let mut result: usize = 0;

    for (k, &el) in zrle.iter().rev().enumerate() {
        if el == ZRLE_ONE {
            result += 2usize.pow(k as u32);
        } else if el == ZRLE_TWO {
            result += 2 * 2usize.pow(k as u32);
        }
    }

    result
}

pub fn zrle_encode(text_content: Vec<usize>) -> Vec<usize> {
    let mut result: Vec<usize> = Vec::new();

    let mut idx_deb = 0;
    let mut idx_end;
    while idx_deb < text_content.len() {
        if text_content[idx_deb] != 0 {
            result.push(text_content[idx_deb]);
            idx_deb += 1;
        } else if text_content[idx_deb] == 0 {
            idx_end = idx_deb + 1;
            while idx_end < text_content.len() && text_content[idx_end] == 0 {
                idx_end += 1;
            }
            result.append(&mut from_int_to_zrle(idx_end - idx_deb));
            idx_deb = idx_end
        }
    }

    result
}

pub fn zrle_decode(text_content: Vec<usize>) -> Vec<usize> {
    let mut result: Vec<usize> = Vec::with_capacity(text_content.len());

    let mut idx_deb = 0;
    let mut idx_end;

    while idx_deb < text_content.len() {
        if (text_content[idx_deb] != ZRLE_ONE) & (text_content[idx_deb] != ZRLE_TWO) {
            result.push(text_content[idx_deb]);
            idx_deb += 1;
        } else {
            idx_end = idx_deb + 1;
            while idx_end < text_content.len() && (text_content[idx_end] == ZRLE_ONE || text_content[idx_end] == ZRLE_TWO) {
                idx_end += 1;
            }
            let length = from_zrle_to_int(&text_content[idx_deb..idx_end]);
            let mut zero_vec = vec![0; length];
            result.append(&mut zero_vec); 
            idx_deb = idx_end;
        }
    }

    result
}



#[cfg(test)]
mod test {
    use crate::z_rle::{ZRLE_ONE, ZRLE_TWO};

    use super::{from_int_to_zrle, from_zrle_to_int, zrle_decode, zrle_encode};

    #[test]
    fn test_conversion() {
        let length = 10;
        let res = from_int_to_zrle(length);
        assert_eq!(res, vec![ZRLE_ONE, ZRLE_TWO, ZRLE_TWO]);
    }

    #[test]
    fn test_deconversion() {
        let zrle = vec![ZRLE_ONE, ZRLE_TWO, ZRLE_TWO];
        let res = from_zrle_to_int(&zrle);
        assert_eq!(res, 10);
    }

    #[test]
    fn test_both() {
        let num = 123;
        assert_eq!(num, from_zrle_to_int(&from_int_to_zrle(num)));
    }

    #[test]
    fn test_encode() {
        let content = vec![97, 0, 0, 0, 98, 0, 0, 99, 0, 2, 13, 256];
        let code = zrle_encode(content.clone());
        println!("{:?}", code);
        let decode = zrle_decode(code);
        assert_eq!(content, decode);
    }
    
    #[test]
    fn test_zrle() {
        let text_content = vec![97, 0, 0, 0, 98, 0, 0, 99, 0, 2, 13, 256];
        let code = zrle_encode(text_content.clone());
        let decode = zrle_decode(code);
        assert_eq!(text_content, decode); 
    }
}