use clap::{Parser, Subcommand};
use csv::Writer;

#[derive(Parser)]
#[clap(version = "1.0")]
struct App {
    #[clap(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// 新しい口座を作る
    New,
    /// 口座に入金する
    Deposit,
    /// 口座から出金する
    Withdraw,
    /// CSVからインポートする
    Import,
    /// レポートを出力する
    Report,
}

fn main() {
    let args = App::parse();
    match args.command {
        Command::New => new(),
        Command::Deposit => unimplemented!(),
        Command::Withdraw => unimplemented!(),
        Command::Import => unimplemented!(),
        Command::Report => unimplemented!(),
    }
}

fn new() {
    // accounts.csvという名前でcsvファイルを作成
    let mut writer = Writer::from_path("accounts.csv").unwrap();
    writer
        .write_record(["日付", "用途", "金額"]) // ヘッダーを書き込む
        .unwrap();
    writer.flush().unwrap();
}