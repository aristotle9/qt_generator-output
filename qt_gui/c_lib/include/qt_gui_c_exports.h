#ifndef QT_GUI_C_EXPORTS_H
#define QT_GUI_C_EXPORTS_H

// This header creates a definition required to export the library's
// symbols properly on all platforms.

#ifdef _WIN32
    #ifdef QT_GUI_C_LIBRARY
        #define QT_GUI_C_EXPORT __declspec(dllexport)
    #else
        #define QT_GUI_C_EXPORT __declspec(dllimport)
    #endif
#else
    #define QT_GUI_C_EXPORT
#endif

#endif // QT_GUI_C_EXPORTS_H

