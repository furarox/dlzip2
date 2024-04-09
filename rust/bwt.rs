pub const BWT_MARKER: usize = 256;

fn counting_sort(
    content: &Vec<usize>,
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

fn get_element_from_rotation(content: &Vec<usize>, rotation: isize, n_col: isize) -> usize {
    let idx = (n_col - rotation + content.len() as isize) as usize % content.len();
    content[idx as usize]
}

type GetCharFn = fn(&Vec<usize>, isize, isize) -> usize;

type StackElement = (isize, usize, usize);

fn radix_sort(text_content: &Vec<usize>, list_rotation: &mut [isize]) {
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

    while !stack.is_empty() {
        let (n_col, n_deb, n_end) = stack.pop().unwrap();
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

pub fn bwt_encode(mut text_content: Vec<usize>) -> Vec<usize> {
    let mut result: Vec<usize> = Vec::with_capacity(text_content.len());

    while text_content.len() > 0 {
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

fn find_nth(text_content: &Vec<usize>, el: usize, occur: usize, last_found: &mut [Vec<usize>]) -> usize {
    if last_found[256].is_empty() {
        for (idx, &text_char) in text_content.iter().enumerate() {
            last_found[text_char].push(idx);
        }
    }
    last_found[el][occur - 1]
}

fn find_rank(text_content: &Vec<usize>, idx: usize) -> usize {
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

    while text_content.len() > 0 {
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