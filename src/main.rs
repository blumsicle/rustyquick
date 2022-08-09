use rand::prelude::*;

fn main() {
    let mut rng = thread_rng();
    let mut buffer: Vec<u32> = Vec::new();

    for _ in 0..10 {
        buffer.push(rng.gen_range(0..100));
    }

    println!(" pre quicksort: {buffer:?}");
    quicksort(&mut buffer);
    println!("post quicksort: {buffer:?}");
}

fn quicksort<T: Ord>(buffer: &mut [T]) {
    let end = buffer.len() - 1;
    _quicksort(buffer, 0, end as isize);
}

fn _quicksort<T: Ord>(buffer: &mut [T], start: isize, end: isize) {
    if start < end {
        let index = partition(buffer, start, end);
        _quicksort(buffer, start, index - 1);
        _quicksort(buffer, index + 1, end);
    }
}

fn partition<T: Ord>(buffer: &mut [T], start: isize, end: isize) -> isize {
    let pivot = end as usize;
    let mut pivot_index = start;

    for i in start..end {
        if buffer[i as usize] < buffer[pivot] {
            buffer.swap(i as usize, pivot_index as usize);
            pivot_index += 1;
        }
    }

    buffer.swap(pivot_index as usize, end as usize);
    pivot_index
}
