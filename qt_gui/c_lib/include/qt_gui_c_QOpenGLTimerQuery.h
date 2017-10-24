#ifndef QT_GUI_C_QOPENGLTIMERQUERY_H
#define QT_GUI_C_QOPENGLTIMERQUERY_H

#include "qt_gui_c_global.h"

extern "C" {

QT_GUI_C_EXPORT QObject* qt_gui_c_QOpenGLTimerQuery_G_static_cast_QObject_ptr(QOpenGLTimerQuery* ptr);
QT_GUI_C_EXPORT QOpenGLTimerQuery* qt_gui_c_QOpenGLTimerQuery_G_static_cast_QOpenGLTimerQuery_ptr(QObject* ptr);
QT_GUI_C_EXPORT void qt_gui_c_QOpenGLTimerQuery_begin(QOpenGLTimerQuery* this_ptr);
QT_GUI_C_EXPORT bool qt_gui_c_QOpenGLTimerQuery_create(QOpenGLTimerQuery* this_ptr);
QT_GUI_C_EXPORT void qt_gui_c_QOpenGLTimerQuery_delete(QOpenGLTimerQuery* this_ptr);
QT_GUI_C_EXPORT void qt_gui_c_QOpenGLTimerQuery_destroy(QOpenGLTimerQuery* this_ptr);
QT_GUI_C_EXPORT void qt_gui_c_QOpenGLTimerQuery_end(QOpenGLTimerQuery* this_ptr);
QT_GUI_C_EXPORT bool qt_gui_c_QOpenGLTimerQuery_isCreated(const QOpenGLTimerQuery* this_ptr);
QT_GUI_C_EXPORT bool qt_gui_c_QOpenGLTimerQuery_isResultAvailable(const QOpenGLTimerQuery* this_ptr);
QT_GUI_C_EXPORT const QMetaObject* qt_gui_c_QOpenGLTimerQuery_metaObject(const QOpenGLTimerQuery* this_ptr);
QT_GUI_C_EXPORT QOpenGLTimerQuery* qt_gui_c_QOpenGLTimerQuery_new_no_args();
QT_GUI_C_EXPORT QOpenGLTimerQuery* qt_gui_c_QOpenGLTimerQuery_new_parent(QObject* parent);
QT_GUI_C_EXPORT GLuint qt_gui_c_QOpenGLTimerQuery_objectId(const QOpenGLTimerQuery* this_ptr);
QT_GUI_C_EXPORT int qt_gui_c_QOpenGLTimerQuery_qt_metacall(QOpenGLTimerQuery* this_ptr, const QMetaObject::Call* arg1, int arg2, void** arg3);
QT_GUI_C_EXPORT void* qt_gui_c_QOpenGLTimerQuery_qt_metacast(QOpenGLTimerQuery* this_ptr, const char* arg1);
QT_GUI_C_EXPORT void qt_gui_c_QOpenGLTimerQuery_recordTimestamp(QOpenGLTimerQuery* this_ptr);
QT_GUI_C_EXPORT void qt_gui_c_QOpenGLTimerQuery_trUtf8_to_output(const char* s, const char* c, int n, QString* output);
QT_GUI_C_EXPORT void qt_gui_c_QOpenGLTimerQuery_tr_to_output(const char* s, const char* c, int n, QString* output);
QT_GUI_C_EXPORT GLuint64 qt_gui_c_QOpenGLTimerQuery_waitForResult(const QOpenGLTimerQuery* this_ptr);
QT_GUI_C_EXPORT GLuint64 qt_gui_c_QOpenGLTimerQuery_waitForTimestamp(const QOpenGLTimerQuery* this_ptr);

} // extern "C"

#endif // QT_GUI_C_QOPENGLTIMERQUERY_H
