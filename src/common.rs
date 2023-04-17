pub fn use_fn(a:u8, b:u8) -> u8 {
    a + b
}

pub fn use_return(num: u32) -> bool {
    num % 2 == 1
}

pub fn use_for() {

    fn use_iter(array:[u8; 10]) {
        for i in array.iter() {
            println!("array[{}]", i);
        }
    }

    fn use_base_for(array:[u8; 10]) {
        for i in array {
            println!("array[{}]", i);
        }
    }

    fn use_range(array:[u8; 10]) {
        for i in 0..array.len() {
            println!("array[{}] : {}",i ,array[i]);
        }
    }
    let array: [u8; 10] = [12, 21, 22, 23, 24, 25, 26, 27, 28, 29];
    use_iter(array);
    use_base_for(array);
    use_range(array);
}

pub fn use_while() {
    let mut i = 0;
    while i < 10 {
        println!("i with while: {}", i);
        i += 1;
    }
}

pub fn use_loop() {
    let mut i = 0;
    loop {
        i += 1;
        if i == 10 {
            println!("current i is {}", i);
            break;
        }
    }
}

pub fn use_loop_index() {
    let mut i = 0;
    let index = loop {
        i += 1;
        if i == 10 {
            break i;
        }
    };
    println!("index: {}", index);
}