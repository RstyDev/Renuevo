📖 Sistema de Administración Eclesiástica

Este proyecto es una aplicación end-to-end desarrollada en Rust, pensada para la administración de instituciones eclesiásticas.

Frontend: Sycamore
 (WebAssembly con Rust)

Backend: Actix Web

Base de datos: SurrealDB

Hosting: Render.com

La arquitectura está separada en frontend y backend, lo que permite un despliegue escalable y modular.

🚀 Entornos y Hosting

La aplicación está hosteada en Render.com, con entornos separados.

👉 Para pruebas públicas, se encuentra disponible un entorno QA:

Frontend: https://renuevoqa.onrender.com

Backend: https://renuevobackendqa.onrender.com

⚠️ Importante:
Render utiliza hosting gratuito, lo que implica que:

Las apps entran en modo “sleep” tras un tiempo de inactividad.

Al acceder nuevamente, pueden tardar ~1 minuto en reactivarse.

Como frontend y backend están separados, es necesario “despertar” ambos servicios antes de usarlos.

🔑 Usuario público de prueba

En el entorno QA se encuentra disponible un usuario preconfigurado:

Nombre: admin
Apellido: admin
Contraseña: 232323


Puedes usar estas credenciales para explorar la aplicación sin necesidad de registro.

🛠️ Tecnologías principales

Rust como lenguaje base

Sycamore para el frontend (WebAssembly)

Actix Web para el backend

SurrealDB como base de datos orientada a grafos y documentos

Render.com como proveedor de hosting

📌 Notas finales

Este proyecto es una muestra de cómo desarrollar un sistema de administración completo en Rust end-to-end, con separación clara entre frontend, backend y base de datos.

Cualquier aporte o feedback es bienvenido 🙌
