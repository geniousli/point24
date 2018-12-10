pub fn sort(mut ary: &mut Vec<i64>) {
    let mut len = ary.len() - 1;
    build_heap(&mut ary);
    while len > 1 {
        swap(&mut ary, 0, len);
        len -= 1;
        rebuild_heap(&mut ary, 0, len);
    }
    swap(&mut ary, 0, len);
}

fn build_heap(mut ary: &mut Vec<i64>) {
    let max = ary.len() - 1;
    let mut len = (((max - 1) as f32) / 2.0).floor() as i64;
    while len >= 0 {
        rebuild_heap(&mut ary, len as usize, max);
        len -= 1;
    }
}

fn rebuild_heap(mut ary: &mut Vec<i64>, i_index: usize, size: usize) {
    let index = i_index;
    let left = (index * 2 + 1) as usize;
    let right = (index * 2 + 2) as usize;
    if left > size && right > size {
        return;
    }

    if left < size && right <= size {
        let left_value = ary[left];
        let right_value = ary[right];
        let mid_value = ary[index];
        if left_value < mid_value && right_value < mid_value {
            return;
        } else {
            if left_value > right_value {
                swap(&mut ary, left, index);
                rebuild_heap(&mut ary, left, size);
            } else if right_value > mid_value {
                swap(&mut ary, right, index);
                rebuild_heap(&mut ary, right, size);
            } else {
                swap(&mut ary, left, index);
                rebuild_heap(&mut ary, left, size);
            }
        }
    } else {
        let left_value = ary[left];
        let mid_value = ary[index];
        if left_value > mid_value {
            swap(&mut ary, left, index);
            rebuild_heap(&mut ary, left, size);
        } else {
            return;
        }
    }
}

fn swap(ary: &mut Vec<i64>, a: usize, b: usize) {
    let a_value = ary[a];
    ary[a] = ary[b];
    ary[b] = a_value;
}