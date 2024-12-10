use rand::Rng;
use std::time::Instant;

pub fn run_test<T>(n: u32, initial: T, insert: fn(String, &T) -> T, mem: fn(&String, &T) -> bool) {

    // 定义字符串的最大长度
    const MAX_LENGTH: usize = 100;

    // 创建一个随机数生成器
    let mut rng = rand::thread_rng();

    let mut t = initial;

    // 开始计时
    let start_time = Instant::now();

    for _ in 0..n {
        // 随机生成字符串长度
        let len = rng.gen_range(1..=MAX_LENGTH);
        // 随机生成字符串内容
        let s: String = (0..len)
            .map(|_| rng.gen_range(b'a'..=b'z') as char)
            .collect();
        let tmp = insert(s, &t);
        t = tmp;
    }

    // 停止计时
    let elapsed = start_time.elapsed();

    // 输出结果
    println!("插入 {} 个字符串总共耗时: {:.2?}", n, elapsed);

    // 开始计时
    let start_time = Instant::now();

    for _ in 0..n {
        // 随机生成字符串长度
        let len = rng.gen_range(1..=MAX_LENGTH);
        // 随机生成字符串内容
        let s: String = (0..len)
            .map(|_| rng.gen_range(b'a'..=b'z') as char)
            .collect();
        mem(&s, &t);
    }

    // 停止计时
    let elapsed = start_time.elapsed();

    // 输出结果
    println!("查询 {} 个字符串总共耗时: {:.2?}", n, elapsed);
}