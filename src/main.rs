use clap::Parser;

#[derive(Parser)]
#[clap(version = "1.0", author = "T4D4")]
struct Args {
    arg1: String,
    arg2: String,
}

fn main() {
    // 構造体Argsで定義した形の引数を受け取ることを機体してparseする
    let _args = Args::parse();
}