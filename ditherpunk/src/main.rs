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

// Question 15
fn bayer_dithering(image: &mut RgbImage, bayer_matrix: &[Vec<u32>]) {
    let matrix_size = bayer_matrix.len() as u32;

    for y in 0..image.height() {
        for x in 0..image.width() {
            let pixel = image.get_pixel(x, y);
            let luminosity = luminosity_of_pixel(*pixel);

            // Récupérer le seuil de la matrice (en répétant la matrice)
            let threshold = bayer_matrix[(y % matrix_size) as usize][(x % matrix_size) as usize] as f32;

            // Appliquer le seuil (normalisé à 255)
            if luminosity > (threshold / (matrix_size * matrix_size) as f32) * 255.0 {
                image.put_pixel(x, y, WHITE);
            } else {
                image.put_pixel(x, y, BLACK);
            }
        }
    }
}
// ---------------------------------------------------------

// Question 16
fn error_diffusion(image: &mut RgbImage) {
    let width = image.width() as i32;
    let height = image.height() as i32;

    // Convertir l'image en niveaux de gris
    let mut grayscale_image: Vec<Vec<f32>> = vec![vec![0.0; width as usize]; height as usize];
    for y in 0..height {
        for x in 0..width {
            let pixel = image.get_pixel(x as u32, y as u32);
            grayscale_image[y as usize][x as usize] = luminosity_of_pixel(*pixel) / 255.0; // Normaliser à [0, 1]
        }
    }

    for y in 0..height {
        for x in 0..width {
            // Récupérer la valeur actuelle
            let old_value = grayscale_image[y as usize][x as usize];

            // Quantification : remplace par noir (0.0) ou blanc (1.0)
            let new_value = if old_value > 0.5 { 1.0 } else { 0.0 };

            // Appliquer la nouvelle valeur au pixel
            let color = if new_value == 1.0 { WHITE } else { BLACK };
            image.put_pixel(x as u32, y as u32, color);

            // Calculer l'erreur
            let error = old_value - new_value;

            // Diffuser l'erreur aux pixels voisins
            if x + 1 < width {
                grayscale_image[y as usize][(x + 1) as usize] += error * 0.5; // Pixel à droite
            }
            if y + 1 < height {
                grayscale_image[(y + 1) as usize][x as usize] += error * 0.5; // Pixel en dessous
            }
        }
    }
}
// ---------------------------------------------------------

// Question 18 
fn error_diffusion_palette(image: &mut RgbImage, palette: &[Rgb<u8>]) {
    let width = image.width() as i32;
    let height = image.height() as i32;

    // Parcours des pixels
    for y in 0..height {
        for x in 0..width {
            let pixel = image.get_pixel(x as u32, y as u32);
            let original_color = [
                pixel[0] as f32,
                pixel[1] as f32,
                pixel[2] as f32,
            ];

            // Trouver la couleur la plus proche dans la palette
            let closest_color = find_closest_color(original_color, palette);

            // Appliquer la couleur la plus proche au pixel
            image.put_pixel(x as u32, y as u32, closest_color);

            // Calculer l'erreur (différence entre l'original et la couleur choisie)
            let error = [
                original_color[0] - closest_color[0] as f32,
                original_color[1] - closest_color[1] as f32,
                original_color[2] - closest_color[2] as f32,
            ];

            // Diffuser l'erreur aux pixels voisins
            if x + 1 < width {
                distribute_error(image, x + 1, y, error, [0.5, 0.5, 0.5]);
            }
            if y + 1 < height {
                distribute_error(image, x, y + 1, error, [0.5, 0.5, 0.5]);
            }
        }
    }
}

// Fonction pour trouver la couleur la plus proche dans la palette
fn find_closest_color(original_color: [f32; 3], palette: &[Rgb<u8>]) -> Rgb<u8> {
    palette
        .iter()
        .min_by_key(|color| {
            let distance = (original_color[0] - color[0] as f32).powi(2)
                + (original_color[1] - color[1] as f32).powi(2)
                + (original_color[2] - color[2] as f32).powi(2);
            (distance * 1000.0) as u32
        })
        .unwrap()
        .to_owned()
}

// Fonction pour distribuer l'erreur à un pixel voisin
fn distribute_error(image: &mut RgbImage, x: i32, y: i32, error: [f32; 3], coefficients: [f32; 3]) {
    let pixel = image.get_pixel_mut(x as u32, y as u32);

    for i in 0..3 {
        let new_value = (pixel[i] as f32 + error[i] * coefficients[i]).clamp(0.0, 255.0);
        pixel[i] = new_value as u8;
    }
}
// ---------------------------------------------------------

fn main() -> Result<(), ImageError> {
    let args: DitherArgs = argh::from_env();

    let path_in = args.input;
    let path_out = args.output.unwrap_or("./img/IUT_OUT.png".to_string());

    // Ouvrir l'image
    let mut img: DynamicImage = open(path_in)?;
    let mut rgb_image = img.to_rgb8();

    // Définir une palette de couleurs (exemple avec 8 couleurs)
    let palette = vec![
        Rgb([0, 0, 0]),    // Noir
        Rgb([255, 255, 255]), // Blanc
        Rgb([255, 0, 0]),  // Rouge
        Rgb([0, 255, 0]),  // Vert
        Rgb([0, 0, 255]),  // Bleu
        Rgb([255, 255, 0]), // Jaune
        Rgb([0, 255, 255]), // Cyan
        Rgb([255, 0, 255]), // Magenta
    ];

    // Appliquer la diffusion d'erreur avec la palette définie
    error_diffusion_palette(&mut rgb_image, &palette);

    // Sauvegarder l'image résultante
    rgb_image.save(&path_out).unwrap();

    println!("Image palettisée avec diffusion d'erreur sauvegardée dans {}", path_out);

    Ok(())
}

