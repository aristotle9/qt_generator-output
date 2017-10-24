#ifndef QT_CORE_C_QTHREAD_H
#define QT_CORE_C_QTHREAD_H

#include "qt_core_c_global.h"

extern "C" {

QT_CORE_C_EXPORT QThread* qt_core_c_QThread_G_dynamic_cast_QThread_ptr(QObject* ptr);
QT_CORE_C_EXPORT QObject* qt_core_c_QThread_G_static_cast_QObject_ptr(QThread* ptr);
QT_CORE_C_EXPORT QThread* qt_core_c_QThread_G_static_cast_QThread_ptr(QObject* ptr);
QT_CORE_C_EXPORT QThread* qt_core_c_QThread_currentThread();
QT_CORE_C_EXPORT void qt_core_c_QThread_delete(QThread* this_ptr);
QT_CORE_C_EXPORT bool qt_core_c_QThread_event(QThread* this_ptr, QEvent* event);
QT_CORE_C_EXPORT QAbstractEventDispatcher* qt_core_c_QThread_eventDispatcher(const QThread* this_ptr);
QT_CORE_C_EXPORT void qt_core_c_QThread_exit_no_args(QThread* this_ptr);
QT_CORE_C_EXPORT void qt_core_c_QThread_exit_retcode(QThread* this_ptr, int retcode);
QT_CORE_C_EXPORT int qt_core_c_QThread_idealThreadCount();
QT_CORE_C_EXPORT bool qt_core_c_QThread_isFinished(const QThread* this_ptr);
QT_CORE_C_EXPORT bool qt_core_c_QThread_isInterruptionRequested(const QThread* this_ptr);
QT_CORE_C_EXPORT bool qt_core_c_QThread_isRunning(const QThread* this_ptr);
QT_CORE_C_EXPORT int qt_core_c_QThread_loopLevel(const QThread* this_ptr);
QT_CORE_C_EXPORT const QMetaObject* qt_core_c_QThread_metaObject(const QThread* this_ptr);
QT_CORE_C_EXPORT void qt_core_c_QThread_msleep(unsigned long arg1);
QT_CORE_C_EXPORT QThread* qt_core_c_QThread_new_no_args();
QT_CORE_C_EXPORT QThread* qt_core_c_QThread_new_parent(QObject* parent);
QT_CORE_C_EXPORT QThread::Priority qt_core_c_QThread_priority(const QThread* this_ptr);
QT_CORE_C_EXPORT void qt_core_c_QThread_quit(QThread* this_ptr);
QT_CORE_C_EXPORT void qt_core_c_QThread_requestInterruption(QThread* this_ptr);
QT_CORE_C_EXPORT void qt_core_c_QThread_setEventDispatcher(QThread* this_ptr, QAbstractEventDispatcher* eventDispatcher);
QT_CORE_C_EXPORT void qt_core_c_QThread_setPriority(QThread* this_ptr, QThread::Priority priority);
QT_CORE_C_EXPORT void qt_core_c_QThread_setStackSize(QThread* this_ptr, unsigned int stackSize);
QT_CORE_C_EXPORT void qt_core_c_QThread_sleep(unsigned long arg1);
QT_CORE_C_EXPORT unsigned int qt_core_c_QThread_stackSize(const QThread* this_ptr);
QT_CORE_C_EXPORT void qt_core_c_QThread_start_arg1(QThread* this_ptr, QThread::Priority arg1);
QT_CORE_C_EXPORT void qt_core_c_QThread_start_no_args(QThread* this_ptr);
QT_CORE_C_EXPORT void qt_core_c_QThread_terminate(QThread* this_ptr);
QT_CORE_C_EXPORT void qt_core_c_QThread_trUtf8_to_output(const char* s, const char* c, int n, QString* output);
QT_CORE_C_EXPORT void qt_core_c_QThread_tr_to_output(const char* s, const char* c, int n, QString* output);
QT_CORE_C_EXPORT void qt_core_c_QThread_usleep(unsigned long arg1);
QT_CORE_C_EXPORT bool qt_core_c_QThread_wait_no_args(QThread* this_ptr);
QT_CORE_C_EXPORT bool qt_core_c_QThread_wait_time(QThread* this_ptr, unsigned long time);
QT_CORE_C_EXPORT void qt_core_c_QThread_yieldCurrentThread();

} // extern "C"

#endif // QT_CORE_C_QTHREAD_H
