#ifndef QT_CORE_C_QTHREADPOOL_H
#define QT_CORE_C_QTHREADPOOL_H

#include "qt_core_c_global.h"

extern "C" {

QT_CORE_C_EXPORT QThreadPool* qt_core_c_QThreadPool_G_dynamic_cast_QThreadPool_ptr(QObject* ptr);
QT_CORE_C_EXPORT QObject* qt_core_c_QThreadPool_G_static_cast_QObject_ptr(QThreadPool* ptr);
QT_CORE_C_EXPORT QThreadPool* qt_core_c_QThreadPool_G_static_cast_QThreadPool_ptr(QObject* ptr);
QT_CORE_C_EXPORT int qt_core_c_QThreadPool_activeThreadCount(const QThreadPool* this_ptr);
QT_CORE_C_EXPORT void qt_core_c_QThreadPool_cancel(QThreadPool* this_ptr, QRunnable* runnable);
QT_CORE_C_EXPORT void qt_core_c_QThreadPool_clear(QThreadPool* this_ptr);
QT_CORE_C_EXPORT void qt_core_c_QThreadPool_delete(QThreadPool* this_ptr);
QT_CORE_C_EXPORT int qt_core_c_QThreadPool_expiryTimeout(const QThreadPool* this_ptr);
QT_CORE_C_EXPORT QThreadPool* qt_core_c_QThreadPool_globalInstance();
QT_CORE_C_EXPORT int qt_core_c_QThreadPool_maxThreadCount(const QThreadPool* this_ptr);
QT_CORE_C_EXPORT const QMetaObject* qt_core_c_QThreadPool_metaObject(const QThreadPool* this_ptr);
QT_CORE_C_EXPORT QThreadPool* qt_core_c_QThreadPool_new_no_args();
QT_CORE_C_EXPORT QThreadPool* qt_core_c_QThreadPool_new_parent(QObject* parent);
QT_CORE_C_EXPORT void qt_core_c_QThreadPool_releaseThread(QThreadPool* this_ptr);
QT_CORE_C_EXPORT void qt_core_c_QThreadPool_reserveThread(QThreadPool* this_ptr);
QT_CORE_C_EXPORT void qt_core_c_QThreadPool_setExpiryTimeout(QThreadPool* this_ptr, int expiryTimeout);
QT_CORE_C_EXPORT void qt_core_c_QThreadPool_setMaxThreadCount(QThreadPool* this_ptr, int maxThreadCount);
QT_CORE_C_EXPORT void qt_core_c_QThreadPool_start_runnable(QThreadPool* this_ptr, QRunnable* runnable);
QT_CORE_C_EXPORT void qt_core_c_QThreadPool_start_runnable_priority(QThreadPool* this_ptr, QRunnable* runnable, int priority);
QT_CORE_C_EXPORT void qt_core_c_QThreadPool_trUtf8_to_output(const char* s, const char* c, int n, QString* output);
QT_CORE_C_EXPORT void qt_core_c_QThreadPool_tr_to_output(const char* s, const char* c, int n, QString* output);
QT_CORE_C_EXPORT bool qt_core_c_QThreadPool_tryStart(QThreadPool* this_ptr, QRunnable* runnable);
QT_CORE_C_EXPORT bool qt_core_c_QThreadPool_tryTake(QThreadPool* this_ptr, QRunnable* runnable);
QT_CORE_C_EXPORT bool qt_core_c_QThreadPool_waitForDone_msecs(QThreadPool* this_ptr, int msecs);
QT_CORE_C_EXPORT bool qt_core_c_QThreadPool_waitForDone_no_args(QThreadPool* this_ptr);

} // extern "C"

#endif // QT_CORE_C_QTHREADPOOL_H
