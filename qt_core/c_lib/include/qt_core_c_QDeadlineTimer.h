#ifndef QT_CORE_C_QDEADLINETIMER_H
#define QT_CORE_C_QDEADLINETIMER_H

#include "qt_core_c_global.h"

extern "C" {

QT_CORE_C_EXPORT void qt_core_c_QDeadlineTimer_G_swap(QDeadlineTimer* value1, QDeadlineTimer* value2);
QT_CORE_C_EXPORT void qt_core_c_QDeadlineTimer_addNSecs_to_output(const QDeadlineTimer* dt, qint64 nsecs, QDeadlineTimer* output);
QT_CORE_C_EXPORT void qt_core_c_QDeadlineTimer_constructor_arg1(QDeadlineTimer::ForeverConstant arg1, QDeadlineTimer* output);
QT_CORE_C_EXPORT void qt_core_c_QDeadlineTimer_constructor_arg1_type_(QDeadlineTimer::ForeverConstant arg1, const Qt::TimerType* type_, QDeadlineTimer* output);
QT_CORE_C_EXPORT void qt_core_c_QDeadlineTimer_constructor_msecs(qint64 msecs, QDeadlineTimer* output);
QT_CORE_C_EXPORT void qt_core_c_QDeadlineTimer_constructor_msecs_type(qint64 msecs, const Qt::TimerType* type, QDeadlineTimer* output);
QT_CORE_C_EXPORT void qt_core_c_QDeadlineTimer_constructor_no_args(QDeadlineTimer* output);
QT_CORE_C_EXPORT void qt_core_c_QDeadlineTimer_constructor_type_(const Qt::TimerType* type_, QDeadlineTimer* output);
QT_CORE_C_EXPORT void qt_core_c_QDeadlineTimer_current_to_output_no_args(QDeadlineTimer* output);
QT_CORE_C_EXPORT void qt_core_c_QDeadlineTimer_current_to_output_timerType(const Qt::TimerType* timerType, QDeadlineTimer* output);
QT_CORE_C_EXPORT qint64 qt_core_c_QDeadlineTimer_deadline(const QDeadlineTimer* this_ptr);
QT_CORE_C_EXPORT qint64 qt_core_c_QDeadlineTimer_deadlineNSecs(const QDeadlineTimer* this_ptr);
QT_CORE_C_EXPORT void qt_core_c_QDeadlineTimer_destructor(QDeadlineTimer* this_ptr);
QT_CORE_C_EXPORT bool qt_core_c_QDeadlineTimer_hasExpired(const QDeadlineTimer* this_ptr);
QT_CORE_C_EXPORT bool qt_core_c_QDeadlineTimer_isForever(const QDeadlineTimer* this_ptr);
QT_CORE_C_EXPORT QDeadlineTimer* qt_core_c_QDeadlineTimer_operator_add_assign(QDeadlineTimer* this_ptr, qint64 msecs);
QT_CORE_C_EXPORT QDeadlineTimer* qt_core_c_QDeadlineTimer_operator_sub_assign(QDeadlineTimer* this_ptr, qint64 msecs);
QT_CORE_C_EXPORT qint64 qt_core_c_QDeadlineTimer_remainingTime(const QDeadlineTimer* this_ptr);
QT_CORE_C_EXPORT qint64 qt_core_c_QDeadlineTimer_remainingTimeNSecs(const QDeadlineTimer* this_ptr);
QT_CORE_C_EXPORT void qt_core_c_QDeadlineTimer_setDeadline_msecs(QDeadlineTimer* this_ptr, qint64 msecs);
QT_CORE_C_EXPORT void qt_core_c_QDeadlineTimer_setDeadline_msecs_timerType(QDeadlineTimer* this_ptr, qint64 msecs, const Qt::TimerType* timerType);
QT_CORE_C_EXPORT void qt_core_c_QDeadlineTimer_setPreciseDeadline_secs(QDeadlineTimer* this_ptr, qint64 secs);
QT_CORE_C_EXPORT void qt_core_c_QDeadlineTimer_setPreciseDeadline_secs_nsecs(QDeadlineTimer* this_ptr, qint64 secs, qint64 nsecs);
QT_CORE_C_EXPORT void qt_core_c_QDeadlineTimer_setPreciseDeadline_secs_nsecs_type(QDeadlineTimer* this_ptr, qint64 secs, qint64 nsecs, const Qt::TimerType* type);
QT_CORE_C_EXPORT void qt_core_c_QDeadlineTimer_setPreciseRemainingTime_secs(QDeadlineTimer* this_ptr, qint64 secs);
QT_CORE_C_EXPORT void qt_core_c_QDeadlineTimer_setPreciseRemainingTime_secs_nsecs(QDeadlineTimer* this_ptr, qint64 secs, qint64 nsecs);
QT_CORE_C_EXPORT void qt_core_c_QDeadlineTimer_setPreciseRemainingTime_secs_nsecs_type(QDeadlineTimer* this_ptr, qint64 secs, qint64 nsecs, const Qt::TimerType* type);
QT_CORE_C_EXPORT void qt_core_c_QDeadlineTimer_setRemainingTime_msecs(QDeadlineTimer* this_ptr, qint64 msecs);
QT_CORE_C_EXPORT void qt_core_c_QDeadlineTimer_setRemainingTime_msecs_type(QDeadlineTimer* this_ptr, qint64 msecs, const Qt::TimerType* type);
QT_CORE_C_EXPORT void qt_core_c_QDeadlineTimer_setTimerType(QDeadlineTimer* this_ptr, const Qt::TimerType* type);
QT_CORE_C_EXPORT void qt_core_c_QDeadlineTimer_swap(QDeadlineTimer* this_ptr, QDeadlineTimer* other);

} // extern "C"

#endif // QT_CORE_C_QDEADLINETIMER_H
