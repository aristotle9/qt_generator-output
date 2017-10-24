#ifndef QT_GUI_C_QSURFACE_H
#define QT_GUI_C_QSURFACE_H

#include "qt_gui_c_global.h"

extern "C" {

QT_GUI_C_EXPORT void qt_gui_c_QSurface_delete(QSurface* this_ptr);
QT_GUI_C_EXPORT void qt_gui_c_QSurface_format_to_output(const QSurface* this_ptr, QSurfaceFormat* output);
QT_GUI_C_EXPORT void qt_gui_c_QSurface_size_to_output(const QSurface* this_ptr, QSize* output);
QT_GUI_C_EXPORT bool qt_gui_c_QSurface_supportsOpenGL(const QSurface* this_ptr);
QT_GUI_C_EXPORT QSurface::SurfaceClass qt_gui_c_QSurface_surfaceClass(const QSurface* this_ptr);
QT_GUI_C_EXPORT QSurface::SurfaceType qt_gui_c_QSurface_surfaceType(const QSurface* this_ptr);

} // extern "C"

#endif // QT_GUI_C_QSURFACE_H
