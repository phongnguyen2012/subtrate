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
fn input_string() -> String {
    let mut input_string = String::new();
    io::stdin().read_line(&mut input_string).expect("Failed to read line");
    let input = input_string.trim();
    input.to_string()
}
fn remove_special_characters_in_string(s: &str) -> String {
    let mut s2 = String::new();
    for c in s.chars() {
        if c.is_alphabetic() || c.is_numeric() {
            s2.push(c);
        }
    }
    s2
}
fn main() {

    let arr: [i32; 7] = [1, 2, 3, 4, 5, 6, 7];
    let arrcon: [i32; 4] = [3, 1, 6, 2];
    let ketqua = compare_sub_arrays(&arr, &arrcon);
    println!("Mang 2 la con mang 1 {}", ketqua);
    println!("*****************");
   
     let data = read_file("poem.txt");
    let s = &*data;
    println!("Moi ban nhap chuoi can tim");
    let input = input_string();
    println!("{}", s);
    let s1 = compare_sub_string(s, input);
    println!("Tu {} xuat hien {}", s1.0, s1.1);
    println!("*****************");
    println!("chi lay chu va so loai bo het ky tu dac biet trong chuoi");
    let s = "{\"name\":\"xx/yyyy/machine/zzz/test_int4\",\"status\":\"online\",\"timestamp\":\"2021-06-11 18:20:42.231770800~! & #$%^&*() UTC\",\"value\":7}8668982856274}";

   let result = remove_special_characters_in_string(s);
   println!("{}", result);
   
    

}
   
    


