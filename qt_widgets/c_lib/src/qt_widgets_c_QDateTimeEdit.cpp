#include "qt_widgets_c_QDateTimeEdit.h"

QDateTimeEdit* qt_widgets_c_QDateTimeEdit_G_dynamic_cast_QDateTimeEdit_ptr_QAbstractSpinBox(QAbstractSpinBox* ptr) {
  return dynamic_cast<QDateTimeEdit*>(ptr);
}

QDateTimeEdit* qt_widgets_c_QDateTimeEdit_G_dynamic_cast_QDateTimeEdit_ptr_QWidget(QWidget* ptr) {
  return dynamic_cast<QDateTimeEdit*>(ptr);
}

QAbstractSpinBox* qt_widgets_c_QDateTimeEdit_G_static_cast_QAbstractSpinBox_ptr(QDateTimeEdit* ptr) {
  return static_cast<QAbstractSpinBox*>(ptr);
}

QDateTimeEdit* qt_widgets_c_QDateTimeEdit_G_static_cast_QDateTimeEdit_ptr_QAbstractSpinBox(QAbstractSpinBox* ptr) {
  return static_cast<QDateTimeEdit*>(ptr);
}

QDateTimeEdit* qt_widgets_c_QDateTimeEdit_G_static_cast_QDateTimeEdit_ptr_QObject(QObject* ptr) {
  return static_cast<QDateTimeEdit*>(ptr);
}

QDateTimeEdit* qt_widgets_c_QDateTimeEdit_G_static_cast_QDateTimeEdit_ptr_QPaintDevice(QPaintDevice* ptr) {
  return static_cast<QDateTimeEdit*>(ptr);
}

QDateTimeEdit* qt_widgets_c_QDateTimeEdit_G_static_cast_QDateTimeEdit_ptr_QWidget(QWidget* ptr) {
  return static_cast<QDateTimeEdit*>(ptr);
}

QObject* qt_widgets_c_QDateTimeEdit_G_static_cast_QObject_ptr(QDateTimeEdit* ptr) {
  return static_cast<QObject*>(ptr);
}

QPaintDevice* qt_widgets_c_QDateTimeEdit_G_static_cast_QPaintDevice_ptr(QDateTimeEdit* ptr) {
  return static_cast<QPaintDevice*>(ptr);
}

QWidget* qt_widgets_c_QDateTimeEdit_G_static_cast_QWidget_ptr(QDateTimeEdit* ptr) {
  return static_cast<QWidget*>(ptr);
}

bool qt_widgets_c_QDateTimeEdit_calendarPopup(const QDateTimeEdit* this_ptr) {
  return this_ptr->calendarPopup();
}

QCalendarWidget* qt_widgets_c_QDateTimeEdit_calendarWidget(const QDateTimeEdit* this_ptr) {
  return this_ptr->calendarWidget();
}

void qt_widgets_c_QDateTimeEdit_clear(QDateTimeEdit* this_ptr) {
  this_ptr->clear();
}

void qt_widgets_c_QDateTimeEdit_clearMaximumDate(QDateTimeEdit* this_ptr) {
  this_ptr->clearMaximumDate();
}

void qt_widgets_c_QDateTimeEdit_clearMaximumDateTime(QDateTimeEdit* this_ptr) {
  this_ptr->clearMaximumDateTime();
}

void qt_widgets_c_QDateTimeEdit_clearMaximumTime(QDateTimeEdit* this_ptr) {
  this_ptr->clearMaximumTime();
}

void qt_widgets_c_QDateTimeEdit_clearMinimumDate(QDateTimeEdit* this_ptr) {
  this_ptr->clearMinimumDate();
}

void qt_widgets_c_QDateTimeEdit_clearMinimumDateTime(QDateTimeEdit* this_ptr) {
  this_ptr->clearMinimumDateTime();
}

void qt_widgets_c_QDateTimeEdit_clearMinimumTime(QDateTimeEdit* this_ptr) {
  this_ptr->clearMinimumTime();
}

QDateTimeEdit::Section qt_widgets_c_QDateTimeEdit_currentSection(const QDateTimeEdit* this_ptr) {
  return this_ptr->currentSection();
}

int qt_widgets_c_QDateTimeEdit_currentSectionIndex(const QDateTimeEdit* this_ptr) {
  return this_ptr->currentSectionIndex();
}

void qt_widgets_c_QDateTimeEdit_dateTime_to_output(const QDateTimeEdit* this_ptr, QDateTime* output) {
  new(output) QDateTime(this_ptr->dateTime());
}

void qt_widgets_c_QDateTimeEdit_date_to_output(const QDateTimeEdit* this_ptr, QDate* output) {
  new(output) QDate(this_ptr->date());
}

void qt_widgets_c_QDateTimeEdit_delete(QDateTimeEdit* this_ptr) {
  delete this_ptr;
}

void qt_widgets_c_QDateTimeEdit_displayFormat_to_output(const QDateTimeEdit* this_ptr, QString* output) {
  new(output) QString(this_ptr->displayFormat());
}

unsigned int qt_widgets_c_QDateTimeEdit_displayedSections(const QDateTimeEdit* this_ptr) {
  return uint(this_ptr->displayedSections());
}

bool qt_widgets_c_QDateTimeEdit_event(QDateTimeEdit* this_ptr, QEvent* event) {
  return this_ptr->event(event);
}

void qt_widgets_c_QDateTimeEdit_maximumDateTime_to_output(const QDateTimeEdit* this_ptr, QDateTime* output) {
  new(output) QDateTime(this_ptr->maximumDateTime());
}

void qt_widgets_c_QDateTimeEdit_maximumDate_to_output(const QDateTimeEdit* this_ptr, QDate* output) {
  new(output) QDate(this_ptr->maximumDate());
}

void qt_widgets_c_QDateTimeEdit_maximumTime_to_output(const QDateTimeEdit* this_ptr, QTime* output) {
  new(output) QTime(this_ptr->maximumTime());
}

const QMetaObject* qt_widgets_c_QDateTimeEdit_metaObject(const QDateTimeEdit* this_ptr) {
  return this_ptr->metaObject();
}

void qt_widgets_c_QDateTimeEdit_minimumDateTime_to_output(const QDateTimeEdit* this_ptr, QDateTime* output) {
  new(output) QDateTime(this_ptr->minimumDateTime());
}

void qt_widgets_c_QDateTimeEdit_minimumDate_to_output(const QDateTimeEdit* this_ptr, QDate* output) {
  new(output) QDate(this_ptr->minimumDate());
}

void qt_widgets_c_QDateTimeEdit_minimumTime_to_output(const QDateTimeEdit* this_ptr, QTime* output) {
  new(output) QTime(this_ptr->minimumTime());
}

QDateTimeEdit* qt_widgets_c_QDateTimeEdit_new_d(const QDate* d) {
  return new QDateTimeEdit(*d);
}

QDateTimeEdit* qt_widgets_c_QDateTimeEdit_new_d_parent(const QDate* d, QWidget* parent) {
  return new QDateTimeEdit(*d, parent);
}

QDateTimeEdit* qt_widgets_c_QDateTimeEdit_new_dt(const QDateTime* dt) {
  return new QDateTimeEdit(*dt);
}

QDateTimeEdit* qt_widgets_c_QDateTimeEdit_new_dt_parent(const QDateTime* dt, QWidget* parent) {
  return new QDateTimeEdit(*dt, parent);
}

QDateTimeEdit* qt_widgets_c_QDateTimeEdit_new_no_args() {
  return new QDateTimeEdit();
}

QDateTimeEdit* qt_widgets_c_QDateTimeEdit_new_parent(QWidget* parent) {
  return new QDateTimeEdit(parent);
}

QDateTimeEdit* qt_widgets_c_QDateTimeEdit_new_t(const QTime* t) {
  return new QDateTimeEdit(*t);
}

QDateTimeEdit* qt_widgets_c_QDateTimeEdit_new_t_parent(const QTime* t, QWidget* parent) {
  return new QDateTimeEdit(*t, parent);
}

int qt_widgets_c_QDateTimeEdit_qt_metacall(QDateTimeEdit* this_ptr, const QMetaObject::Call* arg1, int arg2, void** arg3) {
  return this_ptr->qt_metacall(*arg1, arg2, arg3);
}

void* qt_widgets_c_QDateTimeEdit_qt_metacast(QDateTimeEdit* this_ptr, const char* arg1) {
  return this_ptr->qt_metacast(arg1);
}

QDateTimeEdit::Section qt_widgets_c_QDateTimeEdit_sectionAt(const QDateTimeEdit* this_ptr, int index) {
  return this_ptr->sectionAt(index);
}

int qt_widgets_c_QDateTimeEdit_sectionCount(const QDateTimeEdit* this_ptr) {
  return this_ptr->sectionCount();
}

void qt_widgets_c_QDateTimeEdit_sectionText_to_output(const QDateTimeEdit* this_ptr, QDateTimeEdit::Section section, QString* output) {
  new(output) QString(this_ptr->sectionText(section));
}

void qt_widgets_c_QDateTimeEdit_setCalendarPopup(QDateTimeEdit* this_ptr, bool enable) {
  this_ptr->setCalendarPopup(enable);
}

void qt_widgets_c_QDateTimeEdit_setCalendarWidget(QDateTimeEdit* this_ptr, QCalendarWidget* calendarWidget) {
  this_ptr->setCalendarWidget(calendarWidget);
}

void qt_widgets_c_QDateTimeEdit_setCurrentSection(QDateTimeEdit* this_ptr, QDateTimeEdit::Section section) {
  this_ptr->setCurrentSection(section);
}

void qt_widgets_c_QDateTimeEdit_setCurrentSectionIndex(QDateTimeEdit* this_ptr, int index) {
  this_ptr->setCurrentSectionIndex(index);
}

void qt_widgets_c_QDateTimeEdit_setDate(QDateTimeEdit* this_ptr, const QDate* date) {
  this_ptr->setDate(*date);
}

void qt_widgets_c_QDateTimeEdit_setDateRange(QDateTimeEdit* this_ptr, const QDate* min, const QDate* max) {
  this_ptr->setDateRange(*min, *max);
}

void qt_widgets_c_QDateTimeEdit_setDateTime(QDateTimeEdit* this_ptr, const QDateTime* dateTime) {
  this_ptr->setDateTime(*dateTime);
}

void qt_widgets_c_QDateTimeEdit_setDateTimeRange(QDateTimeEdit* this_ptr, const QDateTime* min, const QDateTime* max) {
  this_ptr->setDateTimeRange(*min, *max);
}

void qt_widgets_c_QDateTimeEdit_setDisplayFormat(QDateTimeEdit* this_ptr, const QString* format) {
  this_ptr->setDisplayFormat(*format);
}

void qt_widgets_c_QDateTimeEdit_setMaximumDate(QDateTimeEdit* this_ptr, const QDate* max) {
  this_ptr->setMaximumDate(*max);
}

void qt_widgets_c_QDateTimeEdit_setMaximumDateTime(QDateTimeEdit* this_ptr, const QDateTime* dt) {
  this_ptr->setMaximumDateTime(*dt);
}

void qt_widgets_c_QDateTimeEdit_setMaximumTime(QDateTimeEdit* this_ptr, const QTime* max) {
  this_ptr->setMaximumTime(*max);
}

void qt_widgets_c_QDateTimeEdit_setMinimumDate(QDateTimeEdit* this_ptr, const QDate* min) {
  this_ptr->setMinimumDate(*min);
}

void qt_widgets_c_QDateTimeEdit_setMinimumDateTime(QDateTimeEdit* this_ptr, const QDateTime* dt) {
  this_ptr->setMinimumDateTime(*dt);
}

void qt_widgets_c_QDateTimeEdit_setMinimumTime(QDateTimeEdit* this_ptr, const QTime* min) {
  this_ptr->setMinimumTime(*min);
}

void qt_widgets_c_QDateTimeEdit_setSelectedSection(QDateTimeEdit* this_ptr, QDateTimeEdit::Section section) {
  this_ptr->setSelectedSection(section);
}

void qt_widgets_c_QDateTimeEdit_setTime(QDateTimeEdit* this_ptr, const QTime* time) {
  this_ptr->setTime(*time);
}

void qt_widgets_c_QDateTimeEdit_setTimeRange(QDateTimeEdit* this_ptr, const QTime* min, const QTime* max) {
  this_ptr->setTimeRange(*min, *max);
}

void qt_widgets_c_QDateTimeEdit_setTimeSpec(QDateTimeEdit* this_ptr, const Qt::TimeSpec* spec) {
  this_ptr->setTimeSpec(*spec);
}

void qt_widgets_c_QDateTimeEdit_sizeHint_to_output(const QDateTimeEdit* this_ptr, QSize* output) {
  new(output) QSize(this_ptr->sizeHint());
}

void qt_widgets_c_QDateTimeEdit_stepBy(QDateTimeEdit* this_ptr, int steps) {
  this_ptr->stepBy(steps);
}

void qt_widgets_c_QDateTimeEdit_time_to_output(const QDateTimeEdit* this_ptr, QTime* output) {
  new(output) QTime(this_ptr->time());
}

void qt_widgets_c_QDateTimeEdit_trUtf8_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(QDateTimeEdit::trUtf8(s, c, n));
}

void qt_widgets_c_QDateTimeEdit_tr_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(QDateTimeEdit::tr(s, c, n));
}

