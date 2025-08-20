# Guía de Contribución

¡Bienvenido! Nos emociona que consideres unirte a la comunidad de **Aer** y **Orion**. Tu interés en contribuir es un paso fundamental para ayudarnos a construir un proyecto robusto y exitoso. Esta guía te proporcionará toda la información necesaria para empezar.

---

## Código de Conducta

Nuestra comunidad se basa en el respeto mutuo. La única regla es: **sé respetuoso y profesional.**

Somos un equipo que trabaja hacia un objetivo común. Respeta a los demás, debate las ideas y no a las personas. Si surge un problema, comunícalo de manera constructiva. Nos reservamos el derecho de eliminar cualquier contribución o acceso a la comunidad si no se sigue este principio. Para más detalles, consulta nuestro `CODE_OF_CONDUCT.md`.

---

## Cómo Contribuir

### Reportando Errores (Bugs)

Detectar y reportar errores es una contribución invaluable. Antes de abrir un *issue*, sigue estos pasos:

1.  **Busca primero:** Asegúrate de que no haya un *issue* ya abierto para el mismo problema.
2.  **Sé descriptivo:** Proporciona todo el contexto necesario para que podamos entender y reproducir el error. Incluye:
    * Los pasos exactos para reproducir el problema.
    * Lo que esperabas que sucediera.
    * Lo que realmente sucedió.
    * Logs, capturas de pantalla y cualquier otra información relevante que nos ayude a diagnosticar el problema sin ambigüedades.

### Proponiendo Mejoras

Si tienes una idea para mejorar el proyecto, ¡nos encantaría escucharla! Recuerda que la misión de **Aer** y **Orion** es mantener la **velocidad y la simplicidad radical**.

1.  Abre un nuevo *issue*.
2.  Describe tu propuesta con claridad. Explica qué problema resuelve y por qué es esencial para el proyecto.
3.  No te desanimes si tu idea es rechazada. Somos muy cuidadosos al defender la integridad del núcleo del proyecto para evitar que se vuelva demasiado complejo o lento.

---

## Envío de *Pull Requests*

Aquí es donde tu código se une al nuestro. Sigue estos pasos para asegurarte de que tu contribución pueda ser revisada y fusionada de manera eficiente.

1.  **Haz un *Fork***: Crea una copia del repositorio en tu cuenta de GitHub.
2.  **Crea una Rama:** No trabajes directamente en la rama `main`. Crea una rama con un nombre descriptivo:
    ```bash
    git checkout -b feat/nombre-de-tu-nueva-caracteristica
    ```
    O si es un error:
    ```bash
    git checkout -b fix/arreglo-del-error-encontrado
    ```
3.  **Desarrollo del Código:**
    * **Mantén el estilo:** Utilizamos `rustfmt` para el formato del código. Antes de confirmar tus cambios, ejecuta `cargo fmt` para asegurar la coherencia del estilo.
    * **Escribe *Tests***: Todas las nuevas funcionalidades deben incluir pruebas unitarias. Si estás corrigiendo un error, añade una prueba que falle antes del arreglo y pase después. Las contribuciones sin pruebas no serán consideradas.
    * **Documenta el código:** Añade comentarios si el código es complejo. Si modificas una API, actualiza la documentación correspondiente.
4.  **Confirmaciones (Commits) Atómicas:** Realiza *commits* pequeños y con mensajes claros que expliquen QUÉ se hizo y POR QUÉ. Sigue el [Formato de *Commits* Convencionales](https://www.conventionalcommits.org/es/v1.0.0/).
    ```bash
    # Ejemplo de un buen commit
    git commit -m "feat(parser): Agregar soporte inicial para etiquetas <h1>"

    # Ejemplo de un mal commit
    git commit -m "cambios"
    ```
5.  **Abre el *Pull Request***:
    * Asegúrate de que tu rama está actualizada con la rama `main` del repositorio original.
    * Envía tu *Pull Request* a nuestra rama `main`.
    * Rellena la plantilla del PR, explicando los cambios y, si aplica, enlaza el *issue* que resuelve.
6.  **Espera la Revisión:** Revisaremos tu código. Es posible que solicitemos cambios para asegurar la calidad y coherencia del proyecto. Este proceso es colaborativo y busca la mejora continua del código.

---

Gracias por tu tiempo y dedicación. ¡Esperamos tus contribuciones!