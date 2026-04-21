## 🎯 What
Se solucionan las alertas de seguridad de CodeQL reportadas en la Integración Continua (CI) referidas al uso de URLs HTTP inseguras en los datos simulados de los tests unitarios.

## ⚠️ Risk
CodeQL detectó que las URLs *hardcodeadas* como `http://example.com` en los tests fluían a través del método `.get(url)` del cliente HTTP, lo que la herramienta estática identifica como un riesgo de severidad alta. Aunque eran valores *mock*, hacían fallar la validación de seguridad de la rama.

## 🛡️ Solution
Se reemplazaron todas las referencias a `http://example.com` por su equivalente seguro `https://example.com` dentro de las macros de mockeo y pruebas unitarias en `src/api.rs`.

### Plan de Archivos y Bloques Alterados
* `src/api.rs`:
    * Actualizado el cuerpo JSON simulado de temas remotos en `test_setup_app_task_success` a usar URLs con HTTPS.
    * Actualizado el cuerpo JSON simulado en `test_setup_app_task_ignores_invalid_themes` a usar URLs con HTTPS.
    * Actualizados los argumentos de invocación en `test_download_theme_file_path_traversal` y `test_download_to_temp_path_traversal` a usar HTTPS.
