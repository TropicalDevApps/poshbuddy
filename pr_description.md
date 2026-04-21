## 🎯 What
Se añade validación de caracteres para prevenir *path traversal* (salto de directorios) en las funciones que descargan archivos de temas basados en nombres controlados externamente. Además se incluyen pruebas unitarias (`tests`) que certifican que la mitigación se está aplicando correctamente en los dos métodos involucrados.

## ⚠️ Risk
Como la variable `name` de las funciones de descarga no estaba siendo saneada, un actor malicioso o un nombre de tema manipulado que incluyera secuencias como `../` podría provocar que la función escribiese fuera del directorio local esperado (ej. `target_dir.join("../../../archivo_critico.json")`), provocando vulnerabilidades críticas de sobreescritura arbitraria de ficheros en el disco local o sobrescribir ficheros de configuración de la terminal.

## 🛡️ Solution
1. **Mitigación Activa**: Se agregó la validación `if name.contains("..") || name.contains('/') || name.contains('\\')` al comienzo de las funciones `download_theme_file` y `download_to_temp` dentro de `src/api.rs`.
2. **Rechazo Inmediato**: Si se detecta cualquiera de esos caracteres de *path traversal* en el nombre solicitado, la función retorna inmediatamente `Err` evitando por completo la interacción con el sistema de archivos local y deteniendo cualquier petición externa de descarga.
3. **Casos de prueba (Testing)**: Se incorporaron los test unitarios `test_download_theme_file_path_traversal` y `test_download_to_temp_path_traversal` en el módulo de pruebas `tests` que demuestran efectivamente que estos nombres defectuosos retornan error indicando *Path traversal detected*.

### Plan de Archivos y Bloques Alterados
* `src/api.rs`:
    * Modificada la función `download_theme_file` para integrar la barrera de validación del nombre en la primera línea.
    * Modificada la función `download_to_temp` para integrar la misma verificación inicial.
    * Agregados los bloques de testing correspondientes (`test_download_theme_file_path_traversal` y `test_download_to_temp_path_traversal`) dentro de `mod tests`.
