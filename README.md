# tdl-mozart-fiuba
 [75.31] Teoría del lenguaje - FIUBA 

## Links
* Slack: [tdl-fiuba.slack](https://tdl-fiuba.slack.com)
* GoogleSite: [fiuba7531](https://sites.google.com/site/fiuba7531/)
* Why Linux32?: [Linux32](https://gerardnico.com/linux/linux32)

## Installing EMACS
Previamente se debe tener instalado EMACS.

### Linux(Ubuntu)
```
sudo apt-get install emacs
```

## Installing MOZART

### Linux (Ubuntu)

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
* Seguir las instrucciones de [EADME.MacOS](https://github.com/mozart/mozart2/blob/master/README.MacOS.md), basicamente:

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
