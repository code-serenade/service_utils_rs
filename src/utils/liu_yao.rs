use rand::Rng;

fn divide_and_remainder(mut total: usize) -> usize {
    let mut rng = rand::rng();
    let left = rng.random_range(1..total); // 随机分堆
    let right = total - left;

    // 每堆去掉一根（天地各去一）
    let left_remain = left - 1;
    let right_remain = right;

    // 分别对两堆模4后剩下的数量
    let left_mod = if left_remain % 4 == 0 {
        4
    } else {
        left_remain % 4
    };
    let right_mod = if right_remain % 4 == 0 {
        4
    } else {
        right_remain % 4
    };

    total = total - (left_mod + right_mod + 1); // 去掉的草
    left_mod + right_mod + 1 // 返回此次的“得数”
}

fn generate_single_line() -> u8 {
    let mut total = 49;
    let mut count = 0;
    for _ in 0..3 {
        count += divide_and_remainder(total);
    }

    // 根据总数返回爻象：6 老阴，7 少阳，8 少阴，9 老阳
    match count {
        36 => 6,
        40 => 7,
        42 => 8,
        46 => 9,
        _ => panic!("Invalid count: {}", count),
    }
}

pub fn generate_hexagram() -> Vec<u8> {
    let mut lines = Vec::new();
    for _ in 0..6 {
        lines.push(generate_single_line());
    }
    lines
}
