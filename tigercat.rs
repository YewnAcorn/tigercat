// Tiger vs cat roamin space

fn main() {
    let cat_size: f64 = 10.0;
    let tiger_size = 300.0;
    let tiger_area = (5.0, 60.0);

    let cat_ratio = cat_size / tiger_size;


    println!("{}", cat_ratio);

    let answer_range  = area_converter(tiger_area, cat_ratio);

    let (x, y) = answer_range;

    println!("the range for a cat should be {} - {} square miles", x, y);
}


fn area_converter(input: (f64, f64), cat_ratio: f64) -> (f64, f64) {
    let (min, max) = input;
    let cat_min = min * cat_ratio;
    let cat_max = max * cat_ratio; 
    (cat_min, cat_max)
}

