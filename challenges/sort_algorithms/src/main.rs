use std::cmp::Ordering;

fn main() {
    let mut stuff = [1, 6, 2, 3, 0, 10];
    println!("Stuff before {:?}", stuff);
    bubble_sort(&mut stuff);
    println!("Stuff after {:?}", stuff);

    let mut other_stuff = [1, 9, 14, 5, 6, 7, 1, 19, 22];
    println!("other_stuff before {:?}", other_stuff);
    merge_sort(&mut other_stuff);
    println!("other_stuff after {:?}", other_stuff);
}

fn bubble_sort(arr: &mut [i32]) {
    for i in 0..arr.len() {
        for j in i..arr.len() {
            if arr[i] > arr[j] {
                let temp = arr[i];
                arr[i] = arr[j];
                arr[j] = temp;
            }
        }
    }
}

fn merge_sort(vec: &mut [i32]) {
    let len = vec.len();
    if len < 2 {
        return;
    }
    // split in halves and sort those first
    let (mut left, mut right) = vec.split_at_mut(len / 2);
    merge_sort(&mut left);
    merge_sort(&mut right);

    // create a temp buffer to put in the sorted values
    let mut temp: Vec<i32> = Vec::new();

    let mut l = left.iter().peekable();
    let mut r = right.iter().peekable();

    //do the sort here
    while (l.peek() != None) || (r.peek() != None) {
        // check if either iter is empty
        if l.peek() == None {
            temp.push(*r.next().unwrap());
            continue;
        }
        if r.peek() == None {
            temp.push(*l.next().unwrap());
            continue;
        }

        // the actual comparison
        match l.peek().cmp(&r.peek()) {
            Ordering::Less | Ordering::Equal => {
                temp.push(*l.next().unwrap());
            }
            Ordering::Greater => {
                temp.push(*r.next().unwrap());
            }
        }
    }
    vec.copy_from_slice(&temp[..]);
}
