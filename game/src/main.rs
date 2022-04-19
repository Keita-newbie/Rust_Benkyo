// 数当てゲーム
use rand::Rng;
use std::io;
use std::cmp::Ordering;

fn main() {
    
    let mut rand = rand::thread_rng();
    let range: [u32; 2] = [1, 10];

    let mut target_num: u32 = rand.gen_range(range[0]..range[1]);

    'main: loop {
        let mut input: String = String::new();
        let mut input2: String = String::new();

        println!(">>>> 数当てゲーム <<<<\n");
        println!("予想値を入力してください。{} ～ {} \n>>>>>", range[0], range[1]);
        // 標準入力
        io::stdin().read_line(&mut input).expect("読み込みに失敗しました。");
        // 入力値から改行コードを削除しつつ、u32の型に変換
        let guess_num: u32 = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("変な値入れたからやり直し\n");
                continue;
            },
        };

        match guess_num.cmp(&target_num) {
            Ordering::Less => println!("小さすぎる.."),
            Ordering::Greater => println!("大きすぎる.."),
            Ordering::Equal => {
                println!("正解!! 答えは {} でした", target_num);
            }
        }

        println!("もう一回挑戦する？0=もうやめる, 1=まだ遊ぶ\n");
        io::stdin().read_line(&mut input2).expect("読み込みに失敗しました。\n");
        // 入力値から改行コードを削除しつつ、u32の型に変換
        let play_flg: u32 = match input2.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("変な値入れたから強制的に遊ばせるね\n");
                target_num = rand.gen_range(range[0]..range[1]);
                continue;
            },
        };

        match play_flg {
            0 => {
                println!("終了..");
                break 'main;
            },
            1 => {
                println!("もういっちょ!!!");
                target_num = rand.gen_range(range[0]..range[1]);
            },
            _ => {
                println!("変な値入れたから強制でもう一回");
                target_num = rand.gen_range(range[0]..range[1]);
            },
        };
    }
}
