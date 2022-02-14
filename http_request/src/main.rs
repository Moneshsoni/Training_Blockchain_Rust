extern crate reqwest;
fn main() {
    match reqwest::get("https://www.javatpoint.com/rust-tutorial"){
        Ok(mut responce)=>{
            //Check if status 200 or not
            if responce.status()== reqwest::StatusCode::Ok{
                match responce.text(){
                    Ok(text)=> println!("Response text: {}",text),
                    Err(_)=>println!("Could not read response code"),
                }
            }else{
                println!("Response was not 200");
                
            }
        }
        Err(_)=>println!("Could not make the request"),
    }
}
