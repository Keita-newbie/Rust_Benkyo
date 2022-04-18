use rand::Rng;

fn main() {
    // if
    let mut counter: i32 = 1;
    if 0 < counter {
        println!("0 < counter");
    }else if counter < 0 {
        println!("counter < 0");
    } else {
        println!("counter == 0");
    }
    
    // for
    let rs = loop {
        println!("counter: {}", counter);
        counter += 1;
        if counter > 10 {
            // counterの値を返すことでrsに格納できる
            break counter;
        }
    };
    println!("rs: {}", rs);

    // for 範囲指定
    for count in 1..10 {
        println!("count: {}", count);
    }

    // for 配列
    let array: [&str; 6] = ["a", "b", "c", "d", "e", "f"];
    for arr in &array {
        println!("arr: {}", arr);
    }

    // while
    let mut counter2: i32 = 1;
    while counter2 < 10 {
        println!("counter2: {}", counter2);
        counter2 += 1;
    }

    // loop
    'main: loop {
        println!("mainループ");
        'sub: loop {
            println!("subループ");
            // subのループから出る(無限ループ)
            // break 'sub;
            println!("subループ終わり");
            // mainのループから出る
            break 'main;
        }
        println!("mainループ終わり");
    };

    // 2つの値一致するまで終われま10
    let mut rng = rand::thread_rng();
    let target_num = rng.gen_range(0, 50);
    let mut rand_num2: i32 = 0;
    'main2: loop {
        rand_num2 = rng.gen_range(0, 50);
        if target_num == rand_num2{
            println!("target_num: {} と 乱数: {}が一致", target_num, rand_num2);
            break 'main2;
        } else{
            println!("次!!!");
        }

    }

    // match: switchの厳しめver。値だけでなく、型まで見る
    let keyword: &str = "Iron Man";
    match keyword {
        "Iron Man" => println!("Tony Stark"),
        "Captain America" => println!("Steve Rogers"),
        "Spider Man" => println!("Peter Parker"),
        _ => println!("どなた？")
    }

    // 列挙型の場合はすべて網羅する必要がある
    enum Person {
        TonyStark,
        SteveRoger,
        PeterParker,
    }
    let target = Person::PeterParker;
    match target {
        Person::TonyStark => println!("Iron Man"),
        Person::SteveRoger => println!("Captain America"),
        Person::PeterParker => println!("Spider Man"),
        _ => println!("どなた？")
    }
}