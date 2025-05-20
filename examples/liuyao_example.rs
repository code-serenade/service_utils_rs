use service_utils_rs::utils::liu_yao::generate_hexagram;

fn main() {
    let hexagram = generate_hexagram();
    println!("你的卦象是（从下到上）:");

    for (i, line) in hexagram.iter().enumerate() {
        let symbol = match line {
            6 => "⚋ 变阴（老阴）",
            7 => "⚊ 阳（少阳）",
            8 => "⚋ 阴（少阴）",
            9 => "⚊ 变阳（老阳）",
            _ => "未知",
        };
        println!("第 {} 爻: {}", i + 1, symbol);
    }
}
