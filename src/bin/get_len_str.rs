extern crate image;
extern crate minifb;
use minifb::{Window, WindowOptions, ScaleMode, Scale};
use image::{GrayImage,ImageBuffer,Luma};
use std::fs::File;
use std::io::Read;

fn get_bit(s:String, u:&mut Vec<u8>){
    // 遍历字节序列并获取每个字节的每个 bit
    u.clear();
    let bytes = s.as_bytes();
    for &byte in bytes {
        for i in (0..8).rev() {
            // 获取字节中的每个 bit
            let bit = (byte >> i) & 1;
            u.push(bit)
        }
    }
}

fn from_bit(u:& Vec<u8>)->Vec<u8>{
    let mut bytes:Vec<u8> = Vec::new();
    for i in 0..(u.len()/8){
        let mut byte = 0u8;
        for k in 0..8{
            byte <<= 1;
            byte += u[i*8 + k]
        }
        bytes.push(byte);
        if byte == 0{
            return bytes
        }
    }
    bytes
}


fn _display_img(img:&ImageBuffer<Luma<u8>, Vec<u8>>){
    // 获取图像尺寸
    let (width, height) = img.dimensions();

    // 创建窗口
    let mut window = Window::new(
        "Gray Image Display",
        width as usize,
        height as usize,
        WindowOptions {
            resize: true,
            scale_mode: ScaleMode::AspectRatioStretch,
            scale: Scale::FitScreen,
            ..WindowOptions::default()
        },
    )
    .expect("Failed to create window");

    // 将图像转换为灰度数组
    let gray_img = img;

    // 将灰度数组转换为可显示的 RGBA 格式
    let mut buffer: Vec<u32> = Vec::with_capacity((width * height) as usize);
    for pixel in gray_img.pixels() {
        let intensity = pixel[0] as u32;
        buffer.push((intensity << 16) | (intensity << 8) | intensity);
    }

    // 显示灰度图像
    while window.is_open() && !window.is_key_down(minifb::Key::Escape) {
        window
            .update_with_buffer(&buffer, width as usize, height as usize)
            .expect("Failed to update window");
    }
}

fn processed_gray_image(img:&ImageBuffer<Luma<u8>, Vec<u8>>, s:String)->ImageBuffer<Luma<u8>, Vec<u8>>{
    // 创建一个新的灰度图像，用于存储处理后的结果
    let mut processed_img = GrayImage::new(img.width(), img.height());
    let mut binary_array:Vec<u8> = vec![0];
    get_bit(s, &mut binary_array);
    // 遍历图像的每个像素并进行处理
    for y in 0..img.height() {
        for x in 0..img.width() {
            // 获取当前像素的灰度值
            let mut pixel_value = img.get_pixel(x, y)[0];

            // 将像素的最低位替换为二进制数组中相应位置上的值
            if let Some(&bit) = binary_array.get((y * img.width() + x) as usize) {
                if bit == 0 {
                    // 清除最低位
                    pixel_value &= 0b11111110;
                } else {
                    // 将最低位设置为 1
                    pixel_value |= 0b00000001;
                }
            }

            // 将处理后的灰度值设置到新图像的对应位置
            processed_img.put_pixel(x, y, image::Luma([pixel_value]));
        }
    }
    processed_img
}

fn decode(img:&ImageBuffer<Luma<u8>, Vec<u8>>)->Vec<u8>{
    // 二进制数组，假设长度与图像的像素数相同
    let mut binary_array:Vec<u8> = vec![0];
    binary_array.clear();
    // 遍历图像的每个像素并进行处理
    for y in 0..img.height() {
        for x in 0..img.width() {
            // 获取当前像素的灰度值
            let pixel_value = img.get_pixel(x, y)[0];

            // 将像素的最低位取出
            let bit:u8 = pixel_value & 0b00000001;
            binary_array.push(bit);
        }
    }
    binary_array
}

fn all(args_vec:Vec<String>){
    let img_path = args_vec.get(2).expect("请输入图片地址");
    let img_save_path = args_vec.get(3).expect("请输入图片保存地址");
    let message = args_vec.get(4).expect("请输入隐藏信息字符串");
    // println!("img_path = {}",img_path);
    // println!("img_save_path = {}",img_save_path);
    // println!("message = {}", message);
    // 打开灰度图像文件
    let img = image::open(img_path).expect("Failed to open image").to_luma8();

    // 调用函数处理图像,实现信息隐藏
    let processed_img = processed_gray_image(&img,message.to_string()+"\0");

    // 保存处理后的图像
    processed_img.save(img_save_path).expect("Failed to save image");
    println!("message 已经隐藏到 {}",img_save_path);
    // _display_img(&processed_img);

    // 提取隐藏的信息
    // 提取最低位
    let bytes_bit = decode(&processed_img);
    // 将提取出来的bit转换为 u8 数组
    let s = from_bit(&bytes_bit);
    // 将 u8 数组转换为string
    let ss = String::from_utf8_lossy(&s);
    println!("隐藏信息提取,得到message.len = {:?}",s.len())
}


fn main(){
    let image = image::open("./image/Lena.bmp").expect("Failed to open image").to_luma8();
    let size = image.height()*image.width();
    println!("{} {}", image.height(), image.width());
    println!("size = {}", size);
     // 打开文件
     let mut file = File::open("./message.txt").unwrap();
    
     // 创建一个缓冲区来存储读取的数据
     let mut buffer = [0; (512*512 - 2)/8];
     
     // 读取文件的前1230080字节
     file.read(&mut buffer).unwrap();
 
     // 将读取的数据转换为字符串（假设文件是文本文件）
     let content = String::from_utf8_lossy(&buffer);
    println!("{}", content.as_bytes().len()*8);
    let mut args_vec = vec!["sda".to_string(),"sda".to_string(),"./image/ppp.jpg".to_string(), "./image/Steg.png".to_string()];
    args_vec.push(content.to_string());
    all(args_vec)
}