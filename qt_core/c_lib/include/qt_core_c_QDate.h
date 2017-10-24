#ifndef QT_CORE_C_QDATE_H
#define QT_CORE_C_QDATE_H

#include "qt_core_c_global.h"

extern "C" {

QT_CORE_C_EXPORT void qt_core_c_QDate_addDays_to_output(const QDate* this_ptr, qint64 days, QDate* output);
QT_CORE_C_EXPORT void qt_core_c_QDate_addMonths_to_output(const QDate* this_ptr, int months, QDate* output);
QT_CORE_C_EXPORT void qt_core_c_QDate_addYears_to_output(const QDate* this_ptr, int years, QDate* output);
QT_CORE_C_EXPORT void qt_core_c_QDate_constructor_no_args(QDate* output);
QT_CORE_C_EXPORT void qt_core_c_QDate_constructor_y_m_d(int y, int m, int d, QDate* output);
QT_CORE_C_EXPORT void qt_core_c_QDate_currentDate_to_output(QDate* output);
QT_CORE_C_EXPORT int qt_core_c_QDate_day(const QDate* this_ptr);
QT_CORE_C_EXPORT int qt_core_c_QDate_dayOfWeek(const QDate* this_ptr);
QT_CORE_C_EXPORT int qt_core_c_QDate_dayOfYear(const QDate* this_ptr);
QT_CORE_C_EXPORT int qt_core_c_QDate_daysInMonth(const QDate* this_ptr);
QT_CORE_C_EXPORT int qt_core_c_QDate_daysInYear(const QDate* this_ptr);
QT_CORE_C_EXPORT qint64 qt_core_c_QDate_daysTo(const QDate* this_ptr, const QDate* arg1);
QT_CORE_C_EXPORT void qt_core_c_QDate_destructor(QDate* this_ptr);
QT_CORE_C_EXPORT void qt_core_c_QDate_fromJulianDay_to_output(qint64 jd_, QDate* output);
QT_CORE_C_EXPORT void qt_core_c_QDate_fromString_to_output_s(const QString* s, QDate* output);
QT_CORE_C_EXPORT void qt_core_c_QDate_fromString_to_output_s_f(const QString* s, const Qt::DateFormat* f, QDate* output);
QT_CORE_C_EXPORT void qt_core_c_QDate_fromString_to_output_s_format(const QString* s, const QString* format, QDate* output);
QT_CORE_C_EXPORT void qt_core_c_QDate_getDate(QDate* this_ptr, int* year, int* month, int* day);
QT_CORE_C_EXPORT void qt_core_c_QDate_getDate_const(const QDate* this_ptr, int* year, int* month, int* day);
QT_CORE_C_EXPORT bool qt_core_c_QDate_isLeapYear(int year);
QT_CORE_C_EXPORT bool qt_core_c_QDate_isNull(const QDate* this_ptr);
QT_CORE_C_EXPORT bool qt_core_c_QDate_isValid_no_args(const QDate* this_ptr);
QT_CORE_C_EXPORT bool qt_core_c_QDate_isValid_y_m_d(int y, int m, int d);
QT_CORE_C_EXPORT void qt_core_c_QDate_longDayName_to_output_weekday(int weekday, QString* output);
QT_CORE_C_EXPORT void qt_core_c_QDate_longDayName_to_output_weekday_type(int weekday, QDate::MonthNameType type, QString* output);
QT_CORE_C_EXPORT void qt_core_c_QDate_longMonthName_to_output_month(int month, QString* output);
QT_CORE_C_EXPORT void qt_core_c_QDate_longMonthName_to_output_month_type(int month, QDate::MonthNameType type, QString* output);
QT_CORE_C_EXPORT int qt_core_c_QDate_month(const QDate* this_ptr);
QT_CORE_C_EXPORT bool qt_core_c_QDate_operator_eq(const QDate* this_ptr, const QDate* other);
QT_CORE_C_EXPORT bool qt_core_c_QDate_operator_ge(const QDate* this_ptr, const QDate* other);
QT_CORE_C_EXPORT bool qt_core_c_QDate_operator_gt(const QDate* this_ptr, const QDate* other);
QT_CORE_C_EXPORT bool qt_core_c_QDate_operator_le(const QDate* this_ptr, const QDate* other);
QT_CORE_C_EXPORT bool qt_core_c_QDate_operator_lt(const QDate* this_ptr, const QDate* other);
QT_CORE_C_EXPORT bool qt_core_c_QDate_operator_neq(const QDate* this_ptr, const QDate* other);
QT_CORE_C_EXPORT bool qt_core_c_QDate_setDate(QDate* this_ptr, int year, int month, int day);
QT_CORE_C_EXPORT void qt_core_c_QDate_shortDayName_to_output_weekday(int weekday, QString* output);
QT_CORE_C_EXPORT void qt_core_c_QDate_shortDayName_to_output_weekday_type(int weekday, QDate::MonthNameType type, QString* output);
QT_CORE_C_EXPORT void qt_core_c_QDate_shortMonthName_to_output_month(int month, QString* output);
QT_CORE_C_EXPORT void qt_core_c_QDate_shortMonthName_to_output_month_type(int month, QDate::MonthNameType type, QString* output);
QT_CORE_C_EXPORT qint64 qt_core_c_QDate_toJulianDay(const QDate* this_ptr);
QT_CORE_C_EXPORT void qt_core_c_QDate_toString_to_output_f(const QDate* this_ptr, const Qt::DateFormat* f, QString* output);
QT_CORE_C_EXPORT void qt_core_c_QDate_toString_to_output_format(const QDate* this_ptr, const QString* format, QString* output);
QT_CORE_C_EXPORT void qt_core_c_QDate_toString_to_output_no_args(const QDate* this_ptr, QString* output);
QT_CORE_C_EXPORT int qt_core_c_QDate_weekNumber_no_args(const QDate* this_ptr);
QT_CORE_C_EXPORT int qt_core_c_QDate_weekNumber_yearNum(const QDate* this_ptr, int* yearNum);
QT_CORE_C_EXPORT int qt_core_c_QDate_year(const QDate* this_ptr);

} // extern "C"

#endif // QT_CORE_C_QDATE_H
