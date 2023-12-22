use oxipng;
use std::fs::{self, DirEntry};
use std::path::PathBuf;

#[derive(Debug)]
struct File {
    file_path: PathBuf,
    file_exet: String,
    is_directory: bool,
    size: u64,
}
// 获取图片信息
fn get_file_info(entry: &DirEntry) -> Result<File, &'static str> {
    let path = entry.path();
    let exet = match path.extension() {
        Some(res) => String::from(res.to_str().unwrap()).to_lowercase(),
        None => String::from(""),
    };
    let metadata = match entry.metadata() {
        Ok(res) => res,
        Err(_) => return Err("无法读取路径元数据"),
    };
    Ok(File {
        file_path: path,
        file_exet: exet,
        is_directory: metadata.is_dir(),
        size: metadata.len(),
    })
}
// 判断是否为png图片
fn is_png_file(file: &File) -> bool {
    if file.file_exet == "png" {
        return true;
    }
    return false;
}
// 压缩图片
fn zip_img(file_info: &File, options: &oxipng::Options) {
    let in_file = oxipng::InFile::Path(file_info.file_path.clone());
    let out_file = oxipng::OutFile::Path {
        path: Some(file_info.file_path.clone()),
        preserve_attrs: false,
    };
    println!(
        "开始压缩图片: {:?} \n压缩前文件大小: {:.2?}KB",
        file_info.file_path,
        file_info.size as f64 / 1024f64
    );
    match oxipng::optimize(&in_file, &out_file, &options) {
        Ok(_) => {
            let out_file_metadata = fs::metadata(&file_info.file_path).unwrap();
            println!(
                "结束压缩图片: {:?} \n压缩后文件大小: {:.2?}KB  \n",
                file_info.file_path,
                out_file_metadata.len() as f64 / 1024f64
            );
        }
        Err(_) => println!("图片: {:?} 压缩失败", file_info.file_path),
    };
}

pub fn run(path: &PathBuf, is_deep: bool) {
    let options = oxipng::Options::default();
    let dirs = fs::read_dir(path.clone()).unwrap();
    for item in dirs {
        let entry = item.unwrap();
        let file_info = get_file_info(&entry).unwrap();
        if is_png_file(&file_info) {
            zip_img(&file_info, &options)
        }
        // 是否深度递归
        if file_info.is_directory && is_deep {
            run(&file_info.file_path, is_deep)
        }
    }
}
