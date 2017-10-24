#ifndef QT_CORE_C_QTIMER_H
#define QT_CORE_C_QTIMER_H

#include "qt_core_c_global.h"

extern "C" {

QT_CORE_C_EXPORT QTimer* qt_core_c_QTimer_G_dynamic_cast_QTimer_ptr(QObject* ptr);
QT_CORE_C_EXPORT QObject* qt_core_c_QTimer_G_static_cast_QObject_ptr(QTimer* ptr);
QT_CORE_C_EXPORT QTimer* qt_core_c_QTimer_G_static_cast_QTimer_ptr(QObject* ptr);
QT_CORE_C_EXPORT void qt_core_c_QTimer_delete(QTimer* this_ptr);
QT_CORE_C_EXPORT int qt_core_c_QTimer_interval(const QTimer* this_ptr);
QT_CORE_C_EXPORT bool qt_core_c_QTimer_isActive(const QTimer* this_ptr);
QT_CORE_C_EXPORT bool qt_core_c_QTimer_isSingleShot(const QTimer* this_ptr);
QT_CORE_C_EXPORT const QMetaObject* qt_core_c_QTimer_metaObject(const QTimer* this_ptr);
QT_CORE_C_EXPORT QTimer* qt_core_c_QTimer_new_no_args();
QT_CORE_C_EXPORT QTimer* qt_core_c_QTimer_new_parent(QObject* parent);
QT_CORE_C_EXPORT int qt_core_c_QTimer_remainingTime(const QTimer* this_ptr);
QT_CORE_C_EXPORT void qt_core_c_QTimer_setInterval(QTimer* this_ptr, int msec);
QT_CORE_C_EXPORT void qt_core_c_QTimer_setSingleShot(QTimer* this_ptr, bool singleShot);
QT_CORE_C_EXPORT void qt_core_c_QTimer_setTimerType(QTimer* this_ptr, const Qt::TimerType* atype);
QT_CORE_C_EXPORT void qt_core_c_QTimer_singleShot_msec_receiver_member(int msec, const QObject* receiver, const char* member);
QT_CORE_C_EXPORT void qt_core_c_QTimer_singleShot_msec_timerType_receiver_member(int msec, const Qt::TimerType* timerType, const QObject* receiver, const char* member);
QT_CORE_C_EXPORT void qt_core_c_QTimer_start_msec(QTimer* this_ptr, int msec);
QT_CORE_C_EXPORT void qt_core_c_QTimer_start_no_args(QTimer* this_ptr);
QT_CORE_C_EXPORT void qt_core_c_QTimer_stop(QTimer* this_ptr);
QT_CORE_C_EXPORT int qt_core_c_QTimer_timerId(const QTimer* this_ptr);
QT_CORE_C_EXPORT void qt_core_c_QTimer_trUtf8_to_output(const char* s, const char* c, int n, QString* output);
QT_CORE_C_EXPORT void qt_core_c_QTimer_tr_to_output(const char* s, const char* c, int n, QString* output);

} // extern "C"

#endif // QT_CORE_C_QTIMER_H
