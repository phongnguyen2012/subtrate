use std::io;
fn compare_sub_arrays(a: &[i32], b: &[i32]) -> bool {
   let mut count = 0;
   let mut result = false;
   print!("Subtring ");
   for x in a.iter(){
    for y in b.iter(){
        if x == y {
            print!("{} ",x);
            count += 1
        }
    }
    
    }
    if count == b.len() {
        result = true;
    }
    result
}
fn compare_sub_string(s1: &str, s2: String) -> (String, i32) {
    let mut s3 = String::new();
    let mut count =0 ;
    for str in s1.split_whitespace(){
        if str == s2 {
            count += 1;
            s3 = str.to_string();
        }
    }
    
    (s3, count)
}

fn main() {

    let arr: [i32; 7] = [1, 2, 3, 4, 5, 6, 7];
    let arrcon: [i32; 4] = [3, 1, 6, 2];
    let ketqua = compare_sub_arrays(&arr, &arrcon);
    println!("Mang 2 la con mang 1 {}", ketqua);
    println!("*****************");
   

    
    let str1 = "mùa xuân, mùa hạ, mùa đông, mùa hè";
    println!("Vui long nhap 1 từ");
    let mut s4 = String::from("");
    io::stdin().read_line(&mut s4).unwrap();
    let s5 = s4.trim().to_string();
    
    let count_word = compare_sub_string(str1, s5.to_string());
    println!("Word = {} and  count = {}", count_word.0, count_word.1);

    

   
    

}
   
    


