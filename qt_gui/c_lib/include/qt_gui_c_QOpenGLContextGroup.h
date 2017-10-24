#ifndef QT_GUI_C_QOPENGLCONTEXTGROUP_H
#define QT_GUI_C_QOPENGLCONTEXTGROUP_H

#include "qt_gui_c_global.h"

extern "C" {

QT_GUI_C_EXPORT QObject* qt_gui_c_QOpenGLContextGroup_G_static_cast_QObject_ptr(QOpenGLContextGroup* ptr);
QT_GUI_C_EXPORT QOpenGLContextGroup* qt_gui_c_QOpenGLContextGroup_G_static_cast_QOpenGLContextGroup_ptr(QObject* ptr);
QT_GUI_C_EXPORT QOpenGLContextGroup* qt_gui_c_QOpenGLContextGroup_currentContextGroup();
QT_GUI_C_EXPORT void qt_gui_c_QOpenGLContextGroup_delete(QOpenGLContextGroup* this_ptr);
QT_GUI_C_EXPORT const QMetaObject* qt_gui_c_QOpenGLContextGroup_metaObject(const QOpenGLContextGroup* this_ptr);
QT_GUI_C_EXPORT int qt_gui_c_QOpenGLContextGroup_qt_metacall(QOpenGLContextGroup* this_ptr, const QMetaObject::Call* arg1, int arg2, void** arg3);
QT_GUI_C_EXPORT void* qt_gui_c_QOpenGLContextGroup_qt_metacast(QOpenGLContextGroup* this_ptr, const char* arg1);
QT_GUI_C_EXPORT void qt_gui_c_QOpenGLContextGroup_shares_to_output(const QOpenGLContextGroup* this_ptr, QList< QOpenGLContext* >* output);
QT_GUI_C_EXPORT void qt_gui_c_QOpenGLContextGroup_trUtf8_to_output(const char* s, const char* c, int n, QString* output);
QT_GUI_C_EXPORT void qt_gui_c_QOpenGLContextGroup_tr_to_output(const char* s, const char* c, int n, QString* output);

} // extern "C"

#endif // QT_GUI_C_QOPENGLCONTEXTGROUP_H
