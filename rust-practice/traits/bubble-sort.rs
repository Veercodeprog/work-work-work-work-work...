fn bubble_sort<T: Ord>(arrorvec: &mut [T]) {
    let mut swapped = true;
    while swapped {
        swapped = false;
        for i in 0..arr.len() - 1 {
            if arr[i] > arr[i + 1] {
                arr.swap(i, i + 1);
                swapped = true;
            }
        }
    }
}

fn main() {
    let mut data = [64, 34, 25, 12, 22, 11, 90];
    println!("Unsorted array: {:?}", data);

    bubble_sort(&mut data);

    println!("Sorted array: {:?}", data);
    let mut vec5: Vec<i32> = (10..15).collect();
    vec5.push(4);
    println!("{:?}", vec5);
    bubble_sort(&mut vec5);
    println!("{:?}", vec5);
}
