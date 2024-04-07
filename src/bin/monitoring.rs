extern crate image;

use statrs::distribution::{ChiSquared, ContinuousCDF};

fn main(){
    // let img_path = "./image/Lena.bmp";
    let img_path = "./image/Steg.png";
    let img = image::open(img_path).expect("Failed to open image").to_luma8();
    let mut kk = [0u32;256];
    println!("{} {}",img.height(), img.width());
    for y in 0..img.height() {
        for x in 0..img.width() {
            // 获取当前像素的灰度值
            let pixel_value = img.get_pixel(x, y).0[0];
            kk[pixel_value as usize] += 1
        }
    }
    
    let mut chi_square = 0.0;
    let mut k = 0;
    let mut sum = 0;
    for i in 0..128 {
        sum += kk[2 * i] + kk[2 * i + 1];
        let expected = (kk[2 * i] + kk[2 * i + 1]) as f64 / 2.0;
        let observed = kk[2 * i] as f64;
        let diff = observed - expected;
        if expected != 0.0 {
            chi_square += diff * diff / expected;
            k = k+1;
        }
        // println!("i = {}, {}, {}, {} , {}",i,kk[2*i],kk[2*i+1],diff, chi_square);
    }
    println!("sum = {sum}");
    println!("Chi-square value: {}, k = {}", chi_square, k);
    // 创建卡方分布对象
    k = k - 1;
    let chi_squared = ChiSquared::new(k as f64).unwrap();
    // 计算卡方分布的CDF
    let p = 1.0 - chi_squared.cdf(chi_square);

    println!("p = {}",p);
}