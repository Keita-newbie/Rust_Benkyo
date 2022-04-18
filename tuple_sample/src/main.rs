fn main() {
    // タプル = 異なる方をまとめて格納できる
    println!("タプルのサンプル p42");

    // mut = 可変変数
    let mut t = (1, "2");
    // タプルをprintする場合、「:?」が必要っぽい。後で調べる。← fmt::Debugでの出力
    // println!("変更前 tuple data: {:?}", t);
    // fmt::Display での出力
    println!("変更前 tuple data: t[0] = {one}, t[1] = {two}",
             one = t.0,
             two = t.1);

    // 中身の更新
    t.0 = 2;
    t.1 = "3";
    // println!("変更後 tuple data: {:?}", t);
    println!("変更前 tuple data: t[0] = {one}, t[1] = {two}",
             one = t.0,
             two = t.1);
}
