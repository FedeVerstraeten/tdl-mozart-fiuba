# tdl-mozart-fiuba
 [75.31] Teoría del lenguaje - FIUBA 

## Links
* Slack: [tdl-fiuba.slack](https://tdl-fiuba.slack.com)
* GoogleSite: [fiuba7531](https://sites.google.com/site/fiuba7531/)
* Why Linux32?: [Linux32](https://gerardnico.com/linux/linux32)
* Simulador Maq. Abstracta Oz: [Kozily](https://kozily.surge.sh/)

## TP - Rust

Material para la preparación del trabajo práctico grupal sobre Rust

* Leer de la [documentación oficial](https://doc.rust-lang.org/book/second-edition/foreword.html) los capítulos interesantes: 1, 2, 3, 4 y 7.
* Temas interesantes de rust: type-safety, zero-cost abstractions, no null, Safe concurrency (no data races), Ownership/borrowing method.
* [Paradigms of Rust for the Go developer](https://medium.com/@deckarep/paradigms-of-rust-for-the-go-developer-210f67cd6a29)
* [Abstraction without overhead: traits in Rust](https://blog.rust-lang.org/2015/05/11/traits.html)
* [From python to go to rust](http://tech.allo-media.net/point/of/view/2018/03/22/from-python-to-go-to-rust.html)
* [Rust Concurrency Explained](https://www.youtube.com/watch?v=Dbytx0ivH7Q)
* [Rust Tutorial: Learn How To Be Productive In Rust](https://www.youtube.com/watch?v=waC2wgknY0s)
* [Rust Playground](https://play.rust-lang.org/) - Website for run examples
* [Into_rust()](http://intorust.com/) - Screencast for learning Rust
* [Rust cookbook](https://rust-lang-nursery.github.io/rust-cookbook/)


## Installing EMACS
Previamente se debe tener instalado EMACS.

### Linux (Ubuntu)
```
sudo apt-get install emacs
```

## Installing MOZART

### Linux (Ubuntu)
Probado en Ubuntu 14.04 LTS de 64bits.

Instalación Mozart 1.4 descargando desde repositorio en **SourceForge**
```
wget -O /tmp/mozart-1.4.0.tar.gz https://sourceforge.net/projects/mozart-oz/files/v1/1.4.0-2008-07-02-tar/mozart-1.4.0.20080704-linux-i486.tar.gz/download
cd /usr/local && sudo tar zxvf /tmp/mozart-1.4.0.tar.gz && rm /tmp/mozart-1.4.0.tar.gz
echo 'PATH=$PATH:/usr/local/mozart/bin' >> ~/.profile
. ~/.profile
```

### MACOSX
Probar si funciona.

* Instalar `homebrew`
* Instalar [emacsformacosx](https://emacsformacosx.com/)
* Seguir las instrucciones de [README.MacOS](https://github.com/mozart/mozart2/blob/master/README.MacOS.md), basicamente:

```
brew tap dskecse/tap
brew cask install mozart2
```

### WINDOWS
Completar. ¡Adelante mis valientes!

## Execution MOZART

### Linux (Ubuntu)

Desde la consola, ejecutar con compatibilidad *Linux32*.

```
myUser$ linux32 oz
```
Si no se usa, no se abre la ventana *Oz Browser* y la consola devuelve el error:
```
Process Oz Emulator exited abnormally with code 127
```

Se abrirá la consola de Mozart.
Ejecutar los comandos con la combinación de teclas `Ctrl+.` + `Ctrl+b`, o bien con el cursor en la pantalla superior, desde el menú *Oz -> FeedBuffer*
