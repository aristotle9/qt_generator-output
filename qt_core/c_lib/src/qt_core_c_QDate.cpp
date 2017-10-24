#include "qt_core_c_QDate.h"

void qt_core_c_QDate_addDays_to_output(const QDate* this_ptr, qint64 days, QDate* output) {
  new(output) QDate(this_ptr->addDays(days));
}

void qt_core_c_QDate_addMonths_to_output(const QDate* this_ptr, int months, QDate* output) {
  new(output) QDate(this_ptr->addMonths(months));
}

void qt_core_c_QDate_addYears_to_output(const QDate* this_ptr, int years, QDate* output) {
  new(output) QDate(this_ptr->addYears(years));
}

void qt_core_c_QDate_constructor_no_args(QDate* output) {
  new(output) QDate();
}

void qt_core_c_QDate_constructor_y_m_d(int y, int m, int d, QDate* output) {
  new(output) QDate(y, m, d);
}

void qt_core_c_QDate_currentDate_to_output(QDate* output) {
  new(output) QDate(QDate::currentDate());
}

int qt_core_c_QDate_day(const QDate* this_ptr) {
  return this_ptr->day();
}

int qt_core_c_QDate_dayOfWeek(const QDate* this_ptr) {
  return this_ptr->dayOfWeek();
}

int qt_core_c_QDate_dayOfYear(const QDate* this_ptr) {
  return this_ptr->dayOfYear();
}

int qt_core_c_QDate_daysInMonth(const QDate* this_ptr) {
  return this_ptr->daysInMonth();
}

int qt_core_c_QDate_daysInYear(const QDate* this_ptr) {
  return this_ptr->daysInYear();
}

qint64 qt_core_c_QDate_daysTo(const QDate* this_ptr, const QDate* arg1) {
  return this_ptr->daysTo(*arg1);
}

void qt_core_c_QDate_destructor(QDate* this_ptr) {
  qt_core_c_call_destructor(this_ptr);
}

void qt_core_c_QDate_fromJulianDay_to_output(qint64 jd_, QDate* output) {
  new(output) QDate(QDate::fromJulianDay(jd_));
}

void qt_core_c_QDate_fromString_to_output_s(const QString* s, QDate* output) {
  new(output) QDate(QDate::fromString(*s));
}

void qt_core_c_QDate_fromString_to_output_s_f(const QString* s, const Qt::DateFormat* f, QDate* output) {
  new(output) QDate(QDate::fromString(*s, *f));
}

void qt_core_c_QDate_fromString_to_output_s_format(const QString* s, const QString* format, QDate* output) {
  new(output) QDate(QDate::fromString(*s, *format));
}

void qt_core_c_QDate_getDate(QDate* this_ptr, int* year, int* month, int* day) {
  this_ptr->getDate(year, month, day);
}

void qt_core_c_QDate_getDate_const(const QDate* this_ptr, int* year, int* month, int* day) {
  this_ptr->getDate(year, month, day);
}

bool qt_core_c_QDate_isLeapYear(int year) {
  return QDate::isLeapYear(year);
}

bool qt_core_c_QDate_isNull(const QDate* this_ptr) {
  return this_ptr->isNull();
}

bool qt_core_c_QDate_isValid_no_args(const QDate* this_ptr) {
  return this_ptr->isValid();
}

bool qt_core_c_QDate_isValid_y_m_d(int y, int m, int d) {
  return QDate::isValid(y, m, d);
}

void qt_core_c_QDate_longDayName_to_output_weekday(int weekday, QString* output) {
  new(output) QString(QDate::longDayName(weekday));
}

void qt_core_c_QDate_longDayName_to_output_weekday_type(int weekday, QDate::MonthNameType type, QString* output) {
  new(output) QString(QDate::longDayName(weekday, type));
}

void qt_core_c_QDate_longMonthName_to_output_month(int month, QString* output) {
  new(output) QString(QDate::longMonthName(month));
}

void qt_core_c_QDate_longMonthName_to_output_month_type(int month, QDate::MonthNameType type, QString* output) {
  new(output) QString(QDate::longMonthName(month, type));
}

int qt_core_c_QDate_month(const QDate* this_ptr) {
  return this_ptr->month();
}

bool qt_core_c_QDate_operator_eq(const QDate* this_ptr, const QDate* other) {
  return this_ptr->operator==(*other);
}

bool qt_core_c_QDate_operator_ge(const QDate* this_ptr, const QDate* other) {
  return this_ptr->operator>=(*other);
}

bool qt_core_c_QDate_operator_gt(const QDate* this_ptr, const QDate* other) {
  return this_ptr->operator>(*other);
}

bool qt_core_c_QDate_operator_le(const QDate* this_ptr, const QDate* other) {
  return this_ptr->operator<=(*other);
}

bool qt_core_c_QDate_operator_lt(const QDate* this_ptr, const QDate* other) {
  return this_ptr->operator<(*other);
}

bool qt_core_c_QDate_operator_neq(const QDate* this_ptr, const QDate* other) {
  return this_ptr->operator!=(*other);
}

bool qt_core_c_QDate_setDate(QDate* this_ptr, int year, int month, int day) {
  return this_ptr->setDate(year, month, day);
}

void qt_core_c_QDate_shortDayName_to_output_weekday(int weekday, QString* output) {
  new(output) QString(QDate::shortDayName(weekday));
}

void qt_core_c_QDate_shortDayName_to_output_weekday_type(int weekday, QDate::MonthNameType type, QString* output) {
  new(output) QString(QDate::shortDayName(weekday, type));
}

void qt_core_c_QDate_shortMonthName_to_output_month(int month, QString* output) {
  new(output) QString(QDate::shortMonthName(month));
}

void qt_core_c_QDate_shortMonthName_to_output_month_type(int month, QDate::MonthNameType type, QString* output) {
  new(output) QString(QDate::shortMonthName(month, type));
}

qint64 qt_core_c_QDate_toJulianDay(const QDate* this_ptr) {
  return this_ptr->toJulianDay();
}

void qt_core_c_QDate_toString_to_output_f(const QDate* this_ptr, const Qt::DateFormat* f, QString* output) {
  new(output) QString(this_ptr->toString(*f));
}

void qt_core_c_QDate_toString_to_output_format(const QDate* this_ptr, const QString* format, QString* output) {
  new(output) QString(this_ptr->toString(*format));
}

void qt_core_c_QDate_toString_to_output_no_args(const QDate* this_ptr, QString* output) {
  new(output) QString(this_ptr->toString());
}

int qt_core_c_QDate_weekNumber_no_args(const QDate* this_ptr) {
  return this_ptr->weekNumber();
}

int qt_core_c_QDate_weekNumber_yearNum(const QDate* this_ptr, int* yearNum) {
  return this_ptr->weekNumber(yearNum);
}

int qt_core_c_QDate_year(const QDate* this_ptr) {
  return this_ptr->year();
}

