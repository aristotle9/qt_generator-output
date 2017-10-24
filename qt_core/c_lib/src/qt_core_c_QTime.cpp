#include "qt_core_c_QTime.h"

void qt_core_c_QTime_addMSecs_to_output(const QTime* this_ptr, int ms, QTime* output) {
  new(output) QTime(this_ptr->addMSecs(ms));
}

void qt_core_c_QTime_addSecs_to_output(const QTime* this_ptr, int secs, QTime* output) {
  new(output) QTime(this_ptr->addSecs(secs));
}

void qt_core_c_QTime_constructor_h_m(int h, int m, QTime* output) {
  new(output) QTime(h, m);
}

void qt_core_c_QTime_constructor_h_m_s(int h, int m, int s, QTime* output) {
  new(output) QTime(h, m, s);
}

void qt_core_c_QTime_constructor_h_m_s_ms(int h, int m, int s, int ms, QTime* output) {
  new(output) QTime(h, m, s, ms);
}

void qt_core_c_QTime_constructor_no_args(QTime* output) {
  new(output) QTime();
}

void qt_core_c_QTime_currentTime_to_output(QTime* output) {
  new(output) QTime(QTime::currentTime());
}

void qt_core_c_QTime_destructor(QTime* this_ptr) {
  qt_core_c_call_destructor(this_ptr);
}

int qt_core_c_QTime_elapsed(const QTime* this_ptr) {
  return this_ptr->elapsed();
}

void qt_core_c_QTime_fromMSecsSinceStartOfDay_to_output(int msecs, QTime* output) {
  new(output) QTime(QTime::fromMSecsSinceStartOfDay(msecs));
}

void qt_core_c_QTime_fromString_to_output_s(const QString* s, QTime* output) {
  new(output) QTime(QTime::fromString(*s));
}

void qt_core_c_QTime_fromString_to_output_s_f(const QString* s, const Qt::DateFormat* f, QTime* output) {
  new(output) QTime(QTime::fromString(*s, *f));
}

void qt_core_c_QTime_fromString_to_output_s_format(const QString* s, const QString* format, QTime* output) {
  new(output) QTime(QTime::fromString(*s, *format));
}

int qt_core_c_QTime_hour(const QTime* this_ptr) {
  return this_ptr->hour();
}

bool qt_core_c_QTime_isNull(const QTime* this_ptr) {
  return this_ptr->isNull();
}

bool qt_core_c_QTime_isValid_h_m_s(int h, int m, int s) {
  return QTime::isValid(h, m, s);
}

bool qt_core_c_QTime_isValid_h_m_s_ms(int h, int m, int s, int ms) {
  return QTime::isValid(h, m, s, ms);
}

bool qt_core_c_QTime_isValid_no_args(const QTime* this_ptr) {
  return this_ptr->isValid();
}

int qt_core_c_QTime_minute(const QTime* this_ptr) {
  return this_ptr->minute();
}

int qt_core_c_QTime_msec(const QTime* this_ptr) {
  return this_ptr->msec();
}

int qt_core_c_QTime_msecsSinceStartOfDay(const QTime* this_ptr) {
  return this_ptr->msecsSinceStartOfDay();
}

int qt_core_c_QTime_msecsTo(const QTime* this_ptr, const QTime* arg1) {
  return this_ptr->msecsTo(*arg1);
}

bool qt_core_c_QTime_operator_eq(const QTime* this_ptr, const QTime* other) {
  return this_ptr->operator==(*other);
}

bool qt_core_c_QTime_operator_ge(const QTime* this_ptr, const QTime* other) {
  return this_ptr->operator>=(*other);
}

bool qt_core_c_QTime_operator_gt(const QTime* this_ptr, const QTime* other) {
  return this_ptr->operator>(*other);
}

bool qt_core_c_QTime_operator_le(const QTime* this_ptr, const QTime* other) {
  return this_ptr->operator<=(*other);
}

bool qt_core_c_QTime_operator_lt(const QTime* this_ptr, const QTime* other) {
  return this_ptr->operator<(*other);
}

bool qt_core_c_QTime_operator_neq(const QTime* this_ptr, const QTime* other) {
  return this_ptr->operator!=(*other);
}

int qt_core_c_QTime_restart(QTime* this_ptr) {
  return this_ptr->restart();
}

int qt_core_c_QTime_second(const QTime* this_ptr) {
  return this_ptr->second();
}

int qt_core_c_QTime_secsTo(const QTime* this_ptr, const QTime* arg1) {
  return this_ptr->secsTo(*arg1);
}

bool qt_core_c_QTime_setHMS_h_m_s(QTime* this_ptr, int h, int m, int s) {
  return this_ptr->setHMS(h, m, s);
}

bool qt_core_c_QTime_setHMS_h_m_s_ms(QTime* this_ptr, int h, int m, int s, int ms) {
  return this_ptr->setHMS(h, m, s, ms);
}

void qt_core_c_QTime_start(QTime* this_ptr) {
  this_ptr->start();
}

void qt_core_c_QTime_toString_to_output_f(const QTime* this_ptr, const Qt::DateFormat* f, QString* output) {
  new(output) QString(this_ptr->toString(*f));
}

void qt_core_c_QTime_toString_to_output_format(const QTime* this_ptr, const QString* format, QString* output) {
  new(output) QString(this_ptr->toString(*format));
}

void qt_core_c_QTime_toString_to_output_no_args(const QTime* this_ptr, QString* output) {
  new(output) QString(this_ptr->toString());
}

