use chrono::NaiveDate;
use clap::{Args, Parser, Subcommand};
use csv::{Reader, Writer, WriterBuilder};
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, fs::OpenOptions};

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
    Deposit(DepositArgs),
    /// 口座から出金する
    Withdraw(WithdrawArgs),
    /// CSVからインポートする
    Import(ImportArgs),
    /// レポートを出力する
    Report(ReportArgs),
}

#[derive(Args)] // <= helpやsuggestなどを用意してくれる
struct NewArgs {
    account_name: String,
}

impl NewArgs {
    fn run(&self) {
        // new サブコマンドの本体
        let file_name = format!("{}.csv", self.account_name);
        let mut writer = Writer::from_path(file_name).unwrap();
        writer.write_record(["日付", "用途", "金額"]).unwrap();
        writer.flush().unwrap();
    }
}

#[derive(Args)]
struct DepositArgs {
    // <= deposit サブコマンドの引数の型を定義
    account_name: String,
    date: NaiveDate,
    usage: String,
    amount: u32,
}
impl DepositArgs {
    fn run(&self) {
        let open_option = OpenOptions::new()
            .write(true)
            .append(true) // <= 追記モード
            .open(format!("{}.csv", self.account_name))
            .unwrap();
        // open_optionを利用した形でwriterを設定
        let mut writer = Writer::from_writer(open_option);
        writer
            .write_record(&[
                self.date.format("%Y-%m-%d").to_string(),
                self.usage.to_string(),
                self.amount.to_string(),
            ])
            .unwrap();
        writer.flush().unwrap();
    }
}

#[derive(Args)]
struct WithdrawArgs {
    account_name: String,
    date: NaiveDate,
    usage: String,
    amount: u32,
}
impl WithdrawArgs {
    fn run(&self) {
        let open_option = OpenOptions::new()
            .write(true)
            .append(true) // <= 追記モード
            .open(format!("{}.csv", self.account_name))
            .unwrap();
        // open_optionを利用した形でwriterを設定
        let mut writer = Writer::from_writer(open_option);
        writer
            .write_record(&[
                self.date.format("%Y-%m-%d").to_string(),
                self.usage.to_string(),
                // MEMO: depositとは異なり、出金額はマイナスにしている
                format!("-{}", self.amount),
            ])
            .unwrap();
    }
}
#[derive(Args)]
struct ImportArgs {
    src_file_name: String,    // importするデータファイル
    dst_account_name: String, // import先としてkakeiboで管理している口座名
}
impl ImportArgs {
    fn run(&self) {
        let open_option = OpenOptions::new()
            .write(true)
            .append(true) // <= 追記モード
            .open(format!("{}.csv", self.dst_account_name))
            .unwrap();
        let mut writer = WriterBuilder::new()
            .has_headers(false) // <= ヘッダーを書かない
            .from_writer(open_option);
        let mut reader = Reader::from_path(&self.src_file_name).unwrap();
        for result in reader.deserialize() {
            // CSVの各行がRecord型として読み込まれることを想定
            let record: Record = result.unwrap();
            writer.serialize(record).unwrap();
        }
    }
}

// CSVの各絡むで期待する型を定義
#[derive(Serialize, Deserialize)]
struct Record {
    日付: NaiveDate,
    用途: String,
    金額: i32,
}
#[derive(Args)]
struct ReportArgs {
    files: Vec<String>,
}
impl ReportArgs {
    fn run(&self) {
        let mut map = HashMap::new();
        for file in &self.files {
            let mut reader = Reader::from_path(file).unwrap();
            for result in reader.records() {
                let record = result.unwrap();
                let amount: i32 = record[2].parse().unwrap();
                let date: NaiveDate = record[0].parse().unwrap();
                let sum = map.entry(date.format("%Y-%m").to_string()).or_insert(0);
                *sum += amount;
            }
        }
        println!("{:?}", map);
    }
}

fn main() {
    let args = App::parse();
    match args.command {
        Command::New(args) => args.run(),
        Command::Deposit(args) => args.run(),
        Command::Withdraw(args) => args.run(),
        Command::Import(args) => args.run(),
        Command::Report(args) => args.run(),
    }
}
