use argh::FromArgs;
use image::{open, DynamicImage, ImageError};
use image::{RgbImage, Rgb};
use rand::Rng; // Pour générer des nombres aléatoires

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

const PALETTE: [Rgb<u8>; 0] = [
    
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

// Question 9
fn color_distance(c1: Rgb<u8>, c2: Rgb<u8>) -> f32 {
    let r_diff = c1[0] as f32 - c2[0] as f32;
    let g_diff = c1[1] as f32 - c2[1] as f32;
    let b_diff = c1[2] as f32 - c2[2] as f32;

    // Calcul de la distance euclidienne
    ((r_diff.powi(2) + g_diff.powi(2) + b_diff.powi(2)).sqrt())
}
// ---------------------------------------------------------

// Question 11
fn to_palette(image: &mut RgbImage, palette: &[Rgb<u8>]) {
    // Si la palette est vide, on ne fait rien
    if palette.is_empty() {
        println!("La palette est vide. Aucun traitement n'est appliqué.");
        return;
    }

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
// ---------------------------------------------------------

// Question 12
fn random_dithering(image: &mut RgbImage) {
    let mut rng = rand::thread_rng(); // Générateur de nombres aléatoires

    for y in 0..image.height() {
        for x in 0..image.width() {
            let pixel = image.get_pixel(x, y);
            let luminosity = luminosity_of_pixel(*pixel);

            // Générer un seuil aléatoire entre 0 et 255
            let random_threshold = rng.gen_range(0.0..255.0);

            // Déterminer la nouvelle couleur en fonction de la luminosité et du seuil
            if luminosity > random_threshold {
                image.put_pixel(x, y, WHITE);
            } else {
                image.put_pixel(x, y, BLACK);
            }
        }
    }
}
// ---------------------------------------------------------

// Question 13
fn generate_bayer_matrix(order: usize) -> Vec<Vec<u32>> {
    if order == 0 {
        // Cas de base : B0 = [[0]]
        return vec![vec![0]];
    }

    // Matrice de l'ordre précédent
    let prev_matrix = generate_bayer_matrix(order - 1);
    let size = prev_matrix.len(); // Taille de la matrice précédente (2^n)
    let new_size = size * 2; // Nouvelle taille (2^(n+1))

    // Matrice Un (remplie de 1)
    let un_matrix = vec![vec![1; size]; size];

    // Initialisation de la nouvelle matrice de Bayer
    let mut matrix = vec![vec![0; new_size]; new_size];

    // Construction des 4 quadrants selon la formule :
    // 1/4 * (4*Bn               4*Bn + 2*Un
    //        4*Bn + 3*Un        4*Bn + Un)
    for i in 0..size {
        for j in 0..size {
            let bn = prev_matrix[i][j];
            let un = un_matrix[i][j];
            matrix[i][j] = 4 * bn;                     // 4*Bn
            matrix[i][j + size] = 4 * bn + 2 * un;     // 4*Bn + 2*Un
            matrix[i + size][j] = 4 * bn + 3 * un;     // 4*Bn + 3*Un
            matrix[i + size][j + size] = 4 * bn + un;  // 4*Bn + Un
        }
    }
    matrix
}
// ---------------------------------------------------------

fn main() -> Result<(), ImageError>{
    // let args: DitherArgs = argh::from_env();

    // let path_in = args.input;
    // let path_out = args.output.unwrap_or("./img/IUT_OUT.png".to_string());

    // // Ouvrir l'image
    // let mut img: DynamicImage = open(path_in)?;

    // let mut rgb_image = img.to_rgb8();

    // let mut pixelblanc = false;

    // // Afficher dans le terminal la couleur du pixel (32, 52) de l’image
    // let pixel = rgb_image.get_pixel(32, 52);
    // println!("Pixel (32, 52) : {:?}", pixel);

    // // Calculer la luminosité du pixel
    // let luminosity = luminosity_of_pixel(*pixel);
    // println!("La luminosité du pixel (100, 100) est : {}", luminosity);

    // // Appliquer le tramage aléatoire
    // random_dithering(&mut rgb_image);

    // // Appliquer le traitement de distance entre deux couleurs
    // let distance = color_distance(BLACK, BLACK);
    // println!("La distance entre rouge et bleu est : {}", distance);

    // rgb_image.save(&path_out).unwrap();

    // Calculer B3
    let b3 = generate_bayer_matrix(3);

    // Affichage de la matrice B3
    println!("Matrice B3 :");
    for row in b3 {
        for value in row {
            print!("{:>3} ", value);
        }
        println!();
    }
    
    //
    Ok(())
}
