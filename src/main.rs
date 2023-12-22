use std::env;
use zippng::run;

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut is_deep = false;
    if args.len() > 1 {
        if args[1] == "-r" {
            is_deep = true;
        }

        if args[1] == "-v" {
            println!("Version: {:?}", env!("CARGO_PKG_VERSION"));
            return;
        }

        if args[1] == "-h" {
            println!(
                "\nCompress PNG format images in the current directory.\n
use: `zippng [-r|-v|-h]`
-r\tRecursively search for PNG files
-v\tView version 
-h\tHelp documentation
\n"
            );
            return;
        }
    }

    let pwd = match env::current_dir() {
        Ok(path) => path,
        Err(err) => panic!("{:#?}", err),
    };

    run(&pwd, is_deep);
}
