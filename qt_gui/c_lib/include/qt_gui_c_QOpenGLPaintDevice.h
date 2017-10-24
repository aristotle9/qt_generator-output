#ifndef QT_GUI_C_QOPENGLPAINTDEVICE_H
#define QT_GUI_C_QOPENGLPAINTDEVICE_H

#include "qt_gui_c_global.h"

extern "C" {

QT_GUI_C_EXPORT QOpenGLPaintDevice* qt_gui_c_QOpenGLPaintDevice_G_dynamic_cast_QOpenGLPaintDevice_ptr(QPaintDevice* ptr);
QT_GUI_C_EXPORT QOpenGLPaintDevice* qt_gui_c_QOpenGLPaintDevice_G_static_cast_QOpenGLPaintDevice_ptr(QPaintDevice* ptr);
QT_GUI_C_EXPORT QPaintDevice* qt_gui_c_QOpenGLPaintDevice_G_static_cast_QPaintDevice_ptr(QOpenGLPaintDevice* ptr);
QT_GUI_C_EXPORT QOpenGLContext* qt_gui_c_QOpenGLPaintDevice_context(const QOpenGLPaintDevice* this_ptr);
QT_GUI_C_EXPORT void qt_gui_c_QOpenGLPaintDevice_delete(QOpenGLPaintDevice* this_ptr);
QT_GUI_C_EXPORT int qt_gui_c_QOpenGLPaintDevice_devType(const QOpenGLPaintDevice* this_ptr);
QT_GUI_C_EXPORT double qt_gui_c_QOpenGLPaintDevice_dotsPerMeterX(const QOpenGLPaintDevice* this_ptr);
QT_GUI_C_EXPORT double qt_gui_c_QOpenGLPaintDevice_dotsPerMeterY(const QOpenGLPaintDevice* this_ptr);
QT_GUI_C_EXPORT void qt_gui_c_QOpenGLPaintDevice_ensureActiveTarget(QOpenGLPaintDevice* this_ptr);
QT_GUI_C_EXPORT QOpenGLPaintDevice* qt_gui_c_QOpenGLPaintDevice_new_no_args();
QT_GUI_C_EXPORT QOpenGLPaintDevice* qt_gui_c_QOpenGLPaintDevice_new_size(const QSize* size);
QT_GUI_C_EXPORT QOpenGLPaintDevice* qt_gui_c_QOpenGLPaintDevice_new_width_height(int width, int height);
QT_GUI_C_EXPORT QPaintEngine* qt_gui_c_QOpenGLPaintDevice_paintEngine(const QOpenGLPaintDevice* this_ptr);
QT_GUI_C_EXPORT bool qt_gui_c_QOpenGLPaintDevice_paintFlipped(const QOpenGLPaintDevice* this_ptr);
QT_GUI_C_EXPORT void qt_gui_c_QOpenGLPaintDevice_setDevicePixelRatio(QOpenGLPaintDevice* this_ptr, double devicePixelRatio);
QT_GUI_C_EXPORT void qt_gui_c_QOpenGLPaintDevice_setDotsPerMeterX(QOpenGLPaintDevice* this_ptr, double arg1);
QT_GUI_C_EXPORT void qt_gui_c_QOpenGLPaintDevice_setDotsPerMeterY(QOpenGLPaintDevice* this_ptr, double arg1);
QT_GUI_C_EXPORT void qt_gui_c_QOpenGLPaintDevice_setPaintFlipped(QOpenGLPaintDevice* this_ptr, bool flipped);
QT_GUI_C_EXPORT void qt_gui_c_QOpenGLPaintDevice_setSize(QOpenGLPaintDevice* this_ptr, const QSize* size);
QT_GUI_C_EXPORT void qt_gui_c_QOpenGLPaintDevice_size_to_output(const QOpenGLPaintDevice* this_ptr, QSize* output);

} // extern "C"

#endif // QT_GUI_C_QOPENGLPAINTDEVICE_H
