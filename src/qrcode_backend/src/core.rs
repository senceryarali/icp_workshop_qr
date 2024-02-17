use image::{imageops, ImageBuffer , Rgba};
use qrcode_generator::QrCodeEcc;
use::std::io::Cursor;

use crate::Options;

pub(super) fn generate(

    input: String , 
    options: Options,
    logo: &[u8] ,
    image_size: usize ,

) -> Result<Vec<u8> , anyhow::Error> {

    let mut qr = image::DynamicImage::ImageLuma8(qrcode_generator::to_image_buffer( // mut dersek mutable olur yoksa degismez
        input,
        QrCodeEcc::Quartile,
        image_size,


    )?).into_rgba8(); //soru isareti napiyo yaw


    if options.add_transparency == Some(true) {

        make_transparent(&mut qr) ;  //rust reference barrowing

    }
    if options.add_logo {
        add_logo(&mut qr, logo);

    }
    if options.add_gradient{
        add_gradient(&mut qr);
    }

    let mut result = vec![];
    qr.write_to(&mut Cursor::new( &mut result), image::ImageOutputFormat::Png)?;
    Ok(result)



    fn make_transparent(qr: & mut ImageBuffer<Rgba<u8> , Vec<u8>> ) {
    {

        for( _x , _y , pixels) in qr.enumerate_pixels_mut(){
            if pixel.0 == [255, 255, 255, 255] {
                *pixel = image::Rgba([255,255,255,0]);
            }
        }
    }
}


    fn add_logo(qr: &mut ImageBuffer<Rgba<u8> ,  Vec<u8> > , logo: &[u8]){
        let image_size = qr.width().min(qr.height()) as usize ; 
        let element_size = get_qr_element_size(qr);
        let mut logo_size = element_size;
        while logo_size + 2 * element_size <= 5 * image_size / 16 {

            logo_size += 2 * element_size;
        }

        let mut logo = image::io::Reader::new(Cursor::new(logo)).with_gussed_format().unwrap().decode().unwrap() ; 

        logo = logo.resize(
            logo_size as u32 , 
            logo_size as u32 ,
            imageops::FilterType::Lanczos3 ,


        );

        iamgeops::replace(
            qr,
            &logo,
            ((image_size - logo_size ) / 2) as i64,
            ((image_size - logo_size ) / 2) as i64,

        );


    }


} // if pixel.o sey demek pixel sifirsa

    fn add_gradient(qr: &mut ImageBuffer<Rgba<u8> , )