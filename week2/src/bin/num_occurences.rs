fn find_num_occurrence_array(n: u8, arr: [u8; 10]) -> usize {
    // TODO: find and return the number of occurrences of "n" in array "arr".
    let mut count: usize = 0;
    for x in arr {
        if x == n {
            count += 1;
        }
    }
    return count;
}

fn find_num_occurrence_array_ref(n: u8, arr_ref: &[u8; 10]) -> usize {
    // TODO: find and return the number of occurrences of "n" in array referenced by "arr_ref".
    let mut count: usize = 0;
    for x in *arr_ref {
        if x == n {
            count += 1;
        }
    }
    return count;
}

fn find_num_occurrence_slice(n: u8, slice: &[u8]) -> usize {
    // TODO: find and return the number of occurrences of "n" in slice reference "slice".
    let mut count: usize = 0;
    for x in slice {
        if *x == n {
            count += 1;
        }
    }
    return count;
}

fn main() {
    // TODO: call find_num_occurrence_array in a loop (with every from 0 to 9 inclusive).
    let array: [u8; 10] = [4, 5, 6, 7, 8, 9, 5, 5, 6, 10];

    println!("Call with pbv");
    for n in array {
        let occurences: usize = find_num_occurrence_array(n, array);
        let end: &str;
        if occurences > 1 {
            end = "times";
        } else {
            end = "time";
        }
        println!("{} occurrences {} {}", n, occurences, end);
    }

    println!("\nCall with pbr");
    for n in array {
        let occurences: usize = find_num_occurrence_array_ref(n, &array);
        let end: &str;
        if occurences > 1 {
            end = "times";
        } else {
            end = "time";
        }
        println!("{} occurrences {} {}", n, occurences, end);
    }

    println!("\nCall with slice");
    let slice = &array[0..7];
    for n in slice {
        let occurences: usize = find_num_occurrence_slice(*n, slice);
        let end: &str;
        if occurences > 1 {
            end = "times";
        } else {
            end = "time";
        }
        println!("{} occurrences {} {}", n, occurences, end);
    }
}
