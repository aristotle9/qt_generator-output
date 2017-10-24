#ifndef QT_CORE_C_QELAPSEDTIMER_H
#define QT_CORE_C_QELAPSEDTIMER_H

#include "qt_core_c_global.h"

extern "C" {

QT_CORE_C_EXPORT QElapsedTimer::ClockType qt_core_c_QElapsedTimer_clockType();
QT_CORE_C_EXPORT void qt_core_c_QElapsedTimer_constructor(QElapsedTimer* output);
QT_CORE_C_EXPORT void qt_core_c_QElapsedTimer_destructor(QElapsedTimer* this_ptr);
QT_CORE_C_EXPORT qint64 qt_core_c_QElapsedTimer_elapsed(const QElapsedTimer* this_ptr);
QT_CORE_C_EXPORT bool qt_core_c_QElapsedTimer_hasExpired(const QElapsedTimer* this_ptr, qint64 timeout);
QT_CORE_C_EXPORT void qt_core_c_QElapsedTimer_invalidate(QElapsedTimer* this_ptr);
QT_CORE_C_EXPORT bool qt_core_c_QElapsedTimer_isMonotonic();
QT_CORE_C_EXPORT bool qt_core_c_QElapsedTimer_isValid(const QElapsedTimer* this_ptr);
QT_CORE_C_EXPORT qint64 qt_core_c_QElapsedTimer_msecsSinceReference(const QElapsedTimer* this_ptr);
QT_CORE_C_EXPORT qint64 qt_core_c_QElapsedTimer_msecsTo(const QElapsedTimer* this_ptr, const QElapsedTimer* other);
QT_CORE_C_EXPORT qint64 qt_core_c_QElapsedTimer_nsecsElapsed(const QElapsedTimer* this_ptr);
QT_CORE_C_EXPORT bool qt_core_c_QElapsedTimer_operator_eq(const QElapsedTimer* this_ptr, const QElapsedTimer* other);
QT_CORE_C_EXPORT bool qt_core_c_QElapsedTimer_operator_neq(const QElapsedTimer* this_ptr, const QElapsedTimer* other);
QT_CORE_C_EXPORT qint64 qt_core_c_QElapsedTimer_restart(QElapsedTimer* this_ptr);
QT_CORE_C_EXPORT qint64 qt_core_c_QElapsedTimer_secsTo(const QElapsedTimer* this_ptr, const QElapsedTimer* other);
QT_CORE_C_EXPORT void qt_core_c_QElapsedTimer_start(QElapsedTimer* this_ptr);

} // extern "C"

#endif // QT_CORE_C_QELAPSEDTIMER_H
