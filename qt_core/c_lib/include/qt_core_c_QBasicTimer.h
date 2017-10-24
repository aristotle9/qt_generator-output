#ifndef QT_CORE_C_QBASICTIMER_H
#define QT_CORE_C_QBASICTIMER_H

#include "qt_core_c_global.h"

extern "C" {

QT_CORE_C_EXPORT void qt_core_c_QBasicTimer_delete(QBasicTimer* this_ptr);
QT_CORE_C_EXPORT bool qt_core_c_QBasicTimer_isActive(const QBasicTimer* this_ptr);
QT_CORE_C_EXPORT QBasicTimer* qt_core_c_QBasicTimer_new();
QT_CORE_C_EXPORT void qt_core_c_QBasicTimer_start_msec_obj(QBasicTimer* this_ptr, int msec, QObject* obj);
QT_CORE_C_EXPORT void qt_core_c_QBasicTimer_start_msec_timerType_obj(QBasicTimer* this_ptr, int msec, const Qt::TimerType* timerType, QObject* obj);
QT_CORE_C_EXPORT void qt_core_c_QBasicTimer_stop(QBasicTimer* this_ptr);
QT_CORE_C_EXPORT int qt_core_c_QBasicTimer_timerId(const QBasicTimer* this_ptr);

} // extern "C"

#endif // QT_CORE_C_QBASICTIMER_H
