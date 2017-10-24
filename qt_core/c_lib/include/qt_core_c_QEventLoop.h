#ifndef QT_CORE_C_QEVENTLOOP_H
#define QT_CORE_C_QEVENTLOOP_H

#include "qt_core_c_global.h"

extern "C" {

QT_CORE_C_EXPORT QEventLoop* qt_core_c_QEventLoop_G_dynamic_cast_QEventLoop_ptr(QObject* ptr);
QT_CORE_C_EXPORT QEventLoop* qt_core_c_QEventLoop_G_static_cast_QEventLoop_ptr(QObject* ptr);
QT_CORE_C_EXPORT QObject* qt_core_c_QEventLoop_G_static_cast_QObject_ptr(QEventLoop* ptr);
QT_CORE_C_EXPORT void qt_core_c_QEventLoop_delete(QEventLoop* this_ptr);
QT_CORE_C_EXPORT bool qt_core_c_QEventLoop_event(QEventLoop* this_ptr, QEvent* event);
QT_CORE_C_EXPORT int qt_core_c_QEventLoop_exec_flags(QEventLoop* this_ptr, unsigned int flags);
QT_CORE_C_EXPORT int qt_core_c_QEventLoop_exec_no_args(QEventLoop* this_ptr);
QT_CORE_C_EXPORT void qt_core_c_QEventLoop_exit_no_args(QEventLoop* this_ptr);
QT_CORE_C_EXPORT void qt_core_c_QEventLoop_exit_returnCode(QEventLoop* this_ptr, int returnCode);
QT_CORE_C_EXPORT bool qt_core_c_QEventLoop_isRunning(const QEventLoop* this_ptr);
QT_CORE_C_EXPORT const QMetaObject* qt_core_c_QEventLoop_metaObject(const QEventLoop* this_ptr);
QT_CORE_C_EXPORT QEventLoop* qt_core_c_QEventLoop_new_no_args();
QT_CORE_C_EXPORT QEventLoop* qt_core_c_QEventLoop_new_parent(QObject* parent);
QT_CORE_C_EXPORT bool qt_core_c_QEventLoop_processEvents_flags(QEventLoop* this_ptr, unsigned int flags);
QT_CORE_C_EXPORT void qt_core_c_QEventLoop_processEvents_flags_maximumTime(QEventLoop* this_ptr, unsigned int flags, int maximumTime);
QT_CORE_C_EXPORT bool qt_core_c_QEventLoop_processEvents_no_args(QEventLoop* this_ptr);
QT_CORE_C_EXPORT void qt_core_c_QEventLoop_quit(QEventLoop* this_ptr);
QT_CORE_C_EXPORT void qt_core_c_QEventLoop_trUtf8_to_output(const char* s, const char* c, int n, QString* output);
QT_CORE_C_EXPORT void qt_core_c_QEventLoop_tr_to_output(const char* s, const char* c, int n, QString* output);
QT_CORE_C_EXPORT void qt_core_c_QEventLoop_wakeUp(QEventLoop* this_ptr);

} // extern "C"

#endif // QT_CORE_C_QEVENTLOOP_H
