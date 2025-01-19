use std::fs::OpenOptions;
use clap::{Args, Parser, Subcommand};
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
    New(NewArgs), // <= Newサブコマンドに渡された引数をNewArgsで受け取る
    /// 口座に入金する
    Deposit,
    /// 口座から出金する
    Withdraw,
    /// CSVからインポートする
    Import,
    /// レポートを出力する
    Report,
}

#[derive(Args)] // <= helpやsuggestなどを用意してくれる
struct NewArgs {
    account_name: String,
}

impl NewArgs {
    fn run(&self) { // new サブコマンドの本体
        let file_name = format!("{}.csv", self.account_name);
        let mut writer = Writer::from_path(file_name).unwrap();
        writer.write_record(["日付", "用途", "金額"]).unwrap();
        writer.flush().unwrap();
    }
}

fn main() {
    let args = App::parse();
    match args.command {
        Command::New(args) => args.run(),
        Command::Deposit => deposit(),
        Command::Withdraw => withdraw(),
        Command::Import => unimplemented!(),
        Command::Report => unimplemented!(),
    }
}

fn deposit() {
    // 追記モードでファイルを開く
    let opne_option = OpenOptions::new()
        .write(true)
        .append(true) // <= 追記モード
        .open("a.csv")
        .unwrap();
    // open_optionを利用した形でwriterを設定
    let mut writer = Writer::from_writer(opne_option);
    writer.write_record(["1", "2", "3"]).unwrap();
    writer.flush().unwrap();
}

fn withdraw() {
    println!("withdraw");
}