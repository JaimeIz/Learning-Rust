fn main() {
    let v = vec![1, 2, 3, 4, 5];

    let third: &i32 = &v[2];
    println!("The third element is {}", third);

    let third: Option<&i32> = v.get(2);
    let thirdnumber = match third {
        Some(number) => { println!("The third element is {}", number); number },
        None => { println!("There is no third element."); &0 }
    };

    // The next line crashes the program!!
    // let does_not_exist = &v[100];
    let does_not_exist = v.get(100);
    if let None = does_not_exist {
        println!("there is nothing in the 101st position");
    }

    let range: &[i32] = &v[0..3];
    dbg!(range);

    let range: Option<&[i32]> = v.get(0..9);
    /*
    match third {
        Some(number) => println!("The third element is {}", number),
        None => {},
        _ => {}
    }
    */
    if let Some(nums) = range { 
        dbg!(nums);
    }
    
    let mut n = 0;
    while let Some(num) = v.get(n) {
        n += 1;
        print!("{num}, ");
    }

    for n in v.iter() {
        println!("{n}");
    }
}
