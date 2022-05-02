use std::io;
use rand::Rng;

// じゃんけん
fn main() {
    let hand: [&str; 3] = ["グー", "チョキ", "パー"];
    let mut rnd = rand::thread_rng();

    'main: loop {
        let choice: isize;
        let cpu: isize = rnd.gen_range(0..3);

        println!("最初はグー!!じゃん、けん....");
        // 0、1、2の値が入力されるまでループ
        'input_hand: loop {
            let mut input = String::new();

            println!("どの手を出す？(0: グー、1: チョキ、2: パー) \n>>>>>>>> ");
            io::stdin().read_line(&mut input).expect("読み込みに失敗しました。");
            // 入力値に含まれている改行を削除して数値に変換
            let hand_num: isize = match input.trim().parse() {
                Ok(num) => num,
                Err(_) => {
                    println!("数値を入力してね");
                    continue;
                },
            };

            if 0 > hand_num || hand_num > 2 {
                println!("0: グー、1: チョキ、2: パー のいずれかの値入力してね");
                continue;
            }
            choice = hand_num;

            break 'input_hand;
        }

        let player_idx: usize = choice as usize;
        let cpu_idx: usize = cpu as usize;

        println!("プレイヤーの手: {} \nコンピュータの手: {}", hand[player_idx], hand[cpu_idx]);

        let result: isize = (choice - cpu + 3) % 3;
        if result == 0 {
            println!("引き分け");
        } else if result == 1 {
            println!("残念..負けちゃった..");
        } else {
            println!("やったね！！プレイヤーの勝ち！！");
        }

        'input_play: loop {
            println!("まだ遊ぶ？ 0:もうやめる、1:もう一回戦\n>>>>>>>> ");

            let mut input = String::new();
            io::stdin().read_line(&mut input).expect("読み込みに失敗しました。");
            let play_flg: usize = match input.trim().parse() {
                Ok(num) => {
                    if num == 0 || num == 1 {
                        num
                    } else {
                        println!("数値(0 ~ 1)を入力してね");
                        continue;
                    }
                },
                Err(_) => {
                    println!("数値(0 ~ 1)を入力してね");
                    continue;
                },
            };

            if play_flg == 0 {
                break 'main;
            } else {
                break 'input_play;
            }
        }
    }



}