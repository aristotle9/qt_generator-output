#ifndef QT_GUI_C_QOPENGLWINDOW_H
#define QT_GUI_C_QOPENGLWINDOW_H

#include "qt_gui_c_global.h"

extern "C" {

QT_GUI_C_EXPORT QOpenGLWindow* qt_gui_c_QOpenGLWindow_G_dynamic_cast_QOpenGLWindow_ptr_QPaintDevice(QPaintDevice* ptr);
QT_GUI_C_EXPORT QOpenGLWindow* qt_gui_c_QOpenGLWindow_G_dynamic_cast_QOpenGLWindow_ptr_QPaintDeviceWindow(QPaintDeviceWindow* ptr);
QT_GUI_C_EXPORT QOpenGLWindow* qt_gui_c_QOpenGLWindow_G_dynamic_cast_QOpenGLWindow_ptr_QSurface(QSurface* ptr);
QT_GUI_C_EXPORT QOpenGLWindow* qt_gui_c_QOpenGLWindow_G_dynamic_cast_QOpenGLWindow_ptr_QWindow(QWindow* ptr);
QT_GUI_C_EXPORT QObject* qt_gui_c_QOpenGLWindow_G_static_cast_QObject_ptr(QOpenGLWindow* ptr);
QT_GUI_C_EXPORT QOpenGLWindow* qt_gui_c_QOpenGLWindow_G_static_cast_QOpenGLWindow_ptr_QObject(QObject* ptr);
QT_GUI_C_EXPORT QOpenGLWindow* qt_gui_c_QOpenGLWindow_G_static_cast_QOpenGLWindow_ptr_QPaintDevice(QPaintDevice* ptr);
QT_GUI_C_EXPORT QOpenGLWindow* qt_gui_c_QOpenGLWindow_G_static_cast_QOpenGLWindow_ptr_QPaintDeviceWindow(QPaintDeviceWindow* ptr);
QT_GUI_C_EXPORT QOpenGLWindow* qt_gui_c_QOpenGLWindow_G_static_cast_QOpenGLWindow_ptr_QSurface(QSurface* ptr);
QT_GUI_C_EXPORT QOpenGLWindow* qt_gui_c_QOpenGLWindow_G_static_cast_QOpenGLWindow_ptr_QWindow(QWindow* ptr);
QT_GUI_C_EXPORT QPaintDeviceWindow* qt_gui_c_QOpenGLWindow_G_static_cast_QPaintDeviceWindow_ptr(QOpenGLWindow* ptr);
QT_GUI_C_EXPORT QPaintDevice* qt_gui_c_QOpenGLWindow_G_static_cast_QPaintDevice_ptr(QOpenGLWindow* ptr);
QT_GUI_C_EXPORT QSurface* qt_gui_c_QOpenGLWindow_G_static_cast_QSurface_ptr(QOpenGLWindow* ptr);
QT_GUI_C_EXPORT QWindow* qt_gui_c_QOpenGLWindow_G_static_cast_QWindow_ptr(QOpenGLWindow* ptr);
QT_GUI_C_EXPORT QOpenGLContext* qt_gui_c_QOpenGLWindow_context(const QOpenGLWindow* this_ptr);
QT_GUI_C_EXPORT GLuint qt_gui_c_QOpenGLWindow_defaultFramebufferObject(const QOpenGLWindow* this_ptr);
QT_GUI_C_EXPORT void qt_gui_c_QOpenGLWindow_delete(QOpenGLWindow* this_ptr);
QT_GUI_C_EXPORT void qt_gui_c_QOpenGLWindow_doneCurrent(QOpenGLWindow* this_ptr);
QT_GUI_C_EXPORT QImage* qt_gui_c_QOpenGLWindow_grabFramebuffer_as_ptr(QOpenGLWindow* this_ptr);
QT_GUI_C_EXPORT bool qt_gui_c_QOpenGLWindow_isValid(const QOpenGLWindow* this_ptr);
QT_GUI_C_EXPORT void qt_gui_c_QOpenGLWindow_makeCurrent(QOpenGLWindow* this_ptr);
QT_GUI_C_EXPORT const QMetaObject* qt_gui_c_QOpenGLWindow_metaObject(const QOpenGLWindow* this_ptr);
QT_GUI_C_EXPORT QOpenGLWindow* qt_gui_c_QOpenGLWindow_new_no_args();
QT_GUI_C_EXPORT QOpenGLWindow* qt_gui_c_QOpenGLWindow_new_shareContext(QOpenGLContext* shareContext);
QT_GUI_C_EXPORT QOpenGLWindow* qt_gui_c_QOpenGLWindow_new_shareContext_updateBehavior(QOpenGLContext* shareContext, QOpenGLWindow::UpdateBehavior updateBehavior);
QT_GUI_C_EXPORT QOpenGLWindow* qt_gui_c_QOpenGLWindow_new_shareContext_updateBehavior_parent(QOpenGLContext* shareContext, QOpenGLWindow::UpdateBehavior updateBehavior, QWindow* parent);
QT_GUI_C_EXPORT QOpenGLWindow* qt_gui_c_QOpenGLWindow_new_updateBehavior(QOpenGLWindow::UpdateBehavior updateBehavior);
QT_GUI_C_EXPORT QOpenGLWindow* qt_gui_c_QOpenGLWindow_new_updateBehavior_parent(QOpenGLWindow::UpdateBehavior updateBehavior, QWindow* parent);
QT_GUI_C_EXPORT int qt_gui_c_QOpenGLWindow_qt_metacall(QOpenGLWindow* this_ptr, const QMetaObject::Call* arg1, int arg2, void** arg3);
QT_GUI_C_EXPORT void* qt_gui_c_QOpenGLWindow_qt_metacast(QOpenGLWindow* this_ptr, const char* arg1);
QT_GUI_C_EXPORT QOpenGLContext* qt_gui_c_QOpenGLWindow_shareContext(const QOpenGLWindow* this_ptr);
QT_GUI_C_EXPORT void qt_gui_c_QOpenGLWindow_trUtf8_to_output(const char* s, const char* c, int n, QString* output);
QT_GUI_C_EXPORT void qt_gui_c_QOpenGLWindow_tr_to_output(const char* s, const char* c, int n, QString* output);
QT_GUI_C_EXPORT QOpenGLWindow::UpdateBehavior qt_gui_c_QOpenGLWindow_updateBehavior(const QOpenGLWindow* this_ptr);

} // extern "C"

#endif // QT_GUI_C_QOPENGLWINDOW_H
