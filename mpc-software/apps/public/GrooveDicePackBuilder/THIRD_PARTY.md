# Third-party software

The application uses Python, PySide6/Qt 6 (LGPLv3/GPL/commercial options; this bundle uses the LGPL-compatible dynamically linked distribution), Shiboken6, and Qt Multimedia's FFmpeg backend. The binary's THIRD_PARTY folder contains notices from the installed wheels. Retain them when redistributing. Qt and FFmpeg are dynamically linked; users can replace compatible libraries. The application does not prevent such replacement. Source, pinned requirements and build instructions are public in the same repository.

- Qt / PySide6: https://code.qt.io/pyside/pyside-setup.git/ ; https://doc.qt.io/qtforpython-6/licenses.html
- FFmpeg: https://ffmpeg.org/legal.html and https://ffmpeg.org/download.html (Qt's runtime reports FFmpeg 7.1.2, LGPL 2.1+ on the tested Mac).
- Python: https://www.python.org/downloads/source/ (PSF license).
- PyInstaller: https://pyinstaller.org/ (GPL with bootloader distribution exception).

The app has no bundled commercial drum samples. Approved interface artwork was generated with the built-in image generation tool; source concept and prompt are in design/. A small vinyl cover region is rendered from that asset. Exported sound packs retain the creator's own declared rights; choosing “Redistributable” does not independently establish legal clearance.

Pack Builder source is copyright kim. Public availability does not alter third-party library licenses. No Apple Developer ID certificate or Windows signing certificate is bundled. macOS builds are ad-hoc signed, not notarized; Windows builds are not code-signed.
