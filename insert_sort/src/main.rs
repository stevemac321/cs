
fn test_gen_ins() {
    {

    println!("gen sort less");
    let less = |x,y| x < y;
    let mut test1 = vec![5, 2, 9, 1, 5, 6];
    let mut test2 = vec![3, 0, -1, 8, 7, 2];
    let mut test3 = vec![10, 20, 30, 40, 50];
    let mut test4 = vec![50, 40, 30, 20, 10];
    let mut test5 = vec![1, 2, 3, 4, 5];

    gen_ins_sort(&mut test1, less);
    gen_ins_sort(&mut test2, less);
    gen_ins_sort(&mut test3, less);
    gen_ins_sort(&mut test4, less);
    gen_ins_sort(&mut test5, less);

    println!("Sorted test1: {:?}", test1);
    println!("Sorted test2: {:?}", test2);
    println!("Sorted test3: {:?}", test3);
    println!("Sorted test4: {:?}", test4);
    println!("Sorted test5: {:?}", test5);
    }
    {
    println!("gen sort greater");
    let greater = |x,y| x > y;
    let mut test1 = vec![5, 2, 9, 1, 5, 6];
    let mut test2 = vec![3, 0, -1, 8, 7, 2];
    let mut test3 = vec![10, 20, 30, 40, 50];
    let mut test4 = vec![50, 40, 30, 20, 10];
    let mut test5 = vec![1, 2, 3, 4, 5];

    gen_ins_sort(&mut test1, greater);
    gen_ins_sort(&mut test2, greater);
    gen_ins_sort(&mut test3, greater);
    gen_ins_sort(&mut test4, greater);
    gen_ins_sort(&mut test5, greater);

    println!("Sorted test1: {:?}", test1);
    println!("Sorted test2: {:?}", test2);
    println!("Sorted test3: {:?}", test3);
    println!("Sorted test4: {:?}", test4);
    println!("Sorted test5: {:?}", test5);
    }
}




fn test_ins() {
    let mut test1 = vec![5, 2, 9, 1, 5, 6];
    let mut test2 = vec![3, 0, -1, 8, 7, 2];
    let mut test3 = vec![10, 20, 30, 40, 50];
    let mut test4 = vec![50, 40, 30, 20, 10];
    let mut test5 = vec![1, 2, 3, 4, 5];

    ins_sort(&mut test1);
    ins_sort(&mut test2);
    ins_sort(&mut test3);
    ins_sort(&mut test4);
    ins_sort(&mut test5);

    println!("Sorted test1: {:?}", test1);
    println!("Sorted test2: {:?}", test2);
    println!("Sorted test3: {:?}", test3);
    println!("Sorted test4: {:?}", test4);
    println!("Sorted test5: {:?}", test5);
}
fn test_sel() {
    let mut test1 = vec![5, 2, 9, 1, 5, 6];
    let mut test2 = vec![3, 0, -1, 8, 7, 2];
    let mut test3 = vec![10, 20, 30, 40, 50];
    let mut test4 = vec![50, 40, 30, 20, 10];
    let mut test5 = vec![1, 2, 3, 4, 5];

    sel_sort(&mut test1);
    sel_sort(&mut test2);
    sel_sort(&mut test3);
    sel_sort(&mut test4);
    ins_sort(&mut test5);

    println!("Sorted test1: {:?}", test1);
    println!("Sorted test2: {:?}", test2);
    println!("Sorted test3: {:?}", test3);
    println!("Sorted test4: {:?}", test4);
    println!("Sorted test5: {:?}", test5);
}
//CLRS algorithm
//                                              Cost   Times
fn ins_sort(arr: &mut [i32]) {
    for j in 1..arr.len() {//                      c1     n
        let key = arr[j];    //                    c2     n-1
        let mut i = j;        //                   c3     n-1

        while i > 0 && arr[i - 1] > key {  //      c4     ∑ from j=2 to n, t_j
            arr[i] = arr[i - 1];            //     c5     ∑ from j=2 to n, (t_j - 1)
            i -= 1;                         //     c6     "                      "
        }
        arr[i] = key;                        //    c7     n-1
    }
}

fn sel_sort(arr: &mut [i32]) {
    for i in 0..arr.len() - 1 {
       let mut lowest =i;
        for j in i + 1..arr.len() {
            if arr[j] < arr[lowest] {
                lowest = j;
            }
        }
        if lowest != i {
            arr.swap(i, lowest);
        }
    }
}
fn main() {
    test_sel();
    test_ins();
    test_gen_ins();
}

fn gen_ins_sort<Compare>(arr: &mut [i32], cmp: Compare)
where
    Compare: Fn(i32, i32) ->bool,
{
    for j in 1..arr.len() {
        let key = arr[j];    
        let mut i = j;      

        while i > 0 && cmp(key, arr[i - 1]) { // swap args so that calling with "less" implies
            arr[i] = arr[i - 1];              // ascending sorted order 
            i -= 1;                        
        }
        arr[i] = key;                     
    }
}

