#include "qt_core_c_QDateTime.h"

void qt_core_c_QDateTime_G_operator_shl_to_output_QDebug_QDate(const QDebug* arg1, const QDate* arg2, QDebug* output) {
  new(output) QDebug(operator<<(*arg1, *arg2));
}

void qt_core_c_QDateTime_G_operator_shl_to_output_QDebug_QTime(const QDebug* arg1, const QTime* arg2, QDebug* output) {
  new(output) QDebug(operator<<(*arg1, *arg2));
}

unsigned int qt_core_c_QDateTime_G_qHash_QDate(const QDate* key) {
  return qHash(*key);
}

unsigned int qt_core_c_QDateTime_G_qHash_QDateTime(const QDateTime* key) {
  return qHash(*key);
}

unsigned int qt_core_c_QDateTime_G_qHash_QDateTime_unsigned_int(const QDateTime* key, unsigned int seed) {
  return qHash(*key, seed);
}

unsigned int qt_core_c_QDateTime_G_qHash_QDate_unsigned_int(const QDate* key, unsigned int seed) {
  return qHash(*key, seed);
}

unsigned int qt_core_c_QDateTime_G_qHash_QTime(const QTime* key) {
  return qHash(*key);
}

unsigned int qt_core_c_QDateTime_G_qHash_QTime_unsigned_int(const QTime* key, unsigned int seed) {
  return qHash(*key, seed);
}

void qt_core_c_QDateTime_G_swap(QDateTime* value1, QDateTime* value2) {
  swap(*value1, *value2);
}

void qt_core_c_QDateTime_addDays_to_output(const QDateTime* this_ptr, qint64 days, QDateTime* output) {
  new(output) QDateTime(this_ptr->addDays(days));
}

void qt_core_c_QDateTime_addMSecs_to_output(const QDateTime* this_ptr, qint64 msecs, QDateTime* output) {
  new(output) QDateTime(this_ptr->addMSecs(msecs));
}

void qt_core_c_QDateTime_addMonths_to_output(const QDateTime* this_ptr, int months, QDateTime* output) {
  new(output) QDateTime(this_ptr->addMonths(months));
}

void qt_core_c_QDateTime_addSecs_to_output(const QDateTime* this_ptr, qint64 secs, QDateTime* output) {
  new(output) QDateTime(this_ptr->addSecs(secs));
}

void qt_core_c_QDateTime_addYears_to_output(const QDateTime* this_ptr, int years, QDateTime* output) {
  new(output) QDateTime(this_ptr->addYears(years));
}

void qt_core_c_QDateTime_constructor_arg1(const QDate* arg1, QDateTime* output) {
  new(output) QDateTime(*arg1);
}

void qt_core_c_QDateTime_constructor_arg1_arg2(const QDate* arg1, const QTime* arg2, QDateTime* output) {
  new(output) QDateTime(*arg1, *arg2);
}

void qt_core_c_QDateTime_constructor_arg1_arg2_spec(const QDate* arg1, const QTime* arg2, const Qt::TimeSpec* spec, QDateTime* output) {
  new(output) QDateTime(*arg1, *arg2, *spec);
}

void qt_core_c_QDateTime_constructor_date_time_spec_offsetSeconds(const QDate* date, const QTime* time, const Qt::TimeSpec* spec, int offsetSeconds, QDateTime* output) {
  new(output) QDateTime(*date, *time, *spec, offsetSeconds);
}

void qt_core_c_QDateTime_constructor_date_time_timeZone(const QDate* date, const QTime* time, const QTimeZone* timeZone, QDateTime* output) {
  new(output) QDateTime(*date, *time, *timeZone);
}

void qt_core_c_QDateTime_constructor_no_args(QDateTime* output) {
  new(output) QDateTime();
}

void qt_core_c_QDateTime_constructor_other(const QDateTime* other, QDateTime* output) {
  new(output) QDateTime(*other);
}

void qt_core_c_QDateTime_currentDateTimeUtc_to_output(QDateTime* output) {
  new(output) QDateTime(QDateTime::currentDateTimeUtc());
}

void qt_core_c_QDateTime_currentDateTime_to_output(QDateTime* output) {
  new(output) QDateTime(QDateTime::currentDateTime());
}

qint64 qt_core_c_QDateTime_currentMSecsSinceEpoch() {
  return QDateTime::currentMSecsSinceEpoch();
}

qint64 qt_core_c_QDateTime_currentSecsSinceEpoch() {
  return QDateTime::currentSecsSinceEpoch();
}

void qt_core_c_QDateTime_date_to_output(const QDateTime* this_ptr, QDate* output) {
  new(output) QDate(this_ptr->date());
}

qint64 qt_core_c_QDateTime_daysTo(const QDateTime* this_ptr, const QDateTime* arg1) {
  return this_ptr->daysTo(*arg1);
}

void qt_core_c_QDateTime_destructor(QDateTime* this_ptr) {
  qt_core_c_call_destructor(this_ptr);
}

void qt_core_c_QDateTime_fromMSecsSinceEpoch_to_output_msecs(qint64 msecs, QDateTime* output) {
  new(output) QDateTime(QDateTime::fromMSecsSinceEpoch(msecs));
}

void qt_core_c_QDateTime_fromMSecsSinceEpoch_to_output_msecs_spec(qint64 msecs, const Qt::TimeSpec* spec, QDateTime* output) {
  new(output) QDateTime(QDateTime::fromMSecsSinceEpoch(msecs, *spec));
}

void qt_core_c_QDateTime_fromMSecsSinceEpoch_to_output_msecs_spec_offsetFromUtc(qint64 msecs, const Qt::TimeSpec* spec, int offsetFromUtc, QDateTime* output) {
  new(output) QDateTime(QDateTime::fromMSecsSinceEpoch(msecs, *spec, offsetFromUtc));
}

void qt_core_c_QDateTime_fromMSecsSinceEpoch_to_output_msecs_timeZone(qint64 msecs, const QTimeZone* timeZone, QDateTime* output) {
  new(output) QDateTime(QDateTime::fromMSecsSinceEpoch(msecs, *timeZone));
}

void qt_core_c_QDateTime_fromSecsSinceEpoch_to_output_secs(qint64 secs, QDateTime* output) {
  new(output) QDateTime(QDateTime::fromSecsSinceEpoch(secs));
}

void qt_core_c_QDateTime_fromSecsSinceEpoch_to_output_secs_spe(qint64 secs, const Qt::TimeSpec* spe, QDateTime* output) {
  new(output) QDateTime(QDateTime::fromSecsSinceEpoch(secs, *spe));
}

void qt_core_c_QDateTime_fromSecsSinceEpoch_to_output_secs_spe_offsetFromUtc(qint64 secs, const Qt::TimeSpec* spe, int offsetFromUtc, QDateTime* output) {
  new(output) QDateTime(QDateTime::fromSecsSinceEpoch(secs, *spe, offsetFromUtc));
}

void qt_core_c_QDateTime_fromSecsSinceEpoch_to_output_secs_timeZone(qint64 secs, const QTimeZone* timeZone, QDateTime* output) {
  new(output) QDateTime(QDateTime::fromSecsSinceEpoch(secs, *timeZone));
}

void qt_core_c_QDateTime_fromString_to_output_s(const QString* s, QDateTime* output) {
  new(output) QDateTime(QDateTime::fromString(*s));
}

void qt_core_c_QDateTime_fromString_to_output_s_f(const QString* s, const Qt::DateFormat* f, QDateTime* output) {
  new(output) QDateTime(QDateTime::fromString(*s, *f));
}

void qt_core_c_QDateTime_fromString_to_output_s_format(const QString* s, const QString* format, QDateTime* output) {
  new(output) QDateTime(QDateTime::fromString(*s, *format));
}

void qt_core_c_QDateTime_fromTime_t_to_output_secsSince1Jan1970UTC(unsigned int secsSince1Jan1970UTC, QDateTime* output) {
  new(output) QDateTime(QDateTime::fromTime_t(secsSince1Jan1970UTC));
}

void qt_core_c_QDateTime_fromTime_t_to_output_secsSince1Jan1970UTC_spec(unsigned int secsSince1Jan1970UTC, const Qt::TimeSpec* spec, QDateTime* output) {
  new(output) QDateTime(QDateTime::fromTime_t(secsSince1Jan1970UTC, *spec));
}

void qt_core_c_QDateTime_fromTime_t_to_output_secsSince1Jan1970UTC_spec_offsetFromUtc(unsigned int secsSince1Jan1970UTC, const Qt::TimeSpec* spec, int offsetFromUtc, QDateTime* output) {
  new(output) QDateTime(QDateTime::fromTime_t(secsSince1Jan1970UTC, *spec, offsetFromUtc));
}

void qt_core_c_QDateTime_fromTime_t_to_output_secsSince1Jan1970UTC_timeZone(unsigned int secsSince1Jan1970UTC, const QTimeZone* timeZone, QDateTime* output) {
  new(output) QDateTime(QDateTime::fromTime_t(secsSince1Jan1970UTC, *timeZone));
}

bool qt_core_c_QDateTime_isDaylightTime(const QDateTime* this_ptr) {
  return this_ptr->isDaylightTime();
}

bool qt_core_c_QDateTime_isNull(const QDateTime* this_ptr) {
  return this_ptr->isNull();
}

bool qt_core_c_QDateTime_isValid(const QDateTime* this_ptr) {
  return this_ptr->isValid();
}

qint64 qt_core_c_QDateTime_msecsTo(const QDateTime* this_ptr, const QDateTime* arg1) {
  return this_ptr->msecsTo(*arg1);
}

int qt_core_c_QDateTime_offsetFromUtc(const QDateTime* this_ptr) {
  return this_ptr->offsetFromUtc();
}

QDateTime* qt_core_c_QDateTime_operator_assign(QDateTime* this_ptr, const QDateTime* other) {
  return &this_ptr->operator=(*other);
}

bool qt_core_c_QDateTime_operator_eq(const QDateTime* this_ptr, const QDateTime* other) {
  return this_ptr->operator==(*other);
}

bool qt_core_c_QDateTime_operator_ge(const QDateTime* this_ptr, const QDateTime* other) {
  return this_ptr->operator>=(*other);
}

bool qt_core_c_QDateTime_operator_gt(const QDateTime* this_ptr, const QDateTime* other) {
  return this_ptr->operator>(*other);
}

bool qt_core_c_QDateTime_operator_le(const QDateTime* this_ptr, const QDateTime* other) {
  return this_ptr->operator<=(*other);
}

bool qt_core_c_QDateTime_operator_lt(const QDateTime* this_ptr, const QDateTime* other) {
  return this_ptr->operator<(*other);
}

bool qt_core_c_QDateTime_operator_neq(const QDateTime* this_ptr, const QDateTime* other) {
  return this_ptr->operator!=(*other);
}

qint64 qt_core_c_QDateTime_secsTo(const QDateTime* this_ptr, const QDateTime* arg1) {
  return this_ptr->secsTo(*arg1);
}

void qt_core_c_QDateTime_setDate(QDateTime* this_ptr, const QDate* date) {
  this_ptr->setDate(*date);
}

void qt_core_c_QDateTime_setMSecsSinceEpoch(QDateTime* this_ptr, qint64 msecs) {
  this_ptr->setMSecsSinceEpoch(msecs);
}

void qt_core_c_QDateTime_setOffsetFromUtc(QDateTime* this_ptr, int offsetSeconds) {
  this_ptr->setOffsetFromUtc(offsetSeconds);
}

void qt_core_c_QDateTime_setSecsSinceEpoch(QDateTime* this_ptr, qint64 secs) {
  this_ptr->setSecsSinceEpoch(secs);
}

void qt_core_c_QDateTime_setTime(QDateTime* this_ptr, const QTime* time) {
  this_ptr->setTime(*time);
}

void qt_core_c_QDateTime_setTimeSpec(QDateTime* this_ptr, const Qt::TimeSpec* spec) {
  this_ptr->setTimeSpec(*spec);
}

void qt_core_c_QDateTime_setTimeZone(QDateTime* this_ptr, const QTimeZone* toZone) {
  this_ptr->setTimeZone(*toZone);
}

void qt_core_c_QDateTime_setTime_t(QDateTime* this_ptr, unsigned int secsSince1Jan1970UTC) {
  this_ptr->setTime_t(secsSince1Jan1970UTC);
}

void qt_core_c_QDateTime_setUtcOffset(QDateTime* this_ptr, int seconds) {
  this_ptr->setUtcOffset(seconds);
}

void qt_core_c_QDateTime_swap(QDateTime* this_ptr, QDateTime* other) {
  this_ptr->swap(*other);
}

void qt_core_c_QDateTime_timeZoneAbbreviation_to_output(const QDateTime* this_ptr, QString* output) {
  new(output) QString(this_ptr->timeZoneAbbreviation());
}

void qt_core_c_QDateTime_timeZone_to_output(const QDateTime* this_ptr, QTimeZone* output) {
  new(output) QTimeZone(this_ptr->timeZone());
}

void qt_core_c_QDateTime_time_to_output(const QDateTime* this_ptr, QTime* output) {
  new(output) QTime(this_ptr->time());
}

void qt_core_c_QDateTime_toLocalTime_to_output(const QDateTime* this_ptr, QDateTime* output) {
  new(output) QDateTime(this_ptr->toLocalTime());
}

qint64 qt_core_c_QDateTime_toMSecsSinceEpoch(const QDateTime* this_ptr) {
  return this_ptr->toMSecsSinceEpoch();
}

void qt_core_c_QDateTime_toOffsetFromUtc_to_output(const QDateTime* this_ptr, int offsetSeconds, QDateTime* output) {
  new(output) QDateTime(this_ptr->toOffsetFromUtc(offsetSeconds));
}

qint64 qt_core_c_QDateTime_toSecsSinceEpoch(const QDateTime* this_ptr) {
  return this_ptr->toSecsSinceEpoch();
}

void qt_core_c_QDateTime_toString_to_output_f(const QDateTime* this_ptr, const Qt::DateFormat* f, QString* output) {
  new(output) QString(this_ptr->toString(*f));
}

void qt_core_c_QDateTime_toString_to_output_format(const QDateTime* this_ptr, const QString* format, QString* output) {
  new(output) QString(this_ptr->toString(*format));
}

void qt_core_c_QDateTime_toString_to_output_no_args(const QDateTime* this_ptr, QString* output) {
  new(output) QString(this_ptr->toString());
}

void qt_core_c_QDateTime_toTimeSpec_to_output(const QDateTime* this_ptr, const Qt::TimeSpec* spec, QDateTime* output) {
  new(output) QDateTime(this_ptr->toTimeSpec(*spec));
}

void qt_core_c_QDateTime_toTimeZone_to_output(const QDateTime* this_ptr, const QTimeZone* toZone, QDateTime* output) {
  new(output) QDateTime(this_ptr->toTimeZone(*toZone));
}

unsigned int qt_core_c_QDateTime_toTime_t(const QDateTime* this_ptr) {
  return this_ptr->toTime_t();
}

void qt_core_c_QDateTime_toUTC_to_output(const QDateTime* this_ptr, QDateTime* output) {
  new(output) QDateTime(this_ptr->toUTC());
}

int qt_core_c_QDateTime_utcOffset(const QDateTime* this_ptr) {
  return this_ptr->utcOffset();
}

