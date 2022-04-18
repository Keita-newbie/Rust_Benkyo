fn main() {
    // [要素の型; 要素数]。サイズはコンパイル時に決まっている必要がある。
    let mut arr: [i32; 3] = [0, 1, 2];

    // [0; 3] 3つの要素(全要素)に対して 0 を格納する。
    let arr_2: [i32; 3] = [0; 3];

    // arrに arr_2の値: 0 を格納
    arr[1] = arr_2[1];
    arr[2] = arr_2[2];
    println!("array data: {:?}", &arr)
}