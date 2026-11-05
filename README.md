# LF Botonera Radio — versión modificada (no oficial)

> Esta es una **versión modificada** de [LF Botonera de Efectos](https://github.com/yosoyluisfernando/LF-Botonera-de-efectos),
> creada por **Luis Fernando Velásquez** y publicada bajo licencia GPL-3.0. Está adaptada
> para usarla como **botonera de efectos de radio**. No es la versión oficial: si
> encuentras un problema, repórtalo en este repositorio y no en el original.

## 📥 Cómo instalarla (Windows 10/11)

1. Entra en **[Releases](https://github.com/danilo-wenner-1/LF-Botonera-de-efectos/releases/latest)**
   y descarga el archivo **`LF.Botonera.Radio_1.5.1_x64-setup.exe`**.
2. Ábrelo con doble clic.
3. Windows probablemente mostrará **«Windows protegió su PC»** (SmartScreen). Es normal:
   el instalador no tiene firma digital paga. Pulsa **Más información → Ejecutar de todas formas**.
4. Sigue el asistente y abre **LF Botonera Radio** desde el menú Inicio.

Se instala **aparte** de la versión oficial, que sigue intacta si la tienes. Guarda su
configuración en otra carpeta (`%APPDATA%\LF Botonera Radio`), así que **empieza vacía**.
Si quieres traer tus botoneras de la versión oficial, en la oficial usa *clic derecho en una
pestaña → Exportar (.bdelf)* (o *Perfil → Exportar*) y aquí *Importar*.

## ✨ Qué tiene de nuevo

| Función | Dónde está |
|---|---|
| **Salida de audio propia por pestaña** (principal, auriculares, otra placa…) | Clic derecho en la pestaña → *Editar pestaña* → «Salida de audio de esta pestaña» |
| **Buscador de efectos** en todas las pestañas | Botón 🔍 a la izquierda de las pestañas. Ejemplos: `aplausos`, `cortinas 5`, `12` |
| **Cambiar de pestaña con la rueda del mouse** | Rueda arriba/abajo con el cursor sobre la barra de pestañas |
| **Atajo de teclado por pestaña** (p. ej. Ctrl+1) | Clic derecho en la pestaña → *Asignar atajo de teclado…* |
| **Tamaño de texto por botón** (− / +) | Clic derecho en un botón → *Editar…* → «Tamaño de texto del botón» |
| **Tamaño de texto por pestaña** | *Editar pestaña* → «Tamaño de texto predeterminado» |
| **Colores HEX y degradados** de dos colores (fondo y texto), con vista previa | Clic derecho en un botón → *Editar…* |
| **Ocultar la barra inferior** para tener más espacio | ⚙️ Ajustes → Principal → «Mostrar la barra inferior» |
| **Archivos M4A** | Se cargan igual que un MP3. Los que no se pueden leer directo se convierten solos a MP3 en segundo plano |

Notas:
- Si la placa de sonido de una pestaña se desconecta, sus efectos **siguen sonando por la
  salida principal** y la app avisa. Al reconectarla vuelve sola.
- Con la barra inferior oculta, **Detener todo** sigue funcionando con su atajo de teclado
  (⚙️ Ajustes → Atajos del Teclado).
- Los colores escritos en HEX o en degradado se ven **exactos**. Los elegidos de la paleta
  clásica se siguen adaptando al tema claro/oscuro, como antes.

## 🧪 Qué conviene probar (primera versión)

Esta versión todavía **no se probó con hardware real**. Si la usas, dinos cómo te fue con:

1. Tres pestañas con tres salidas distintas, disparando a la vez, y desconectar una placa mientras suena.
2. Cambiar de pestaña con la rueda y con atajos (Ctrl+1, Ctrl+2…). Intentar repetir un atajo debe avisar del conflicto.
3. Tamaños de texto y colores: cerrar y volver a abrir la app y comprobar que todo se mantiene.
4. El buscador con efectos repartidos en varias pestañas.
5. **Archivos M4A** de distinto origen (descargados, de WhatsApp, de iPhone…) y tamaño.

**Limitaciones conocidas:** las pestañas con salida propia no aparecen en el vúmetro ni
en la Consola de audio, el Mic no las atenúa y no obedecen al volumen máster. Admite hasta
8 placas distintas a la vez. Un M4A grande recién cargado puede tardar unos segundos en
poder dispararse (aparece el aviso «se está preparando»).

## 🛠️ Compilar desde el código

Ver [COMPILAR.md](COMPILAR.md). En Windows hay que descargar antes FFmpeg con
`scripts\fetch-ffmpeg.ps1`, porque supera el tamaño que GitHub permite guardar.

## Créditos y licencia

- Programa original: **LF Botonera de Efectos**, © Luis Fernando Velásquez —
  <https://github.com/yosoyluisfernando/LF-Botonera-de-efectos>. Si te sirve, apoya al autor original.
- Esta versión modificada se distribuye bajo la misma licencia **GPL-3.0-or-later** ([LICENSE](LICENSE)).
  Los cambios están descritos en [CHANGELOG.md](CHANGELOG.md).
- Incluye **FFmpeg** (GPL v3) para convertir archivos M4A. Ver [THIRD_PARTY_NOTICES.md](THIRD_PARTY_NOTICES.md).

---

*Debajo sigue el README original del proyecto.*

---

<div align="center">
  <img src="icono_circular.png" alt="Logo de LF Botonera de Efectos" width="96" height="96">

  <h1>LF Botonera de Efectos (Soundboard)</h1>

  <p><strong>Botonera profesional de efectos de sonido para radio, locución y streaming.</strong></p>

  <p>Dispara efectos, cortinas, identificaciones, locuciones y audios de apoyo en directo, con un motor nativo en Rust y una interfaz diseñada para la velocidad y el control total.</p>

  [![Microsoft Store](https://img.shields.io/badge/Microsoft%20Store-Descargar-0078D4?logo=microsoft)](https://apps.microsoft.com/detail/9NJ8ST39QP7V)
  [![Windows](https://img.shields.io/badge/Windows-10%2F11-0078D4?logo=windows)](#descarga)
  [![Linux](https://img.shields.io/badge/Linux-DEB%20%7C%20AppImage-FCC624?logo=linux&logoColor=black)](#descarga)
  [![Licencia](https://img.shields.io/badge/Licencia-GPL--3.0--or--later-green)](LICENSE)
  [![Release](https://img.shields.io/github/v/release/yosoyluisfernando/LF-Botonera-de-efectos?label=versión)](https://github.com/yosoyluisfernando/LF-Botonera-de-efectos/releases/latest)
</div>

**Español** · [English](README.en.md) · [Português (Brasil)](README.pt-BR.md) · [Português (Portugal)](README.pt-PT.md)

[Descarga](#descarga) · [Primeros pasos](#primeros-pasos) · [Funciones](#características) · [Ayuda](#comunidad-y-contribuciones)

## ¿Qué es?

**LF Botonera de Efectos** es una aplicación de escritorio para disparar sonidos en vivo con precisión y velocidad. Está pensada para locutores, emisoras de radio, operadores técnicos, productores, creadores de contenido y streamers que necesitan una botonera confiable y rápida durante una transmisión real.

## Capturas de pantalla

Capturas de Windows con interfaz en español, tomadas en julio de 2026. La versión actual puede presentar diferencias.

![Rejilla de botones en tema claro, con efectos organizados por colores y controles de reproducción.](Capturas/windows/es/principal/01-rejilla-modo-claro.png)

<details>
<summary>Ver tema oscuro y editor de pistas</summary>

![Rejilla en tema oscuro con el panel lateral de botones fijos.](Capturas/windows/es/principal/05-rejilla-modo-oscuro-panel-fijo.png)

![Editor de pistas con forma de onda, puntos de inicio y fin y controles de ganancia y normalización.](Capturas/windows/es/editor/01-forma-de-onda-y-cues.png)

</details>

## Descarga

**[Descargar desde Microsoft Store](https://apps.microsoft.com/detail/9NJ8ST39QP7V)** (recomendado para Windows, con actualizaciones automáticas) · **[Descargas directas para Windows y Linux](https://github.com/yosoyluisfernando/LF-Botonera-de-efectos/releases/latest)**

| Sistema | Archivo recomendado | Descripción |
|---|---|---|
| Windows 10/11 | [Microsoft Store](https://apps.microsoft.com/detail/9NJ8ST39QP7V) | Opción recomendada: instalación sencilla y actualizaciones automáticas |
| Windows 10/11 | `LF.Botonera.de.Efectos_*_x64-setup.exe` | Instalador directo con asistente |
| Windows 10/11 | `LF.Botonera.de.Efectos_*_x64_en-US.msi` | Instalador MSI para entornos controlados o corporativos |
| Ubuntu / Debian | `LF.Botonera.de.Efectos_*_amd64.deb` | Paquete `.deb` para distribuciones basadas en Debian |
| Fedora / openSUSE | `LF.Botonera.de.Efectos_*_x86_64.rpm` | Paquete RPM para distribuciones compatibles |
| Linux universal | `LF.Botonera.de.Efectos_*_amd64.AppImage` | Ejecutable portable, sin instalación |

> **Windows:** requiere Windows 10 20H2 o posterior con WebView2 Runtime (incluido en Windows 11 y en versiones actualizadas de Windows 10).

> Si ya utilizabas el instalador MSI o EXE, consulta la guía de [migración a Microsoft Store](Documentación/MIGRACION_MICROSOFT_STORE.md) para conservar tu configuración al cambiar de edición.

## Primeros pasos

1. Instala la aplicación y completa el asistente inicial.
2. Arrastra un archivo de audio a un botón vacío y púlsalo para reproducirlo.
3. Con clic derecho puedes editar el botón y asignarle un atajo.
4. Organiza tus programas en pestañas y perfiles. Consulta el [manual de usuario (español)](MANUAL.md) para más detalles.

## Características

- **Sonidos a mano:** botones personalizables, perfiles, pestañas, atajos de teclado y control MIDI.
- **Música de fondo:** reproductor con cola, volumen y salida propios; detener los efectos no corta la música.
- **Biblioteca y buscador:** encuentra pistas y arrástralas a los botones o al reproductor.
- **Prepara tus audios:** pre-escucha CUE, editor de pistas, ganancia y normalización.
- **Control Mic:** reduce temporalmente el nivel de los destinos elegidos, de forma manual o automática. No mezcla, graba ni transmite el micrófono.
- **Cuatro idiomas:** español, inglés, portugués de Brasil y portugués de Portugal.

<details>
<summary><strong>Ver todas las funciones</strong></summary>

### Organización y reproducción

- Perfiles y pestañas ilimitados, con rejillas configurables, ajustes de audio por perfil y dispositivo de salida por pestaña.
- Colores personalizables que se adaptan al tema claro u oscuro; identificación de botones con emojis e iconos.
- Motor de audio nativo en Rust (rodio + cpal) con mezcla simultánea de múltiples fuentes.
- Modos por botón: bucle, superposición, reinicio al pulsar y detener otros. Controles globales Normal, Loop, Multi, Restart, Solo y Detener todo.
- Carpeta secuencial: cada pulsación reproduce el siguiente archivo.
- Locuciones de hora compuestas con tus archivos de audio; anuncios de temperatura y humedad con datos de Open-Meteo.

### Teclado, MIDI y pre-escucha

- Atajos para botones, pestañas y acciones globales, incluidos los controles del reproductor. Los globales pueden funcionar con la ventana en segundo plano; el modo de mapeo visual muestra las asignaciones sobre la rejilla.
- Entradas MIDI seleccionables y asignaciones Note On, Control Change o Program Change a botones, pestañas y acciones globales.
- Pre-escucha CUE por otro dispositivo, como unos auriculares, para comprobar una pista antes de ponerla al aire.
- Panel flotante con nombre del audio, progreso, tiempo, volumen, botón Stop y desplazamiento por la pista con un clic.

### Editor de pistas

- Forma de onda que refleja la ganancia aplicada y marca en rojo las zonas que saturarían; zoom de 1× a 30× con deslizador o Ctrl + rueda del ratón.
- Puntos de inicio y fin sin modificar el original. Normalización opcional a −14 LUFS con techo de pico de −1 dBFS y ajuste manual de ganancia en dB.
- Análisis con progreso y caché persistente de forma de onda para reabrir más rápido las pistas ya analizadas.
- Reproducción con reinicio sin duplicar el audio, Stop cíclico (pausa → vuelve a la marca → vuelve a 0:00), reproducción desde el cue y cursor animado a 60 fps.
- Editor integrado o en ventana independiente. Los ajustes se guardan por archivo: cue y ganancia se aplican a todos los botones que lo usan y pueden acompañar las pestañas y perfiles exportados.

### Biblioteca, metadatos y respaldo

- Ventana de Biblioteca para recorrer Música, Efectos, carpetas indexadas y unidades locales. Búsqueda en el panel fijo por nombre, título, artista, álbum, género y palabras clave.
- Actualización automática al detectar cambios en las carpetas. Reproduce al aire, escucha en CUE, añade al reproductor o arrastra una pista a un botón.
- Edición individual o múltiple de metadatos y tags. Renombrado físico opcional y escritura opcional de metadatos compatibles dentro de una pista individual de Música.
- Retiro reversible de carpetas del catálogo sin borrar sus audios, con retención configurable de 30 a 365 días.
- Respaldo `.lfbackup` de configuración y base de datos. **No incluye los archivos de audio.** La restauración valida el respaldo, crea una copia de seguridad del estado actual y reinicia la aplicación.

### Rendimiento e interfaz

- Precarga en RAM del perfil completo, la pestaña visible o los audios al reproducirlos. Caché de 32, 64, 128 o 256 MB, con caducidad configurable para pistas sin uso y acceso inmediato al cue de los audios en caché.
- Temas claro, oscuro y automático; arrastre de archivos y pistas, reordenamiento de botones con Alt + arrastre y pestañas reordenables.
- Vúmetro estéreo, reloj, fecha, cuenta regresiva, deshacer/rehacer, tamaño de texto ajustable, asistente inicial y aviso de actualizaciones.
- Recuperación automática si el dispositivo de audio configurado no está disponible al iniciar.

</details>

## Formatos de audio compatibles

MP3 · WAV · FLAC · OGG/Vorbis · OGG/Opus · AAC · M4A · AIFF

La decodificación se apoya en symphonia (decodificador puro Rust), sin depender de componentes propietarios de pago.

## Compatibilidad con LF Automatizador

LF Botonera de Efectos mantiene compatibilidad de formato con el LF Automatizador:

| Formato | Contenido |
|---|---|
| `.bdelf` | Una pestaña (paleta) de botones |
| `.bdeplf` | Un perfil completo con todas sus pestañas |

Los archivos exportados desde la Botonera pueden abrirse en el LF Automatizador y viceversa. Los campos exclusivos de cada aplicación se ignoran sin errores.

Los exports desde la Botonera incluyen opcionalmente el cue y la ganancia de cada audio (campo `bdelf_tracks`). El LFA los ignora; la Botonera los restaura al importar, adaptándolos a las rutas del equipo destino.

## Compilar desde el código fuente

Necesitas [Node.js 20+ con npm](https://nodejs.org/), [Rust estable](https://rustup.rs/) y los [requisitos de Tauri 2](https://tauri.app/start/prerequisites/) para tu sistema.

Desde la carpeta del repositorio, ejecuta `npm install` y después `npm run tauri build`. Los instaladores quedan en `src-tauri/target/release/bundle/`. Para desarrollo con recarga automática, usa `npm run tauri dev` o `DEV.bat` en Windows.

Consulta [Compilación y versiones](Documentación/COMPILACION_Y_VERSIONES.md) para las instrucciones de compilación y el proceso de publicación; la [Arquitectura](Documentación/ARCHITECTURE.md) describe la estructura del código. Antes de contribuir, lee la [guía de contribución](Documentación/CONTRIBUTING.md).

## Firma de Windows

Los instaladores directos EXE/MSI de Windows no están firmados con certificado de desarrollador y pueden mostrar una advertencia al abrirlos por primera vez. Microsoft Store gestiona la firma y las actualizaciones de su propia edición. El código fuente está disponible públicamente y el proyecto se distribuye bajo licencia libre.

## Comunidad y Contribuciones

| Documento | Descripción |
|---|---|
| **[Guía de Contribución](Documentación/CONTRIBUTING.md)** | Cómo empezar, reglas del código y cómo enviar un PR. |
| **[Código de Conducta](Documentación/CODE_OF_CONDUCT.md)** | Nuestras normas para mantener un ambiente profesional e inclusivo. |
| **[Política de Seguridad](Documentación/SECURITY.md)** | Cómo reportar vulnerabilidades de forma responsable. |
| **[Sitio web oficial](https://lfbotonera.blogspot.com/)** | Presentación, funciones, capturas y disponibilidad del proyecto. |
| **[Política de Privacidad](PRIVACY.md)** | Qué se guarda localmente y qué conexiones externas realiza la aplicación. |
| **[Soporte](SUPPORT.md)** | Cómo pedir ayuda o informar un problema reproducible. |
| **[Avisos de terceros](THIRD_PARTY_NOTICES.md)** | Estado de licencias y avisos de componentes incluidos. |

La documentación enlazada está principalmente en español. El selector superior cambia el idioma de esta presentación.

También puedes unirte a nuestros espacios de comunicación:

| Espacio | Enlace |
|---|---|
| Canal de información | [Entrar al canal](https://t.me/+XKof2wDvGVw1YTRh) |
| Grupo de comunidad | [Entrar al grupo](https://t.me/+bXppwWvJvSg5YjNh) |

## Apoyar el desarrollo

Si esta herramienta te resulta útil en radio, producción o streaming, puedes apoyar su mantenimiento:

**[Donar vía PayPal](https://www.paypal.com/donate/?hosted_button_id=3JJVFFBVR4MQQ)**

## Créditos de desarrollo

**Autor y creador:** Luis Fernando Velásquez

Este proyecto fue desarrollado con asistencia de herramientas de inteligencia artificial:

- **[Claude AI](https://claude.ai)** (Anthropic) — asistencia en arquitectura Rust, implementación del motor de audio, documentación técnica y revisión de código.
- **[Codex / GPT](https://openai.com)** (OpenAI) — asistencia en etapas iniciales del desarrollo.

Las herramientas de IA participaron como asistentes de desarrollo. El diseño del producto, las decisiones de arquitectura, la dirección creativa y la autoría del proyecto pertenecen al autor.

## Licencia

Copyright (C) 2026 **Luis Fernando Velásquez**.

Este programa es **software libre**: puedes redistribuirlo y/o modificarlo bajo los términos de la **Licencia Pública General de GNU GPL-3.0-or-later**.

Este programa se distribuye con la esperanza de que sea útil, pero **sin garantía alguna**. Consulta el archivo [LICENSE](LICENSE) para el texto completo.
