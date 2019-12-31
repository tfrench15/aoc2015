
use md5;

fn main() {
    let input = "ckczppom".to_string();
    let number = run(&input);

    println!("number is {}", number);
}

fn compute_md5(num: u64, input: &str) -> md5::Digest {
    let num_str = num.to_string();
    let data = format!("{}{}", input, num_str);
    let bytes = data.as_bytes();

    md5::compute(bytes)   
}

fn run(input: &str) -> u64 {
    let mut n = 1;

    loop {
        let digest = compute_md5(n, input);
        let hash = format!("{:x}", digest);

        if hash.starts_with("000000") {
            break n;
        }

        n += 1
    }
}

