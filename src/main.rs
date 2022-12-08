fn main() {
    let mut arr = vec![-1, 3, 1, 2, 8, 3, 2, 9, 0];
    // let arr = array_sort(&mut arr);
    array_sort1(&mut arr);
    println!("{:?}", arr);
}

fn array_sort(arr: &mut Vec<i32>) -> &mut Vec<i32> {
    let mut temp;
    let mut j = 1;
    while j < arr.len() - 1 {
        let mut i = 0;
        while i < arr.len() - j {
            if arr[i] > arr[i + 1] {
                temp = arr[i + 1];
                arr[i + 1] = arr[i];
                arr[i] = temp;
            }
            i = i + 1;
        }
        // println!("{:?}", arr);
        j = j + 1;
    }
    // println!("{:?}", arr);
    return arr;
}

fn array_sort1<T: PartialOrd + Copy>(arr: &mut [T]) -> &[T] {
    let mut temp;
    let mut j = 1;
    while j < arr.len() - 1 {
        let mut i = 0;
        while i < arr.len() - j {
            if arr[i] > arr[i + 1] {
                temp = arr[i + 1];
                arr[i + 1] = arr[i];
                arr[i] = temp;
            }
            i = i + 1;
        }
        // println!("{:?}", arr);
        j = j + 1;
    }
    // println!("{:?}", arr);
    return arr;
}
