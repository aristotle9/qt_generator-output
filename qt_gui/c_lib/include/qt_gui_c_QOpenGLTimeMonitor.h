#ifndef QT_GUI_C_QOPENGLTIMEMONITOR_H
#define QT_GUI_C_QOPENGLTIMEMONITOR_H

#include "qt_gui_c_global.h"

extern "C" {

QT_GUI_C_EXPORT QObject* qt_gui_c_QOpenGLTimeMonitor_G_static_cast_QObject_ptr(QOpenGLTimeMonitor* ptr);
QT_GUI_C_EXPORT QOpenGLTimeMonitor* qt_gui_c_QOpenGLTimeMonitor_G_static_cast_QOpenGLTimeMonitor_ptr(QObject* ptr);
QT_GUI_C_EXPORT bool qt_gui_c_QOpenGLTimeMonitor_create(QOpenGLTimeMonitor* this_ptr);
QT_GUI_C_EXPORT void qt_gui_c_QOpenGLTimeMonitor_delete(QOpenGLTimeMonitor* this_ptr);
QT_GUI_C_EXPORT void qt_gui_c_QOpenGLTimeMonitor_destroy(QOpenGLTimeMonitor* this_ptr);
QT_GUI_C_EXPORT bool qt_gui_c_QOpenGLTimeMonitor_isCreated(const QOpenGLTimeMonitor* this_ptr);
QT_GUI_C_EXPORT bool qt_gui_c_QOpenGLTimeMonitor_isResultAvailable(const QOpenGLTimeMonitor* this_ptr);
QT_GUI_C_EXPORT const QMetaObject* qt_gui_c_QOpenGLTimeMonitor_metaObject(const QOpenGLTimeMonitor* this_ptr);
QT_GUI_C_EXPORT QOpenGLTimeMonitor* qt_gui_c_QOpenGLTimeMonitor_new_no_args();
QT_GUI_C_EXPORT QOpenGLTimeMonitor* qt_gui_c_QOpenGLTimeMonitor_new_parent(QObject* parent);
QT_GUI_C_EXPORT void qt_gui_c_QOpenGLTimeMonitor_objectIds_to_output(const QOpenGLTimeMonitor* this_ptr, QVector< GLuint >* output);
QT_GUI_C_EXPORT int qt_gui_c_QOpenGLTimeMonitor_qt_metacall(QOpenGLTimeMonitor* this_ptr, const QMetaObject::Call* arg1, int arg2, void** arg3);
QT_GUI_C_EXPORT void* qt_gui_c_QOpenGLTimeMonitor_qt_metacast(QOpenGLTimeMonitor* this_ptr, const char* arg1);
QT_GUI_C_EXPORT int qt_gui_c_QOpenGLTimeMonitor_recordSample(QOpenGLTimeMonitor* this_ptr);
QT_GUI_C_EXPORT void qt_gui_c_QOpenGLTimeMonitor_reset(QOpenGLTimeMonitor* this_ptr);
QT_GUI_C_EXPORT int qt_gui_c_QOpenGLTimeMonitor_sampleCount(const QOpenGLTimeMonitor* this_ptr);
QT_GUI_C_EXPORT void qt_gui_c_QOpenGLTimeMonitor_setSampleCount(QOpenGLTimeMonitor* this_ptr, int sampleCount);
QT_GUI_C_EXPORT void qt_gui_c_QOpenGLTimeMonitor_trUtf8_to_output(const char* s, const char* c, int n, QString* output);
QT_GUI_C_EXPORT void qt_gui_c_QOpenGLTimeMonitor_tr_to_output(const char* s, const char* c, int n, QString* output);
QT_GUI_C_EXPORT void qt_gui_c_QOpenGLTimeMonitor_waitForIntervals_to_output(const QOpenGLTimeMonitor* this_ptr, QVector< GLuint64 >* output);
QT_GUI_C_EXPORT void qt_gui_c_QOpenGLTimeMonitor_waitForSamples_to_output(const QOpenGLTimeMonitor* this_ptr, QVector< GLuint64 >* output);

} // extern "C"

#endif // QT_GUI_C_QOPENGLTIMEMONITOR_H
