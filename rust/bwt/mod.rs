pub const BWT_MARKER: usize = 256;
pub const MAX_VEC_SIZE: usize = 500_000;

mod sais;
use sais::sais_u8;

#[allow(dead_code)]
fn counting_sort(
    content: &[usize],
    list_rotation: &mut [isize],
    n_col: isize,
    get_char: GetCharFn,
    n_start: usize,
    n_end: usize,
) -> Vec<(usize, usize)> {
    let mut occurences: [usize; 257] = [0; 257];

    for rotation in n_start..n_end {
        occurences[get_char(content, list_rotation[rotation], n_col)] += 1
    }

    let mut begin_idx: [usize; 257] = [n_start; 257];
    let mut end_idx: [usize; 257] = [n_end; 257];
    for (idx, &occurence) in occurences[..256].iter().enumerate() {
        begin_idx[idx + 1] = begin_idx[idx] + occurence;
        end_idx[idx] = begin_idx[idx + 1];
    }

    let mut idx_rotation: usize = n_start;
    let mut j: usize = 0;
    let mut current_number: usize;
    while j < 256 {
        if idx_rotation == end_idx[j] {
            j += 1;
            idx_rotation = begin_idx[j];
            continue;
        }

        current_number = get_char(content, list_rotation[idx_rotation], n_col);

        if current_number == j {
            idx_rotation += 1;
            continue;
        }

        list_rotation.swap(idx_rotation, begin_idx[current_number]);
        begin_idx[current_number] += 1;

        if get_char(content, list_rotation[idx_rotation], n_col) == j {
            idx_rotation += 1;
        }
    }

    let mut equality_list: Vec<(usize, usize)> = Vec::new();
    for (idx, &occurence) in occurences.iter().enumerate() {
        if occurence > 1 {
            equality_list.push((end_idx[idx] - occurence, end_idx[idx]))
        }
    }

    equality_list
}

fn get_element_from_rotation(content: &[usize], rotation: isize, n_col: isize) -> usize {
    let idx = (n_col - rotation + content.len() as isize) as usize % content.len();
    content[idx]
}

fn get_element_from_suffix(content: &[usize], suffix: isize) -> usize {
    let rotation = (content.len() as isize - suffix) % (content.len() as isize);
    get_element_from_rotation(content, rotation, content.len() as isize - 1)
}

type GetCharFn = fn(&[usize], isize, isize) -> usize;

#[allow(dead_code)]
type StackElement = (isize, usize, usize);

#[allow(dead_code)]
fn radix_sort(text_content: &[usize], list_rotation: &mut [isize]) {
    let mut stack: Vec<StackElement> = Vec::new();

    let equalities = counting_sort(
        text_content,
        list_rotation,
        0,
        get_element_from_rotation,
        0,
        list_rotation.len(),
    );
    for &(n_deb, n_end) in equalities.iter() {
        stack.push((1, n_deb, n_end));
    }

    while let Some((n_col, n_deb, n_end)) = stack.pop() {
        let inner_equality = counting_sort(
            text_content,
            list_rotation,
            n_col,
            get_element_from_rotation,
            n_deb,
            n_end,
        );
        for &(n_start, n_final) in inner_equality.iter() {
            stack.push((n_col + 1, n_start, n_final));
        }
    }
}

fn min(el1: usize, el2: usize) -> usize {
    if el1 > el2 {
        el2
    } else {
        el1
    }
}

#[allow(dead_code)]
pub fn old_bwt_encode(mut text_content: Vec<usize>) -> Vec<usize> {
    let mut result: Vec<usize> = Vec::with_capacity(text_content.len());

    while !text_content.is_empty() {
        let drain_range = ..min(text_content.len(), 500_000);
        let mut subtext: Vec<usize> = text_content.drain(drain_range).collect();
        subtext.push(BWT_MARKER);
        let rotation_range = 0..subtext.len() as isize;
        let mut list_rotation: Vec<isize> = (rotation_range).collect();

        radix_sort(&subtext, &mut list_rotation);

        let last_col = subtext.len() as isize - 1;
        for &rotation in list_rotation.iter() {
            result.push(get_element_from_rotation(&subtext, rotation, last_col));
        }
    }

    result
}

fn find_nth(
    text_content: &[usize],
    el: usize,
    occur: usize,
    last_found: &mut [Vec<usize>],
) -> usize {
    if last_found[256].is_empty() {
        for (idx, &text_char) in text_content.iter().enumerate() {
            last_found[text_char].push(idx);
        }
    }
    last_found[el][occur - 1]
}

fn find_rank(text_content: &[usize], idx: usize) -> usize {
    let mut begin: usize = 0;
    let mut end = idx;
    let mut m: usize;
    let el = text_content[end];

    while end > begin {
        m = (begin + end) / 2;
        if text_content[m] < el {
            begin = m + 1;
        } else if text_content[m] >= el {
            end = m;
        }
    }

    idx - end + 1
}

pub fn bwt_decode(mut text_content: Vec<usize>) -> Vec<u8> {
    let mut result: Vec<u8> = Vec::with_capacity(text_content.len());

    while !text_content.is_empty() {
        let drain_range = ..min(text_content.len(), 500_001);
        let last_column: Vec<usize> = text_content.drain(drain_range).collect();
        let mut first_column = last_column.clone();
        first_column.sort();

        let mut last_found: Vec<Vec<usize>> = Vec::with_capacity(257);
        for _ in 0..257 {
            last_found.push(Vec::new());
        }

        let mut row = find_nth(&last_column, BWT_MARKER, 1, &mut last_found);
        let mut el = first_column[row];
        let mut occur: usize;

        while el != BWT_MARKER {
            result.push(el as u8);
            occur = find_rank(&first_column, row);
            row = find_nth(&last_column, el, occur, &mut last_found);
            el = first_column[row];
        }
    }

    result
}

pub fn bwt_encode(mut text_content: Vec<usize>) -> Vec<usize> {
    let mut result: Vec<usize> = Vec::with_capacity(text_content.len());

    while !text_content.is_empty() {
        let max_idx = min(MAX_VEC_SIZE, text_content.len());
        let mut sub_text: Vec<usize> = text_content.drain(..max_idx).collect();
        sub_text.push(BWT_MARKER);
        let suffix_array = sais_u8(&mut sub_text, 257);

        for &suffix in suffix_array[1..].iter() {
            result.push(get_element_from_suffix(&sub_text, suffix));
        }
    }

    result
}

#[cfg(test)]
mod test {
    use super::super::content;
    use super::{bwt_decode, bwt_encode, old_bwt_encode, sais_u8};

    #[ignore = "broken"]
    #[test]
    fn test_sais_u8() {
        let config = content::Config {
            file_path: String::from("pg5097.txt"),
        };
        let mut text_content: Vec<usize> = content::read_content(config);
        let list_suffix = sais_u8(&mut text_content, 257);
        for idx in 0..list_suffix.len() - 2 {
            let suffix = list_suffix[idx] as usize;
            let next_suffix = list_suffix[idx + 1] as usize;

            if text_content[suffix] > text_content[next_suffix] {
                panic!();
            }
        }
    }

    #[test]
    fn test_new_bwt() {
        let text = String::from("mmiissiissiippii");
        let text_content_u8 = text.as_bytes();
        let mut text_content = Vec::new();

        for &el in text_content_u8.iter() {
            text_content.push(el as usize);
        }

        let res1 = old_bwt_encode(text_content.clone());
        let res2 = bwt_encode(text_content);

        assert_eq!(res1, res2);
    }

    #[test]
    fn test_new_bwt_decode() {
        let text = String::from("mmiissiissiippii");
        let text_content_u8 = text.as_bytes();
        let mut text_content = Vec::new();

        for &el in text_content_u8.iter() {
            text_content.push(el as usize);
        }

        let code_bwt = old_bwt_encode(text_content.clone());
        let decode = bwt_decode(code_bwt);

        assert_eq!(decode, text_content_u8);
    }
}
