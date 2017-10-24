#ifndef QT_CORE_C_QTIME_H
#define QT_CORE_C_QTIME_H

#include "qt_core_c_global.h"

extern "C" {

QT_CORE_C_EXPORT void qt_core_c_QTime_addMSecs_to_output(const QTime* this_ptr, int ms, QTime* output);
QT_CORE_C_EXPORT void qt_core_c_QTime_addSecs_to_output(const QTime* this_ptr, int secs, QTime* output);
QT_CORE_C_EXPORT void qt_core_c_QTime_constructor_h_m(int h, int m, QTime* output);
QT_CORE_C_EXPORT void qt_core_c_QTime_constructor_h_m_s(int h, int m, int s, QTime* output);
QT_CORE_C_EXPORT void qt_core_c_QTime_constructor_h_m_s_ms(int h, int m, int s, int ms, QTime* output);
QT_CORE_C_EXPORT void qt_core_c_QTime_constructor_no_args(QTime* output);
QT_CORE_C_EXPORT void qt_core_c_QTime_currentTime_to_output(QTime* output);
QT_CORE_C_EXPORT void qt_core_c_QTime_destructor(QTime* this_ptr);
QT_CORE_C_EXPORT int qt_core_c_QTime_elapsed(const QTime* this_ptr);
QT_CORE_C_EXPORT void qt_core_c_QTime_fromMSecsSinceStartOfDay_to_output(int msecs, QTime* output);
QT_CORE_C_EXPORT void qt_core_c_QTime_fromString_to_output_s(const QString* s, QTime* output);
QT_CORE_C_EXPORT void qt_core_c_QTime_fromString_to_output_s_f(const QString* s, const Qt::DateFormat* f, QTime* output);
QT_CORE_C_EXPORT void qt_core_c_QTime_fromString_to_output_s_format(const QString* s, const QString* format, QTime* output);
QT_CORE_C_EXPORT int qt_core_c_QTime_hour(const QTime* this_ptr);
QT_CORE_C_EXPORT bool qt_core_c_QTime_isNull(const QTime* this_ptr);
QT_CORE_C_EXPORT bool qt_core_c_QTime_isValid_h_m_s(int h, int m, int s);
QT_CORE_C_EXPORT bool qt_core_c_QTime_isValid_h_m_s_ms(int h, int m, int s, int ms);
QT_CORE_C_EXPORT bool qt_core_c_QTime_isValid_no_args(const QTime* this_ptr);
QT_CORE_C_EXPORT int qt_core_c_QTime_minute(const QTime* this_ptr);
QT_CORE_C_EXPORT int qt_core_c_QTime_msec(const QTime* this_ptr);
QT_CORE_C_EXPORT int qt_core_c_QTime_msecsSinceStartOfDay(const QTime* this_ptr);
QT_CORE_C_EXPORT int qt_core_c_QTime_msecsTo(const QTime* this_ptr, const QTime* arg1);
QT_CORE_C_EXPORT bool qt_core_c_QTime_operator_eq(const QTime* this_ptr, const QTime* other);
QT_CORE_C_EXPORT bool qt_core_c_QTime_operator_ge(const QTime* this_ptr, const QTime* other);
QT_CORE_C_EXPORT bool qt_core_c_QTime_operator_gt(const QTime* this_ptr, const QTime* other);
QT_CORE_C_EXPORT bool qt_core_c_QTime_operator_le(const QTime* this_ptr, const QTime* other);
QT_CORE_C_EXPORT bool qt_core_c_QTime_operator_lt(const QTime* this_ptr, const QTime* other);
QT_CORE_C_EXPORT bool qt_core_c_QTime_operator_neq(const QTime* this_ptr, const QTime* other);
QT_CORE_C_EXPORT int qt_core_c_QTime_restart(QTime* this_ptr);
QT_CORE_C_EXPORT int qt_core_c_QTime_second(const QTime* this_ptr);
QT_CORE_C_EXPORT int qt_core_c_QTime_secsTo(const QTime* this_ptr, const QTime* arg1);
QT_CORE_C_EXPORT bool qt_core_c_QTime_setHMS_h_m_s(QTime* this_ptr, int h, int m, int s);
QT_CORE_C_EXPORT bool qt_core_c_QTime_setHMS_h_m_s_ms(QTime* this_ptr, int h, int m, int s, int ms);
QT_CORE_C_EXPORT void qt_core_c_QTime_start(QTime* this_ptr);
QT_CORE_C_EXPORT void qt_core_c_QTime_toString_to_output_f(const QTime* this_ptr, const Qt::DateFormat* f, QString* output);
QT_CORE_C_EXPORT void qt_core_c_QTime_toString_to_output_format(const QTime* this_ptr, const QString* format, QString* output);
QT_CORE_C_EXPORT void qt_core_c_QTime_toString_to_output_no_args(const QTime* this_ptr, QString* output);

} // extern "C"

#endif // QT_CORE_C_QTIME_H
