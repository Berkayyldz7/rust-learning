#[allow(unused_variables)]
#[allow(unused_imports)] // Uyarıları gizler.
#[allow(non_snake_case)]

use std::{f32::consts, io};
const MY_FIRST_CONST: u32 = 3 * 60 * 60;
// let mut myVar: String = "my first variable"; ---> Hata verir çünkü main fonksiyonu kapsamında tanımlanabilir.

fn main() {
    // let mut myVar: String = "Variable_1"; ---> String ile &str birbirinden farklı şeylerdir. Dolayısıyla böyle kafımıza göre tanımlama yapamıyoruz.

    // String'ler memory heap'te hayat bulurken, string literaller read-only memory'de can bulur.

    let mut myVar2 = "dnm"; //  &str; "dnm" stringinin bellekteki pointer'ını ve pointer counter'ını tutar.

    println!("{myVar2} for this step myVar2 equal dnm"); 

    myVar2 = "abcdf"; // Burada değişen şey myVar2 DEĞİL! Notlarda detaylı anlatıldı.

    let mut new_str = String::new(); // heap üzerinde dinamik bir adresleme oluşturur.

    match io::stdin().read_line(&mut new_str){
        Ok(prm) => {
            println!("{}", prm);
        },
        Err(_)=>{
            println!("ann error occured.")
        }
    }
    
    

    println!("{}",myVar2);


    let my_var3: &str = "hello";


    // let x = new_str.as_mut_ptr();

    // unsafe {
    //     println!("the adress is{:?}", x);
    // }


    for byte in my_var3.bytes(){  // String metinleri binary olarak çıktı almamızı sağlar.
        println!("{:b}", byte)        // {:b} binary olarak bas demektir.
     }

    // 1101000
    // 1100101
    // 1101100
    // 1101100
    // 1101111

    let my_num = 2110;

    let pointer = &my_num as *const i32;

    println!("{:p}", pointer);

    {
        let x :&str =  "inner scope values";
        println!("{} hellos from inner scope", x);

        let my_num = 211015;
        println!("{} inside of the inner scope hellos for firs shadowing", my_num );
    }

    {
        let x: u32 = 2356;
        println!("{:#x}", x);

        // 0x934

        println!("{:e}", x)

        // 2.356e3 ---> e3'ün anlamı 10 üzeri 3 demek
    }


}
