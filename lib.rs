use candid::{CandidType , Deserialize}; //import ui
use std::include_bytes; //import bytes

mod core; // module olarak 
// ! macro yapisi demek
const IMAGE_SIZE_IN_PIXELS: usize = 1024;
const LOGO_TRANSPARENT: &[U8] = include_bytes!("../assets/logotransparent.png");
const LOGO_WHITE: &[u8] = include_bytes!("../assets/logo_white.png");

#[derive(CandidType ,Deserialize)] //napiyo fikrim yok inherit galiba? 
struct Options {

    add_logo: bool ,
    add_gradient: bool , 
    add_transparency: Option<bool>,

}
#[derive(CandidType ,Deserialize)] //napiyo fikrim yok inherit galiba? 

struct QrError{

    message: String, 
    

}
#[derive(CandidType ,Deserialize)] //napiyo fikrim yok inherit galiba? 

enum QrResult {
    Image(Vec<u8>) ,
    Err(QrError) , 
}

fn qrcode_impl(input: String , options: Options) -> QrResult{

    let logo = if options.add_transparency  == Some(true){
        LOGO_TRANSPARENT
    }
    else{
        LOGO_WHITE
    };

    let result = match core::generate(input, options, logo , IMAGE_SIZE_IN_PIXELS){

        Ok(blob) => QrResult::Image(blob) , 
        Err(err) => QrResult::Err(QrError{
            message: err.to_string(),
        }),
    };
    ic_cdk::println!("Executed Instructions: {}" , ic_cdk::api::performance_counter(0));
    result
} 
#[ic_cdk::update]
fn qrcode_query(input: String, options: Options) -> QrResult{
    qrcode_impl(input, options)
}
#[ic_cdk::query]
fn qrcode_query(input: String, options: Options) -> QrResult{
    qrcode_impl(input, options)
}
