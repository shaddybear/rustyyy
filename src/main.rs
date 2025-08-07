fn main() {
     let is_male: bool = true;
     let is_above_18:  bool = true;

     if is_male {
        println!("You are a male");
     }

     else {
        println!("You are a female");
     }
     if is_male && is_above_18 {
         println!("You are a legal male");
     }
}