use clap::{Parser, Subcommand};

#[derive(Parser)]
#[clap(version = "1.0")]
struct App {
    #[clap(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    // 新しい口座を作る
    New,
    // 口座に入金する
    Deposit,
    // 口座から出金する
    Withdraw,
    // CSVからインポートする
    Import,
    // レポートを出力する
    Report,
}

fn main() {
    // 構造体Argsで定義した形の引数を受け取ることを機体してparseする
    let _args = App::parse();
}