# Ditherpunk


## 1 

### question 1


- Le type DynamicImage est une structure qui peut contenir différentes - représentations d'une image en fonction de son format (par exemple, RGB, RGBA, etc.).

- Pour obtenir une image en mode rgb8, il faut utiliser la méthode .to_rgb8() qui convertit l'image en une image avec 3 canaux (R, G, B), chacun étant un u8.


### question 2

Pour utiliser le mode seuil avec une image d'entrée et une sortie spécifiée, il faut exécuter cette commande :

```rs
cargo run -- img/IUT.jpg img/IUT_OUT.png seuil
```