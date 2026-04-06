# Guía de Contribución a PoshBuddy

¡Gracias por interesarte en mejorar PoshBuddy! Queremos que contribuir sea lo más fácil y transparente posible.

## 🚀 ¿Cómo puedo contribuir?

### 1. Reportar Bugs
Si encuentras un error, por favor utiliza nuestra [Plantilla de Bug Report](.github/ISSUE_TEMPLATE/bug_report.yml). Asegúrate de incluir:
- Pasos claros para reproducir el error.
- El comportamiento esperado vs. el actual.
- Tu entorno (versión de PowerShell, OS, etc.).

### 2. Sugerir Mejoras
¡Las ideas nuevas son bienvenidas! Utiliza la [Plantilla de Feature Request](.github/ISSUE_TEMPLATE/feature_request.yml) para describir tu propuesta y por qué crees que sería útil para la comunidad.

### 3. Enviar Pull Requests
1. Haz un fork del repositorio.
2. Crea una rama para tu funcionalidad (`git checkout -b feature/nueva-mejora`).
3. Realiza tus cambios siguiendo el estilo de código existente (PowerShell nativo, minimalista).
4. Asegúrate de que el script `PoshBuddy.ps1` funcione correctamente.
5. Haz un commit de tus cambios (`git commit -am 'Añade funcionalidad X'`).
6. Sube los cambios a tu rama (`git push origin feature/nueva-mejora`).
7. Abre un Pull Request.

## 🛠️ Estándares de Código
- Usamos **PowerShell nativo** para evitar dependencias pesadas.
- Preferimos manipular la consola mediante `RawUI` para mantener la experiencia TUI fluida.
- Los temas se gestionan en `~/.poshthemes`.

## 🗺️ Roadmap 2026
Consulta el `README.md` para ver los objetivos a largo plazo del proyecto.

---
**Desarrollado con rigor técnico por Julio (Senior Engineer).**
