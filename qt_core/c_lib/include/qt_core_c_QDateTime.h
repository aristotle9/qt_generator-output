#ifndef QT_CORE_C_QDATETIME_H
#define QT_CORE_C_QDATETIME_H

#include "qt_core_c_global.h"

extern "C" {

QT_CORE_C_EXPORT void qt_core_c_QDateTime_G_operator_shl_to_output_QDebug_QDate(const QDebug* arg1, const QDate* arg2, QDebug* output);
QT_CORE_C_EXPORT void qt_core_c_QDateTime_G_operator_shl_to_output_QDebug_QTime(const QDebug* arg1, const QTime* arg2, QDebug* output);
QT_CORE_C_EXPORT unsigned int qt_core_c_QDateTime_G_qHash_QDate(const QDate* key);
QT_CORE_C_EXPORT unsigned int qt_core_c_QDateTime_G_qHash_QDateTime(const QDateTime* key);
QT_CORE_C_EXPORT unsigned int qt_core_c_QDateTime_G_qHash_QDateTime_unsigned_int(const QDateTime* key, unsigned int seed);
QT_CORE_C_EXPORT unsigned int qt_core_c_QDateTime_G_qHash_QDate_unsigned_int(const QDate* key, unsigned int seed);
QT_CORE_C_EXPORT unsigned int qt_core_c_QDateTime_G_qHash_QTime(const QTime* key);
QT_CORE_C_EXPORT unsigned int qt_core_c_QDateTime_G_qHash_QTime_unsigned_int(const QTime* key, unsigned int seed);
QT_CORE_C_EXPORT void qt_core_c_QDateTime_G_swap(QDateTime* value1, QDateTime* value2);
QT_CORE_C_EXPORT void qt_core_c_QDateTime_addDays_to_output(const QDateTime* this_ptr, qint64 days, QDateTime* output);
QT_CORE_C_EXPORT void qt_core_c_QDateTime_addMSecs_to_output(const QDateTime* this_ptr, qint64 msecs, QDateTime* output);
QT_CORE_C_EXPORT void qt_core_c_QDateTime_addMonths_to_output(const QDateTime* this_ptr, int months, QDateTime* output);
QT_CORE_C_EXPORT void qt_core_c_QDateTime_addSecs_to_output(const QDateTime* this_ptr, qint64 secs, QDateTime* output);
QT_CORE_C_EXPORT void qt_core_c_QDateTime_addYears_to_output(const QDateTime* this_ptr, int years, QDateTime* output);
QT_CORE_C_EXPORT void qt_core_c_QDateTime_constructor_arg1(const QDate* arg1, QDateTime* output);
QT_CORE_C_EXPORT void qt_core_c_QDateTime_constructor_arg1_arg2(const QDate* arg1, const QTime* arg2, QDateTime* output);
QT_CORE_C_EXPORT void qt_core_c_QDateTime_constructor_arg1_arg2_spec(const QDate* arg1, const QTime* arg2, const Qt::TimeSpec* spec, QDateTime* output);
QT_CORE_C_EXPORT void qt_core_c_QDateTime_constructor_date_time_spec_offsetSeconds(const QDate* date, const QTime* time, const Qt::TimeSpec* spec, int offsetSeconds, QDateTime* output);
QT_CORE_C_EXPORT void qt_core_c_QDateTime_constructor_date_time_timeZone(const QDate* date, const QTime* time, const QTimeZone* timeZone, QDateTime* output);
QT_CORE_C_EXPORT void qt_core_c_QDateTime_constructor_no_args(QDateTime* output);
QT_CORE_C_EXPORT void qt_core_c_QDateTime_constructor_other(const QDateTime* other, QDateTime* output);
QT_CORE_C_EXPORT void qt_core_c_QDateTime_currentDateTimeUtc_to_output(QDateTime* output);
QT_CORE_C_EXPORT void qt_core_c_QDateTime_currentDateTime_to_output(QDateTime* output);
QT_CORE_C_EXPORT qint64 qt_core_c_QDateTime_currentMSecsSinceEpoch();
QT_CORE_C_EXPORT qint64 qt_core_c_QDateTime_currentSecsSinceEpoch();
QT_CORE_C_EXPORT void qt_core_c_QDateTime_date_to_output(const QDateTime* this_ptr, QDate* output);
QT_CORE_C_EXPORT qint64 qt_core_c_QDateTime_daysTo(const QDateTime* this_ptr, const QDateTime* arg1);
QT_CORE_C_EXPORT void qt_core_c_QDateTime_destructor(QDateTime* this_ptr);
QT_CORE_C_EXPORT void qt_core_c_QDateTime_fromMSecsSinceEpoch_to_output_msecs(qint64 msecs, QDateTime* output);
QT_CORE_C_EXPORT void qt_core_c_QDateTime_fromMSecsSinceEpoch_to_output_msecs_spec(qint64 msecs, const Qt::TimeSpec* spec, QDateTime* output);
QT_CORE_C_EXPORT void qt_core_c_QDateTime_fromMSecsSinceEpoch_to_output_msecs_spec_offsetFromUtc(qint64 msecs, const Qt::TimeSpec* spec, int offsetFromUtc, QDateTime* output);
QT_CORE_C_EXPORT void qt_core_c_QDateTime_fromMSecsSinceEpoch_to_output_msecs_timeZone(qint64 msecs, const QTimeZone* timeZone, QDateTime* output);
QT_CORE_C_EXPORT void qt_core_c_QDateTime_fromSecsSinceEpoch_to_output_secs(qint64 secs, QDateTime* output);
QT_CORE_C_EXPORT void qt_core_c_QDateTime_fromSecsSinceEpoch_to_output_secs_spe(qint64 secs, const Qt::TimeSpec* spe, QDateTime* output);
QT_CORE_C_EXPORT void qt_core_c_QDateTime_fromSecsSinceEpoch_to_output_secs_spe_offsetFromUtc(qint64 secs, const Qt::TimeSpec* spe, int offsetFromUtc, QDateTime* output);
QT_CORE_C_EXPORT void qt_core_c_QDateTime_fromSecsSinceEpoch_to_output_secs_timeZone(qint64 secs, const QTimeZone* timeZone, QDateTime* output);
QT_CORE_C_EXPORT void qt_core_c_QDateTime_fromString_to_output_s(const QString* s, QDateTime* output);
QT_CORE_C_EXPORT void qt_core_c_QDateTime_fromString_to_output_s_f(const QString* s, const Qt::DateFormat* f, QDateTime* output);
QT_CORE_C_EXPORT void qt_core_c_QDateTime_fromString_to_output_s_format(const QString* s, const QString* format, QDateTime* output);
QT_CORE_C_EXPORT void qt_core_c_QDateTime_fromTime_t_to_output_secsSince1Jan1970UTC(unsigned int secsSince1Jan1970UTC, QDateTime* output);
QT_CORE_C_EXPORT void qt_core_c_QDateTime_fromTime_t_to_output_secsSince1Jan1970UTC_spec(unsigned int secsSince1Jan1970UTC, const Qt::TimeSpec* spec, QDateTime* output);
QT_CORE_C_EXPORT void qt_core_c_QDateTime_fromTime_t_to_output_secsSince1Jan1970UTC_spec_offsetFromUtc(unsigned int secsSince1Jan1970UTC, const Qt::TimeSpec* spec, int offsetFromUtc, QDateTime* output);
QT_CORE_C_EXPORT void qt_core_c_QDateTime_fromTime_t_to_output_secsSince1Jan1970UTC_timeZone(unsigned int secsSince1Jan1970UTC, const QTimeZone* timeZone, QDateTime* output);
QT_CORE_C_EXPORT bool qt_core_c_QDateTime_isDaylightTime(const QDateTime* this_ptr);
QT_CORE_C_EXPORT bool qt_core_c_QDateTime_isNull(const QDateTime* this_ptr);
QT_CORE_C_EXPORT bool qt_core_c_QDateTime_isValid(const QDateTime* this_ptr);
QT_CORE_C_EXPORT qint64 qt_core_c_QDateTime_msecsTo(const QDateTime* this_ptr, const QDateTime* arg1);
QT_CORE_C_EXPORT int qt_core_c_QDateTime_offsetFromUtc(const QDateTime* this_ptr);
QT_CORE_C_EXPORT QDateTime* qt_core_c_QDateTime_operator_assign(QDateTime* this_ptr, const QDateTime* other);
QT_CORE_C_EXPORT bool qt_core_c_QDateTime_operator_eq(const QDateTime* this_ptr, const QDateTime* other);
QT_CORE_C_EXPORT bool qt_core_c_QDateTime_operator_ge(const QDateTime* this_ptr, const QDateTime* other);
QT_CORE_C_EXPORT bool qt_core_c_QDateTime_operator_gt(const QDateTime* this_ptr, const QDateTime* other);
QT_CORE_C_EXPORT bool qt_core_c_QDateTime_operator_le(const QDateTime* this_ptr, const QDateTime* other);
QT_CORE_C_EXPORT bool qt_core_c_QDateTime_operator_lt(const QDateTime* this_ptr, const QDateTime* other);
QT_CORE_C_EXPORT bool qt_core_c_QDateTime_operator_neq(const QDateTime* this_ptr, const QDateTime* other);
QT_CORE_C_EXPORT qint64 qt_core_c_QDateTime_secsTo(const QDateTime* this_ptr, const QDateTime* arg1);
QT_CORE_C_EXPORT void qt_core_c_QDateTime_setDate(QDateTime* this_ptr, const QDate* date);
QT_CORE_C_EXPORT void qt_core_c_QDateTime_setMSecsSinceEpoch(QDateTime* this_ptr, qint64 msecs);
QT_CORE_C_EXPORT void qt_core_c_QDateTime_setOffsetFromUtc(QDateTime* this_ptr, int offsetSeconds);
QT_CORE_C_EXPORT void qt_core_c_QDateTime_setSecsSinceEpoch(QDateTime* this_ptr, qint64 secs);
QT_CORE_C_EXPORT void qt_core_c_QDateTime_setTime(QDateTime* this_ptr, const QTime* time);
QT_CORE_C_EXPORT void qt_core_c_QDateTime_setTimeSpec(QDateTime* this_ptr, const Qt::TimeSpec* spec);
QT_CORE_C_EXPORT void qt_core_c_QDateTime_setTimeZone(QDateTime* this_ptr, const QTimeZone* toZone);
QT_CORE_C_EXPORT void qt_core_c_QDateTime_setTime_t(QDateTime* this_ptr, unsigned int secsSince1Jan1970UTC);
QT_CORE_C_EXPORT void qt_core_c_QDateTime_setUtcOffset(QDateTime* this_ptr, int seconds);
QT_CORE_C_EXPORT void qt_core_c_QDateTime_swap(QDateTime* this_ptr, QDateTime* other);
QT_CORE_C_EXPORT void qt_core_c_QDateTime_timeZoneAbbreviation_to_output(const QDateTime* this_ptr, QString* output);
QT_CORE_C_EXPORT void qt_core_c_QDateTime_timeZone_to_output(const QDateTime* this_ptr, QTimeZone* output);
QT_CORE_C_EXPORT void qt_core_c_QDateTime_time_to_output(const QDateTime* this_ptr, QTime* output);
QT_CORE_C_EXPORT void qt_core_c_QDateTime_toLocalTime_to_output(const QDateTime* this_ptr, QDateTime* output);
QT_CORE_C_EXPORT qint64 qt_core_c_QDateTime_toMSecsSinceEpoch(const QDateTime* this_ptr);
QT_CORE_C_EXPORT void qt_core_c_QDateTime_toOffsetFromUtc_to_output(const QDateTime* this_ptr, int offsetSeconds, QDateTime* output);
QT_CORE_C_EXPORT qint64 qt_core_c_QDateTime_toSecsSinceEpoch(const QDateTime* this_ptr);
QT_CORE_C_EXPORT void qt_core_c_QDateTime_toString_to_output_f(const QDateTime* this_ptr, const Qt::DateFormat* f, QString* output);
QT_CORE_C_EXPORT void qt_core_c_QDateTime_toString_to_output_format(const QDateTime* this_ptr, const QString* format, QString* output);
QT_CORE_C_EXPORT void qt_core_c_QDateTime_toString_to_output_no_args(const QDateTime* this_ptr, QString* output);
QT_CORE_C_EXPORT void qt_core_c_QDateTime_toTimeSpec_to_output(const QDateTime* this_ptr, const Qt::TimeSpec* spec, QDateTime* output);
QT_CORE_C_EXPORT void qt_core_c_QDateTime_toTimeZone_to_output(const QDateTime* this_ptr, const QTimeZone* toZone, QDateTime* output);
QT_CORE_C_EXPORT unsigned int qt_core_c_QDateTime_toTime_t(const QDateTime* this_ptr);
QT_CORE_C_EXPORT void qt_core_c_QDateTime_toUTC_to_output(const QDateTime* this_ptr, QDateTime* output);
QT_CORE_C_EXPORT int qt_core_c_QDateTime_utcOffset(const QDateTime* this_ptr);

} // extern "C"

#endif // QT_CORE_C_QDATETIME_H
