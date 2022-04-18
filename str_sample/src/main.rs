fn main() {
    // 変数定義 let XXX(変数名): 型 = 値;。<<< 不変変数 >>>
    let _sample_var: i32 = 2;

    // let XXX: String(String型) = String::from("XXX"); Stringのfrom()関数
    let str_var1: String = String::from("Hello World!!");
    // &str = 固定長のstring型変数。参照する際によく使われる
    let str_var2: &str = &str_var1;
    // 固定長 → 可変長に変換。文字列が長いとその分メモリが食われる
    let str_var3: String = str_var2.to_string(); 

    // すべて「Hello World!!」
    println!("the str_var1 is: {}", str_var1);
    println!("the str_var2 is: {}", str_var2);
    println!("the str_var3 is: {}", str_var3);
}