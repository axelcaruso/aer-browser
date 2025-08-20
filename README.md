# Aer Browser

[![Build Status](https://img.shields.io/badge/build-passing-brightgreen)](https://github.com/axelcaruso/aer-browser/actions)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![PRs Welcome](https://img.shields.io/badge/PRs-welcome-brightgreen.svg)](CONTRIBUTING.md)

**Aer es un navegador web moderno, de alto rendimiento y de código abierto, construido desde cero y potenciado por el motor de renderizado Orion.**

---

## Nuestros Objetivos

Los navegadores modernos se han vuelto innecesariamente complejos y pesados. **Aer** es nuestra respuesta: un navegador que se enfoca en lo esencial.

*   **Rendimiento Extremo:** Cada línea de código está optimizada para la máxima velocidad. Sin telemetría, sin bloatware. Solo la web, renderizada de la forma más rápida posible.
*   **Simplicidad Radical:** Una interfaz limpia y un núcleo sencillo. Nos centramos en implementar los estándares web de forma correcta y eficiente, dejando fuera todo lo que no sea esencial para la navegación.
*   **Seguridad por Diseño:** Construido en Rust, aprovechamos la seguridad de memoria del lenguaje para eliminar categorías enteras de vulnerabilidades desde la raíz del proyecto.
*   **Totalmente Open Source:** Creemos en el poder de la comunidad. Aer y Orion son y siempre serán software libre, bajo la licencia MIT.

## Estado del Proyecto

**ALTAMENTE EXPERIMENTAL.** Aer se encuentra en una fase muy temprana de desarrollo. No es, ni de lejos, un navegador para el uso diario. Estamos construyendo los cimientos. ¡Cualquier ayuda es bienvenida!

---

## Componentes

Este repositorio contiene los dos pilares del proyecto:

*   📁 `/aer`: La aplicación principal del navegador. Contiene la interfaz de usuario (UI), la gestión de pestañas y toda la lógica de la aplicación.
*   📁 `/orion`: El corazón del proyecto. Nuestro motor de renderizado, **Orion**, responsable de parsear HTML y CSS, construir el DOM, y renderizar el contenido en pantalla.

## Construir desde el código fuente

**Requisitos:**

*   Rust y Cargo (última versión estable).
*   Dependencias de sistema requeridas por el ecosistema de Rust para tu plataforma.

**Pasos:**

1.  **Clonar el repositorio:**
    ```bash
    git clone https://github.com/axelcaruso/aer-browser.git
    cd aer-browser
    ```

2.  **Construir el proyecto en modo release (optimizado):**
    ```bash
    cargo build --release
    ```

3.  **Ejecutar Aer:**
    ```bash
    ./target/release/aer
    ```

## Contribuciones

Las contribuciones son el combustible de este proyecto. Si quieres escribir código, reportar un bug, proponer una mejora o arreglar la documentación, eres bienvenido.

Por favor, lee nuestra [**Guía de Contribución**](CONTRIBUTING.md) para empezar.

## Licencia

Este proyecto está distribuido bajo la **Licencia MIT**.

---
