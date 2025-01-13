fn main() {
    // コマンドライン引数の一覧を出力する
    for arg in std::env::args() {
        println!("{}", arg);
    }
}
