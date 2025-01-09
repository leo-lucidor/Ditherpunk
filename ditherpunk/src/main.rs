use argh::FromArgs;
use image::{open, DynamicImage, ImageError};

#[derive(Debug, Clone, PartialEq, FromArgs)]
/// Convertit une image en monochrome ou vers une palette réduite de couleurs.
struct DitherArgs {

    /// le fichier d’entrée
    #[argh(positional)]
    input: String,

    /// le fichier de sortie (optionnel)
    #[argh(positional)]
    output: Option<String>,

    /// le mode d’opération
    #[argh(subcommand)]
    mode: Mode
}

#[derive(Debug, Clone, PartialEq, FromArgs)]
#[argh(subcommand)]
enum Mode {
    Seuil(OptsSeuil),
    Palette(OptsPalette),
}

#[derive(Debug, Clone, PartialEq, FromArgs)]
#[argh(subcommand, name="seuil")]
/// Rendu de l’image par seuillage monochrome.
struct OptsSeuil {}


#[derive(Debug, Clone, PartialEq, FromArgs)]
#[argh(subcommand, name="palette")]
/// Rendu de l’image avec une palette contenant un nombre limité de couleurs
struct OptsPalette {

    /// le nombre de couleurs à utiliser, dans la liste [NOIR, BLANC, ROUGE, VERT, BLEU, JAUNE, CYAN, MAGENTA]
    #[argh(option)]
    n_couleurs: usize
}
 
const WHITE: image::Rgb<u8> = image::Rgb([255, 255, 255]);
const GREY: image::Rgb<u8> = image::Rgb([127, 127, 127]);
const BLACK: image::Rgb<u8> = image::Rgb([0, 0, 0]);
const BLUE: image::Rgb<u8> = image::Rgb([0, 0, 255]);
const RED: image::Rgb<u8> = image::Rgb([255, 0, 0]);
const GREEN: image::Rgb<u8> = image::Rgb([0, 255, 0]);
const YELLOW: image::Rgb<u8> = image::Rgb([255, 255, 0]);
const MAGENTA: image::Rgb<u8> = image::Rgb([255, 0, 255]);
const CYAN: image::Rgb<u8> = image::Rgb([0, 255, 255]);

fn main() -> Result<(), ImageError>{
    let args: DitherArgs = argh::from_env();

    let path_in = args.input;
    let path_out = args.output.unwrap_or("./img/IUT_OUT.png".to_string());

    // Ouvrir l'image
    let mut img: DynamicImage = open(path_in)?;

    let mut rgb_image = img.to_rgb8();

    let mut pixelblanc = false;

    for (x, y, pixel) in rgb_image.enumerate_pixels_mut() {
        if pixelblanc {
            // passer le pixel en blanc en utilisant le rgb(255, 255, 255)
            pixel.0[0] = 255;
            pixel.0[1] = 255;
            pixel.0[2] = 255;
            pixelblanc = false;
        } else {
            pixelblanc = true;
        }
    }

    // Afficher dans le terminal la couleur du pixel (32, 52) de l’image
    let pixel = rgb_image.get_pixel(32, 52);
    println!("Pixel (32, 52) : {:?}", pixel);

    rgb_image.save(&path_out).unwrap();
    
    
    //
    Ok(())
}
