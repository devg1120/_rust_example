use std::process::{Command, Stdio};
use std::io::prelude::*;
use std::io::BufReader;

fn main() {
   // 子プロセスとして実行します
   let mut child = Command::new("ls")
       .args(&["-l", "-a"])
       .stdout(Stdio::piped())
       .spawn()
       .expect("failed to start `ls`");
   // stdout への出力のハンドラを取得します
   let mut stdout = child.stdout.take().unwrap();
   
   // BufReader をかませることで 1 行ずつ実行結果を取り出します
   let reader = BufReader::new(stdout);
   for line in reader.lines() {
       println!("{:?}", line);
   }
}

