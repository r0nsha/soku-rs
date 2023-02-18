#[macro_export]
macro_rules! measure {
    ($label: literal, $block: block) => {{
        let now = std::time::Instant::now();
        let result = $block;
        println!("{}: {:.2?}", $label, now.elapsed());
        result
    }};
}
