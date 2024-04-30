const SENTINEL: usize = 0;

/// Build the type map and count the characters to construct bucket
/// type_map is true for S- type and false for L- type
///
fn build_type_map_u8(text: &[usize], alphabet_size: usize) -> (Vec<bool>, Vec<usize>) {
    let mut t: Vec<bool> = vec![false; text.len()];
    let mut char_count: Vec<usize> = vec![0; alphabet_size];

    let mut i: usize = 0;
    let mut j: usize;

    while i < text.len() - 1 {
        char_count[text[i]] += 1;
        j = i + 1;
        // This case will always breaks before reaching the end bcs of SENTINEL
        while text[i] == text[j] {
            char_count[text[j]] += 1;
            j += 1;
        }

        if text[i] < text[j] {
            for k in i..j {
                t[k] = true;
            }
        } else {
            for k in i..j {
                t[k] = false;
            }
        }

        i = j;
    }

    t[text.len() - 1] = true;
    char_count[0] = 1;

    (t, char_count)
}

fn find_lms_character(type_map: &[bool]) -> Vec<usize> {
    let mut pa: Vec<usize> = Vec::new();

    let mut i: usize = 1;
    while i < type_map.len() {
        if type_map[i] && !type_map[i - 1] {
            pa.push(i);
        }
        i += 1;
    }

    pa
}

/// Build bucket representation with the head and the end
fn build_bucket(char_count: &[usize], alphabet_size: usize) -> (Vec<usize>, Vec<usize>) {
    let mut bucket_head: Vec<usize> = vec![0; alphabet_size];
    let mut bucket_tail: Vec<usize> = vec![1; alphabet_size];

    for (el, &count) in char_count[1..].iter().enumerate() {
        bucket_head[el + 1] = bucket_tail[el];
        bucket_tail[el + 1] = bucket_head[el + 1] + count;
    }

    (bucket_head, bucket_tail)
}

/// Make a guess of suffix array sort by placing LMS characters into their buckets
fn guess_lms_sort(text: &[usize], sa: &mut [isize], pa: &[usize], bucket_tail: &mut [usize]) {
    for &el in pa.iter() {
        bucket_tail[text[el]] -= 1;
        let idx = bucket_tail[text[el]];
        sa[idx] = el as isize;
    }
}

/// Induce sort all L- type element
fn induce_sort_l(text: &[usize], sa: &mut [isize], bucket_head: &mut [usize], type_map: &[bool]) {
    let mut idx = 0;
    while idx < sa.len() {
        if sa[idx] > 0 {
            let idx_el = sa[idx] as usize - 1;
            if !type_map[idx_el] {
                sa[bucket_head[text[idx_el]]] = sa[idx] - 1;
                bucket_head[text[idx_el]] += 1;
            }
        }

        idx += 1;
    }
}

fn induce_sort_s(text: &[usize], sa: &mut [isize], bucket_tail: &mut [usize], type_map: &[bool]) {
    let mut idx = sa.len() - 1;
    while idx > 0 {
        if sa[idx] > 0 {
            let idx_el = sa[idx] as usize - 1;
            if type_map[idx_el] {
                bucket_tail[text[idx_el]] -= 1;
                sa[bucket_tail[text[idx_el]]] = sa[idx] - 1;
            }
        }

        idx -= 1;
    }
}

fn is_lms_char(suffix: isize, type_map: &[bool]) -> bool {
    suffix > 0 && type_map[suffix as usize] && !type_map[suffix as usize - 1]
}

fn is_lms_substring_eq(
    text: &[usize],
    type_map: &[bool],
    mut suffix1: usize,
    mut suffix2: usize,
) -> bool {
    if suffix1 == text.len() - 1 {
        // the last lms substring is unique
        return false;
    }

    if (text[suffix1] != text[suffix2]) || (type_map[suffix1] != type_map[suffix2]) {
        return false;
    }

    suffix1 += 1;
    suffix2 += 1;

    while !is_lms_char(suffix1 as isize, type_map) && !is_lms_char(suffix2 as isize, type_map) {
        if (text[suffix1] != text[suffix2]) || (type_map[suffix1] != type_map[suffix2]) {
            return false;
        }
        suffix1 += 1;
        suffix2 += 1;
    }

    is_lms_char(suffix1 as isize, type_map) && is_lms_char(suffix2 as isize, type_map)
}

fn build_summary_string(
    text: &[usize],
    sa: &[isize],
    pa: &[usize],
    type_map: &[bool],
) -> (usize, Vec<usize>) {
    // First we scan sa to detect lms substring, when we detect one, we check if it is different from the previous
    // if not, we assign it the same name, otherwise we increase the name by 1

    let mut name_map: Vec<usize> = vec![0; text.len()];
    let mut last_lms: usize = text.len() - 1;
    let mut current_name: usize = 1;

    // Insert last suffix
    name_map[text.len() - 1] = current_name;

    for &suffix in sa[1..].iter() {
        if is_lms_char(suffix, type_map) {
            if is_lms_substring_eq(text, type_map, last_lms, suffix as usize) {
                name_map[suffix as usize] = current_name;
            } else {
                current_name += 1;
                last_lms = suffix as usize;
                name_map[suffix as usize] = current_name;
            }
        }
    }

    let mut s1: Vec<usize> = Vec::with_capacity(pa.len());
    for &lms_suffix in pa.iter() {
        s1.push(name_map[lms_suffix]);
    }

    (current_name, s1)
}

fn build_summary_sa(s1: &mut Vec<usize>, alphabet_size: usize) -> Vec<isize> {
    // If every character is unique directly compute SA1
    if alphabet_size == s1.len() {
        let mut sa1: Vec<isize> = vec![-1; s1.len() + 1];
        sa1[0] = s1.len() as isize;
        for (idx, &el) in s1.iter().enumerate() {
            sa1[el] = idx as isize;
        }
        sa1
    } else {
        sais_u8(s1, alphabet_size + 1)
    }
}

fn exact_lms_sort(
    text: &[usize],
    sa: &mut [isize],
    sa1: &[isize],
    pa: &[usize],
    bucket_tail: &mut [usize],
) {
    let mut idx = sa1.len() - 1;
    while idx > 0 {
        let lms_suffix = pa[sa1[idx] as usize];
        bucket_tail[text[lms_suffix]] -= 1;
        sa[bucket_tail[text[lms_suffix]]] = lms_suffix as isize;

        idx -= 1;
    }
}

pub fn sais_u8(text: &mut Vec<usize>, alphabet_size: usize) -> Vec<isize> {
    // First push the sentinel at the end of the text
    text.push(SENTINEL);

    // Then count characters and build type map
    let (type_map, char_count) = build_type_map_u8(text, alphabet_size);

    // Find all LMS characters
    let pa = find_lms_character(&type_map);

    // Build the bucket
    let (mut bucket_head, mut bucket_tail) = build_bucket(&char_count, alphabet_size);

    // Put each LMS character into their bucket
    let mut sa: Vec<isize> = vec![-1; text.len()];
    guess_lms_sort(text, &mut sa, &pa, &mut bucket_tail);

    // Reset bucket (at least tail)
    let (_, mut bucket_tail) = build_bucket(&char_count, alphabet_size);

    // Slot other suffix into the guess array
    induce_sort_l(text, &mut sa, &mut bucket_head, &type_map);
    induce_sort_s(text, &mut sa, &mut bucket_tail, &type_map);

    // Create shortened string s1
    let (name_count, mut s1) = build_summary_string(text, &sa, &pa, &type_map);

    // Create suffix array of s1 (recursive call can happen here)
    let sa1 = build_summary_sa(&mut s1, name_count);

    // Sort LMS character from sa1
    let mut sa: Vec<isize> = vec![-1; text.len()];
    let (mut bucket_head, mut bucket_tail) = build_bucket(&char_count, alphabet_size);
    exact_lms_sort(text, &mut sa, &sa1, &pa, &mut bucket_tail);

    // Sort the rest of the characters
    induce_sort_l(text, &mut sa, &mut bucket_head, &type_map);
    let (_, mut bucket_tail) = build_bucket(&char_count, alphabet_size);
    induce_sort_s(text, &mut sa, &mut bucket_tail, &type_map);

    // Remove the sentinel
    text.pop();
    sa
}

#[cfg(test)]
mod sais_test {

    use super::*;

    #[test]
    fn test_build_type_map_u8() {
        let text: Vec<usize> = vec![3, 1, 2, 2, 1, 7, 5, 0];
        let (type_map, char_count) = build_type_map_u8(&text, 257);

        assert_eq!(
            type_map,
            vec![false, true, false, false, true, false, false, true]
        );

        assert_eq!(char_count[0], 1);
        assert_eq!(char_count[1], 2);
        assert_eq!(char_count[2], 2);
        assert_eq!(char_count[3], 1);
        assert_eq!(char_count[4], 0);
    }

    #[test]
    fn test_build_bucket() {
        let text: Vec<usize> = vec![3, 1, 2, 2, 1, 7, 5, 0];
        let (_type_map, char_count) = build_type_map_u8(&text, 257);
        let (bucket_head, bucket_tail) = build_bucket(&char_count, 257);

        assert_eq!(bucket_head[0], 0);
        assert_eq!(bucket_tail[0], 1);
        assert_eq!(bucket_head[1], 1);
        assert_eq!(bucket_tail[1], 3);
        assert_eq!(bucket_head[2], 3);
        assert_eq!(bucket_tail[2], 5);
        assert_eq!(bucket_head[7], 7);
        assert_eq!(bucket_tail[7], 8);
    }

    #[test]
    fn test_guess_lms_sort() {
        let text: Vec<usize> = vec![3, 1, 2, 2, 1, 7, 5, 0];
        let (type_map, char_count) = build_type_map_u8(&text, 257);
        let (_, mut bucket_tail) = build_bucket(&char_count, 257);
        let pa = find_lms_character(&type_map);

        let mut sa: Vec<isize> = vec![-1; text.len()];
        guess_lms_sort(&text, &mut sa, &pa, &mut bucket_tail);

        assert_eq!(sa, vec![7, 4, 1, -1, -1, -1, -1, -1]);
    }

    #[test]
    fn test_find_lms_char() {
        let text: Vec<usize> = vec![3, 1, 2, 2, 1, 7, 5, 0];
        let (type_map, _) = build_type_map_u8(&text, 257);
        let pa = find_lms_character(&type_map);

        assert_eq!(pa, vec![1, 4, 7]);
    }

    #[test]
    fn test_induce_sort_l() {
        let text: Vec<usize> = vec![2, 1, 1, 2, 1, 1, 2, 1, 3, 0];
        let (type_map, char_count) = build_type_map_u8(&text, 257);
        let (mut bucket_head, mut bucket_tail) = build_bucket(&char_count, 257);
        let pa = find_lms_character(&type_map);

        let mut sa: Vec<isize> = vec![-1; text.len()];
        guess_lms_sort(&text, &mut sa, &pa, &mut bucket_tail);
        induce_sort_l(&text, &mut sa, &mut bucket_head, &type_map);

        assert_eq!(sa, vec![9, -1, -1, 7, 4, 1, 6, 3, 0, 8]);
    }

    #[test]
    fn test_induce_sort_s() {
        let text: Vec<usize> = vec![2, 1, 1, 2, 1, 1, 2, 1, 3, 0];
        let (type_map, char_count) = build_type_map_u8(&text, 257);
        let (mut bucket_head, mut bucket_tail) = build_bucket(&char_count, 257);
        let pa = find_lms_character(&type_map);

        let mut sa: Vec<isize> = vec![-1; text.len()];
        guess_lms_sort(&text, &mut sa, &pa, &mut bucket_tail);
        let (_, mut bucket_tail) = build_bucket(&char_count, 257);
        induce_sort_l(&text, &mut sa, &mut bucket_head, &type_map);
        induce_sort_s(&text, &mut sa, &mut bucket_tail, &type_map);

        assert_eq!(sa, vec![9, 4, 1, 5, 2, 7, 6, 3, 0, 8])
    }

    #[test]
    fn test_build_summary() {
        let text: Vec<usize> = vec![2, 1, 1, 2, 1, 1, 2, 1, 3, 0];
        let (type_map, char_count) = build_type_map_u8(&text, 257);
        let (mut bucket_head, mut bucket_tail) = build_bucket(&char_count, 257);
        let pa = find_lms_character(&type_map);

        let mut sa: Vec<isize> = vec![-1; text.len()];
        guess_lms_sort(&text, &mut sa, &pa, &mut bucket_tail);
        let (_, mut bucket_tail) = build_bucket(&char_count, 257);
        induce_sort_l(&text, &mut sa, &mut bucket_head, &type_map);
        induce_sort_s(&text, &mut sa, &mut bucket_tail, &type_map);

        let (_, s1) = build_summary_string(&text, &sa, &pa, &type_map);

        assert_eq!(s1, vec![2, 2, 3, 1])
    }

    #[test]
    fn build_sa1_1() {
        let text: Vec<usize> = vec![3, 1, 2, 2, 1, 7, 5, 0];
        let (type_map, char_count) = build_type_map_u8(&text, 257);
        let (mut bucket_head, mut bucket_tail) = build_bucket(&char_count, 257);
        let pa = find_lms_character(&type_map);

        let mut sa: Vec<isize> = vec![-1; text.len()];
        guess_lms_sort(&text, &mut sa, &pa, &mut bucket_tail);
        let (_, mut bucket_tail) = build_bucket(&char_count, 257);
        induce_sort_l(&text, &mut sa, &mut bucket_head, &type_map);
        induce_sort_s(&text, &mut sa, &mut bucket_tail, &type_map);
        let (alphabet_size, mut s1) = build_summary_string(&text, &sa, &pa, &type_map);
        let sa1 = build_summary_sa(&mut s1, alphabet_size);

        assert_eq!(sa1, vec![3, 2, 0, 1]);
    }

    #[test]
    fn test_exact_sort_lms_easy() {
        let text: Vec<usize> = vec![3, 1, 2, 2, 1, 7, 5, 0];
        let (type_map, char_count) = build_type_map_u8(&text, 257);
        let (mut bucket_head, mut bucket_tail) = build_bucket(&char_count, 257);
        let pa = find_lms_character(&type_map);

        let mut sa: Vec<isize> = vec![-1; text.len()];
        guess_lms_sort(&text, &mut sa, &pa, &mut bucket_tail);
        let (_, mut bucket_tail) = build_bucket(&char_count, 257);
        induce_sort_l(&text, &mut sa, &mut bucket_head, &type_map);
        induce_sort_s(&text, &mut sa, &mut bucket_tail, &type_map);
        let (alphabet_size, mut s1) = build_summary_string(&text, &sa, &pa, &type_map);
        let sa1 = build_summary_sa(&mut s1, alphabet_size);

        let mut sa: Vec<isize> = vec![-1; text.len()];
        let (_, mut bucket_tail) = build_bucket(&char_count, 257);
        exact_lms_sort(&text, &mut sa, &sa1, &pa, &mut bucket_tail);

        assert_eq!(sa, vec![7, 1, 4, -1, -1, -1, -1, -1]);
    }

    #[test]
    fn test_sais_u8_easy() {
        let mut text: Vec<usize> = vec![3, 1, 2, 2, 1, 7, 5];
        let sa = sais_u8(&mut text, 257);
        assert_eq!(vec![7, 1, 4, 3, 2, 0, 6, 5], sa)
    }

    #[test]
    fn test_sais_u8_hard() {
        let mut text: Vec<usize> = vec![2, 1, 1, 2, 1, 1, 2, 1, 3];
        let sa = sais_u8(&mut text, 257);
        assert_eq!(vec![9, 1, 4, 2, 5, 7, 0, 3, 6, 8], sa);
    }
}
