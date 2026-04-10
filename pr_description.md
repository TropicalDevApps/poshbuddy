🎯 **Qué:** La falta de cobertura de pruebas para la función `filtered_themes` dentro de la estructura `App` en `src/app.rs`.
📊 **Cobertura:** Los escenarios ahora evaluados incluyen:
  - Filtro vacío (devuelve todos los temas).
  - Coincidencia ignorando mayúsculas y minúsculas ("case-insensitive match").
  - Coincidencia parcial con un elemento de la lista.
  - Sin coincidencia (filtro que no se encuentra en ningún tema).
✨ **Resultado:** Mejora de la cobertura de pruebas al verificar directamente el comportamiento del filtrado.

### Archivos Modificados
- `src/app.rs`: Se ha añadido el bloque `#[cfg(test)] mod tests` al final del archivo, incluyendo la implementación de `create_test_app()` para instanciar la estructura directamente sin efectos secundarios y 4 pruebas individuales.
