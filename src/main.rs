
use std::string::String;
use sqlite3;
use sqlite3::State;
use std::io;


fn main() {

    
let connection = sqlite3::open("database.db").unwrap();// Veritabani baglantisi yapilir

let mut k_adi:String="".to_string();//String olarak bir sutun 
let mut yas:String="".to_string();//String olarak bir sutun 


//Kullanici adi alinir
println!("Kullanici adini giriniz : ");
 io::stdin().read_line(&mut k_adi).expect("Failed To read Input");//String olarak okunur
 k_adi.pop();//Satirin sonundaki bosluk kaldırılır

//Yas alinir
println!("Yasinizi giriniz : ");
io::stdin().read_line(&mut yas).expect("Hatali giris");
yas.pop();


connection.execute("CREATE TABLE IF NOT EXISTS kullanici (ad TEXT, yas INTEGER);",).unwrap();//Eger veritabani ve tablo yoksa olustur 

//Tabloya veri ekleme kodumuz
let mut seti = connection
    .prepare("INSERT INTO kullanici (ad, yas) VALUES (?, ?);")
    .unwrap();


//Girilen verileri SQL kodunda birlestirmek
seti.bind(1, &*k_adi).unwrap();
seti.bind(2,&*yas).unwrap();
//SQL kodunun işletilmesi
seti.next().unwrap();


    
//Veritabani iceriginin listelenmesi
    let mut statement = connection
    .prepare("SELECT * FROM kullanici ")
    .unwrap();



//Where sorgusunda kullanmak üzere alternatif kod 
//statement.bind(1, 50).unwrap();


//Verilerin ekrana basilmasi
while let State::Row = statement.next().unwrap() {
    println!("Ad = {}", statement.read::<String>(0).unwrap());
    println!("Yas = {}", statement.read::<i64>(1).unwrap());
}
}