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

        // Shadowing işlemleri
        
        let x :&str =  "inner scope values";
        println!("{} hellos from inner scope", x);

        let my_num = 211015;
        println!("{} inside of the inner scope hellos for firs shadowing", my_num );
    }

    {

        // Decimal veriyi hexadecimal olarak yazdırma 
        let x: u32 = 2356;
        println!("{:#x}", x);

        // 0x934

        println!("{:e}", x)

        // 2.356e3 ---> e3'ün anlamı 10 üzeri 3 demek
    }


    { 
        // Pointer üzerinden veriye erişme ve belllekteki adresini ekrana yazdırma
        let x = 25;
        let ref_x = &x;

        println!("{} x'in refensı ...", ref_x);

        let mut y:i32 = 10;
        y = *ref_x;  // *ref_x demek x'in pointer'ı demek yani onun refer ettiği veri alanı demek.
        println!("{} :y values equal to", y);


        let mut z = 15;
        let ref_z = &mut z as *const i32;

        println!("{:p} z'nin pointerı", ref_z);

        // 0x16d8d6584 z'nin pointerı
    }

    { 
        // Referans Üzerinden Verinin Değerini Değiştirme

        let mut xi = 21;
        let ref_xi = &mut xi;

        *ref_xi = 200;

        println!("{} : xi'nin referansı üzerinden değişmiş değeri", xi);


    }


    {
        // Char Type

        let myChar: char = '😂';
        println!("{}",myChar);

        // Her bir char bellekte 4 bayt yer kaplar.
        // '' ile tanımlanır.
    }

    {
        // float

        let my_float: f64 = 21.10;

        println!("{}",my_float);
    }

    {
        // Tuple 

        // Farklı veri tiplerini depolamaya yarar.

        let tup:(i32,f64,char,u16,&str, String) = (21,21.10,'😘',2110,"abc", new_str.clone());

        // Eğer bir değerin sahipliği taşınmışsa ve sonra orijinal sahibi tarafından kullanılmaya çalışılırsa, derleyici bu durumu sahiplik kuralı ihlali olarak algılar ve hata verir.

        println!("{}",tup.5);

    }


    {
        let my_int:i32 = 2110;
        let my_int_ref = &my_int;
        let tup_2:(&i32, f64, String) = (my_int_ref, 21.10, new_str); // Burdaki tuple içinde new_str kullanmaya kalktığımızda; new_str, yukaridaki ( 144. satır ) tup tuplenın içinde bulunduğu kod bloğundan dışarı çıktığı için sahipliği serbest bırakıldı ve bellek temizlendi. 
        // tup_2'de new_str'nin yeni sahibi olmak isteyecektir. Yani new_str sahipliği taşınacaktır MOVE EDİLECEKTİR.
        // Ama sorun şurda new_str artık YAŞAMIYOR!!!
        // Çünkü 150. kod satırından itibaren rust Drop fonksiyonunu çağırdı ve belleği temizledi.
        // Yukarıdaki satırda da bahsedildiği üzere new_str; tup onun yeni sahibi olduğu için tup'un tanımlandığı kod blokları içerisinde yaşar ve serbest bırakılır.

        // Eğer bir değerin sahipliği taşınmışsa ve sonra orijinal sahibi tarafından kullanılmaya çalışılırsa, derleyici bu durumu sahiplik kuralı ihlali olarak algılar ve hata verir.

        println!("{}", tup_2.2);
    }

    // Main() scope içinde ******

    let my_str_2: String = String::from("_clonned_str");
    // ----------------------------------------

    {
        let tup_3:(String, f64, bool) = (my_str_2.clone(), 21.10, true);
        println!("{}", tup_3.0);
    }

    {
        let tup_4 = (my_str_2, "this string was clonned because every variables has one owner", 21);
        println!("{}",tup_4.0);
    }

    {
        // ARRAY TYPE

        let my_arr = [1234];
        let my_str_arr:[&str; 1] = ["abcdf"];

        println!("{}",my_str_arr[0]);
    }


}
