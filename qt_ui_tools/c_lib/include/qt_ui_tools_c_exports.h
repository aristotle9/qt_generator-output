#ifndef QT_UI_TOOLS_C_EXPORTS_H
#define QT_UI_TOOLS_C_EXPORTS_H

// This header creates a definition required to export the library's
// symbols properly on all platforms.

#ifdef _WIN32
    #ifdef QT_UI_TOOLS_C_LIBRARY
        #define QT_UI_TOOLS_C_EXPORT __declspec(dllexport)
    #else
        #define QT_UI_TOOLS_C_EXPORT __declspec(dllimport)
    #endif
#else
    #define QT_UI_TOOLS_C_EXPORT
#endif

#endif // QT_UI_TOOLS_C_EXPORTS_H

