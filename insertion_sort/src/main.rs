pub fn insertion_sort(slice: &mut [i32], steps: usize) {
    let len = slice.len();
    if len <= 1 || steps == 0 {
        return;
    }

    let actual_steps = steps.min(len - 1);
    
    for i in 1..=actual_steps {
        let mut j = i;
        while j > 0 && slice[j] < slice[j - 1] {
            slice.swap(j, j - 1);
            j -= 1;
        }
    }
}

fn main() {
    let mut target = [5, 3, 7, 2, 1, 6, 8, 4];
    insertion_sort(&mut target, 1);
    println!("{:?}", target);

    let mut target = [5, 3, 7, 2, 1, 6, 8, 4];
    let len = target.len();
    insertion_sort(&mut target, len - 1);
    println!("{:?}", target);
}