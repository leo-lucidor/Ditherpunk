# Ditherpunk


## Partie - 1 

### Question 2 - Pour ouvrir une image depuis un fichier, on utilise … On obtient un DynamicImage, à quoi correspond ce type?


- Le type DynamicImage est une structure qui peut contenir différentes - représentations d'une image en fonction de son format (par exemple, RGB, RGBA, etc.).

- Pour obtenir une image en mode rgb8, il faut utiliser la méthode .to_rgb8() qui convertit l'image en une image avec 3 canaux (R, G, B), chacun étant un u8.

Pour utiliser le mode seuil avec une image d'entrée et une sortie spécifiée, il faut exécuter cette commande :

```rs
cargo run -- img/IUT.jpg img/IUT_OUT.png seuil
```

### Question 3 - Sauver l’image obtenue au format png. Que se passe-t-il si l’image de départ avait un canal alpha?

- Si l'image d'entrée a un canal "alpha", une erreur survien lors du traitement : **Error: Decoding(DecodingError { format: Exact(Png), underlying: Some(Format(FormatError { inner: InvalidSignature })) })**

- Cette erreur se déclenche à cause de ce canal "alpha" au moment d'executer la **fonction "to_rgb8()" lors du traitement**. On peut l'expliquer par le fait que cette fonction "to_rgb8()" essaie de convertir une image RGBA, en RGB8. Le RGB8 ne contenant pas de canal alpha, le format de lecture est en erreur.

### Question 4 - Afficher dans le terminal la couleur du pixel (32, 52) de l’image de votre choix

- Pour afficher la couleur du pixel (32, 52) d'une image. Il nous faut utiliser la fonction "get_pixel()" avec les coordonnées "32, 52".
- Il nous faut ensuite exectuter "println!("Pixel (32, 52) : {:?}", pixel);" pour afficher la couleur en RGB du Pixel.

### Question 5 - Passer un pixel sur deux d’une image en blanc. Est-ce que l’image obtenue est reconnaissable?

# Passer un pixel sur deux d’une image en blanc : Est-ce que l’image obtenue est reconnaissable ?

Lorsqu’on passe un pixel sur deux d’une image en blanc, l’image résultante reste souvent reconnaissable, bien que son apparence soit altérée. La perception humaine est particulièrement douée pour interpréter des motifs et reconstruire des formes même lorsqu’une partie de l’information visuelle est absente ou modifiée.

Cependant, plusieurs facteurs influencent la reconnaissance :

# Passer un pixel sur deux d’une image en blanc : Est-ce que l’image obtenue est reconnaissable ?

- Pour modifier l'image, j'ai utilisé un simple algorithme qui parcourt chaque pixel et passe un pixel sur deux en blanc (`rgb(255, 255, 255)`). Voici le code utilisé :

```rust
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
```

- Grâce à la haute résolution de l'image utilisé, l'image obtenue reste bien reconnaissable malgré cette transformation.