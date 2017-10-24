#ifndef QT_WIDGETS_C_QOPENGLWIDGET_H
#define QT_WIDGETS_C_QOPENGLWIDGET_H

#include "qt_widgets_c_global.h"

extern "C" {

QT_WIDGETS_C_EXPORT QOpenGLWidget* qt_widgets_c_QOpenGLWidget_G_dynamic_cast_QOpenGLWidget_ptr(QWidget* ptr);
QT_WIDGETS_C_EXPORT QObject* qt_widgets_c_QOpenGLWidget_G_static_cast_QObject_ptr(QOpenGLWidget* ptr);
QT_WIDGETS_C_EXPORT QOpenGLWidget* qt_widgets_c_QOpenGLWidget_G_static_cast_QOpenGLWidget_ptr_QObject(QObject* ptr);
QT_WIDGETS_C_EXPORT QOpenGLWidget* qt_widgets_c_QOpenGLWidget_G_static_cast_QOpenGLWidget_ptr_QPaintDevice(QPaintDevice* ptr);
QT_WIDGETS_C_EXPORT QOpenGLWidget* qt_widgets_c_QOpenGLWidget_G_static_cast_QOpenGLWidget_ptr_QWidget(QWidget* ptr);
QT_WIDGETS_C_EXPORT QPaintDevice* qt_widgets_c_QOpenGLWidget_G_static_cast_QPaintDevice_ptr(QOpenGLWidget* ptr);
QT_WIDGETS_C_EXPORT QWidget* qt_widgets_c_QOpenGLWidget_G_static_cast_QWidget_ptr(QOpenGLWidget* ptr);
QT_WIDGETS_C_EXPORT QOpenGLContext* qt_widgets_c_QOpenGLWidget_context(const QOpenGLWidget* this_ptr);
QT_WIDGETS_C_EXPORT GLuint qt_widgets_c_QOpenGLWidget_defaultFramebufferObject(const QOpenGLWidget* this_ptr);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QOpenGLWidget_delete(QOpenGLWidget* this_ptr);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QOpenGLWidget_doneCurrent(QOpenGLWidget* this_ptr);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QOpenGLWidget_format_to_output(const QOpenGLWidget* this_ptr, QSurfaceFormat* output);
QT_WIDGETS_C_EXPORT QImage* qt_widgets_c_QOpenGLWidget_grabFramebuffer_as_ptr(QOpenGLWidget* this_ptr);
QT_WIDGETS_C_EXPORT bool qt_widgets_c_QOpenGLWidget_isValid(const QOpenGLWidget* this_ptr);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QOpenGLWidget_makeCurrent(QOpenGLWidget* this_ptr);
QT_WIDGETS_C_EXPORT const QMetaObject* qt_widgets_c_QOpenGLWidget_metaObject(const QOpenGLWidget* this_ptr);
QT_WIDGETS_C_EXPORT int qt_widgets_c_QOpenGLWidget_qt_metacall(QOpenGLWidget* this_ptr, const QMetaObject::Call* arg1, int arg2, void** arg3);
QT_WIDGETS_C_EXPORT void* qt_widgets_c_QOpenGLWidget_qt_metacast(QOpenGLWidget* this_ptr, const char* arg1);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QOpenGLWidget_setFormat(QOpenGLWidget* this_ptr, const QSurfaceFormat* format);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QOpenGLWidget_setUpdateBehavior(QOpenGLWidget* this_ptr, QOpenGLWidget::UpdateBehavior updateBehavior);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QOpenGLWidget_trUtf8_to_output(const char* s, const char* c, int n, QString* output);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QOpenGLWidget_tr_to_output(const char* s, const char* c, int n, QString* output);
QT_WIDGETS_C_EXPORT QOpenGLWidget::UpdateBehavior qt_widgets_c_QOpenGLWidget_updateBehavior(const QOpenGLWidget* this_ptr);

} // extern "C"

#endif // QT_WIDGETS_C_QOPENGLWIDGET_H
