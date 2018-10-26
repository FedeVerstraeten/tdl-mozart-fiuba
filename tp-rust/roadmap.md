---
title: TDL - Algunas ideas para RUST
author: Facultad de Ingeniería - UBA
geometry: left=2.5cm,right=2.5cm,margin=2cm,headheight=36pt
date: 07 de Octubre de 2018
fontsize: 12pt
rsize: "a4"
header-includes:
    - \usepackage{fancyhdr}
    - \pagestyle{fancy}
    - \usepackage{amsfonts, amsmath, amsthm, amssymb}
    - \usepackage{graphicx}
    - \usepackage{xcolor}
    - \usepackage{siunitx}
---

RUST: "safe, concurrent, practical language"
============================================

A continuación se enumeran algunos conceptos que se podrán tener en cuenta como punto de partida para encarar la investigación del lenguaje de programación RUST para la realización de la presentación el día **Lunes 12/11/2018**.

* Uso de memoria. Eficiente? Seguro?
* Creadores, historia, comunidad.
* Evolución: predecesores e inspiraciones.
* Paradigma.
* Plataformas.
* Lenguaje compilado. Compilador `rustc`.
* Versiones de RUST.
* ¿Posee tipado de variables?
* Realizar comparativas con otros lenguajes similares, como C/C++ (syntax, performance, características, etc.).
* Punteros nulos (null) o colgantes (dangling) no están permitidos.
* Propósito del lenguaje. ¿Para qué sirve, donde se aplica?
* Lenguaje Open Source.
* **Empresas** que lo hayan aplicado (manera satisfactoria o no) y lo utilicen en sus desarrollos.
* **Benchmark**. ¿Contra qué o quién comparamos?
* **Clases:** son soportados usando la estructura `traits`. ¿Soporta clases o estructurado adaptado?
* ¿Soporta Herencia y Polimorfismo?
* **¡Concurrencia!** ¿Es rápida?
* ¿Es un lenguaje basado en actores?
* ¿Imperativo, declarativo o ambos?
* Rust no usa garbarge colector.
* **Syntax Rust:** `match`, `fn`, `let`,`mul`, etc.
* Variables mutadas (`mut`).
* Las Macros son estructurales en lugar de textuales como en C.
* Rust, hasta la versión 0.4, además de los tipos de datos estático convencionales (int, char, etc.) incluye `typestates`. Si bien fueron retirados, su funcionalidad aún está presente en el leguaje.
* Desde la versión 0.2 hasta la 0.4 estuvo implementada las `classes`. A partir de 0.4 se añadieron los `traits` como medio de herencia y polimorfirsmo, eliminando las clases como característica independiente.
* **Servo:** proyecto de desarrollo de un motor de renderizador para browser (Firefox).
* ¿Qué otros proyectos usan Rust actualmente?
* Los `traits` se inspiraron en el lenguaje Haskell.
* **Inferencia de tipos:** variables declaradas con `let`, no requieren ser inicializadas con un valor asignado para determinar el tipo. Se resuelve en tiempo de compilación.
* Rust a través de los traits resuelve el "Problema del diamante". Problema en OOP qde ambigüedad en herencia múltiple.
* Better memory safety will main training performance.
* Rust ganó el premio al "Lenguaje más amado" en el *Stack Overflow Dev. Survey* en 2016, 2017 y 2018 (Confirmar).
* **Pattern Matching:** usando la palabra reservada `match`.
* Casi todo en Rust es una *expression*, incluso los operadores de control de flujo.
* A pesar de poseer syntax similar a C/C++, el funcionamiento es más parecido a un lenguaje funcional como Haskell.

KEYWORDS
========
- type-afety
- zero-cost abstractions
- no null
- Safe concurrency (no data races)
- Ownership/borrowing method
- Concurrency
- Safe memory
- large-system integrity
- memory layout
- ML family
- null and dangling pointers
- data races
- option type
- pointer: some, None
- lifetime
- compiler: borrow checker
- RAII (garbage collector)
- references (using &)
- ownership variables
