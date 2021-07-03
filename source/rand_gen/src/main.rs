use rand::Rng;
use rand_distr::{Distribution, Normal, NormalError};
use rand::distributions::{Standard, Alphanumeric};

#[derive(Debug)]
struct Point {
    x: i32,
    y: i32
}

impl Distribution<Point> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Point {
        let (rand_x, rand_y) = rng.gen();
        Point {
            x: rand_x,
            y: rand_y
        }
    }
}

fn main() -> std::result::Result<(), NormalError> {
    let mut rng = rand::thread_rng();
    println!("rand u8: {}", rng.gen::<u8>());
    println!("rand i32: {}", rng.gen::<i32>());

    println!("rand range(0..10): {}", rng.gen_range(0..10));

    let normal = Normal::new(1.0, 10.0)?;
    let rand_value = normal.sample(&mut rng);
    println!("normal distribution, value: {}", rand_value);

    // 生成自定义类型随机值
    let rand_point: Point = rng.gen();
    println!("norand_point: {:?}", rand_point);

    // rand::distributions::Alphanumeric 是字母数字样本，范围为 A-Z，a-z，0-9。
    let rand_string: String = rng.sample_iter(&Alphanumeric)
        .take(30)
        .map(|x| x as char)
        .collect();
    println!("rand_string: {}", rand_string);

    Ok(())
}
