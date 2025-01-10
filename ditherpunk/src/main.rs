use argh::FromArgs;
use image::{open, DynamicImage, ImageError};
use image::{RgbImage, Rgb};

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

const PALETTE: [Rgb<u8>; 8] = [
    BLACK, WHITE, BLUE, RED, GREEN, YELLOW, MAGENTA, CYAN
];

fn luminosity_of_pixel(pixel: Rgb<u8>) -> f32 {
    // Extraire les canaux R, G, B du pixel
    let (r, g, b) = (pixel[0], pixel[1], pixel[2]);
    // Calcul de la luminosité en utilisant la formule pondérée
    0.299 * r as f32 + 0.587 * g as f32 + 0.114 * b as f32
}

fn to_monochrome(image: &mut RgbImage) {
    for y in 0..image.height() {
        for x in 0..image.width() {
            let pixel = image.get_pixel(x, y);
            let luminosity = luminosity_of_pixel(*pixel);

            // Remplacement par blanc ou noir en fonction de la luminosité
            if luminosity > 127.5 {
                image.put_pixel(x, y, WHITE);
            } else {
                image.put_pixel(x, y, BLACK);
            }
        }
    }
}

fn to_pair_colors(image: &mut RgbImage, color_low: Rgb<u8>, color_high: Rgb<u8>) {
    for y in 0..image.height() {
        for x in 0..image.width() {
            let pixel = image.get_pixel(x, y);
            let luminosity = luminosity_of_pixel(*pixel);

            // Remplacement par `color_high` ou `color_low` en fonction de la luminosité
            if luminosity > 127.5 {
                image.put_pixel(x, y, color_high);
            } else {
                image.put_pixel(x, y, color_low);
            }
        }
    }
}

fn color_distance(c1: Rgb<u8>, c2: Rgb<u8>) -> f32 {
    let r_diff = c1[0] as f32 - c2[0] as f32;
    let g_diff = c1[1] as f32 - c2[1] as f32;
    let b_diff = c1[2] as f32 - c2[2] as f32;

    // Calcul de la distance euclidienne
    ((r_diff.powi(2) + g_diff.powi(2) + b_diff.powi(2)).sqrt())
}

fn to_palette(image: &mut RgbImage, palette: &[Rgb<u8>]) {
    for y in 0..image.height() {
        for x in 0..image.width() {
            let pixel = image.get_pixel(x, y);
            let closest_color = find_closest_color(*pixel, palette);
            image.put_pixel(x, y, closest_color);
        }
    }
}

fn find_closest_color(pixel: Rgb<u8>, palette: &[Rgb<u8>]) -> Rgb<u8> {
    let mut min_distance = f32::MAX;
    let mut closest_color = palette[0];

    for &color in palette {
        let distance = color_distance(pixel, color);
        if distance < min_distance {
            min_distance = distance;
            closest_color = color;
        }
    }

    closest_color
}

fn main() -> Result<(), ImageError>{
    let args: DitherArgs = argh::from_env();

    let path_in = args.input;
    let path_out = args.output.unwrap_or("./img/IUT_OUT.png".to_string());

    // Ouvrir l'image
    let mut img: DynamicImage = open(path_in)?;

    let mut rgb_image = img.to_rgb8();

    let mut pixelblanc = false;

    // Afficher dans le terminal la couleur du pixel (32, 52) de l’image
    let pixel = rgb_image.get_pixel(32, 52);
    println!("Pixel (32, 52) : {:?}", pixel);

    // Calculer la luminosité du pixel
    let luminosity = luminosity_of_pixel(*pixel);
    println!("La luminosité du pixel (100, 100) est : {}", luminosity);

    // Convertir l'image à la palette
    to_palette(&mut rgb_image, &PALETTE);

    // Appliquer le traitement de distance entre deux couleurs
    let distance = color_distance(BLACK, BLACK);
    println!("La distance entre rouge et bleu est : {}", distance);

    rgb_image.save(&path_out).unwrap();
    
    //
    Ok(())
}
