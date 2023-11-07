use rand::Rng;

fn main() {
    let mut v: [i32; 10] = [0; 10];
    let mut i = 0;

    while i < 10 {
       let num = rand::thread_rng().gen_range(1..=10);
        if v.contains(&num) {
            continue;
        }
        v[i] = num;
        i += 1;
    }
    println!("before sort v: {:?}", v);

    let mut m = 0;
    while m < 10 {
        let mut n = 0;
        while n < 10 {
            if v[m] < v[n] {
                let temp = v[m];
                v[m] = v[n];
                v[n] = temp;
            }
            n += 1;
        }
        m += 1;
    }
    println!("after sort v: {:?}", v);
}
