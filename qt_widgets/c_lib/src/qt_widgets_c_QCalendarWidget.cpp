#include "qt_widgets_c_QCalendarWidget.h"

QCalendarWidget* qt_widgets_c_QCalendarWidget_G_dynamic_cast_QCalendarWidget_ptr(QWidget* ptr) {
  return dynamic_cast<QCalendarWidget*>(ptr);
}

QCalendarWidget* qt_widgets_c_QCalendarWidget_G_static_cast_QCalendarWidget_ptr_QObject(QObject* ptr) {
  return static_cast<QCalendarWidget*>(ptr);
}

QCalendarWidget* qt_widgets_c_QCalendarWidget_G_static_cast_QCalendarWidget_ptr_QPaintDevice(QPaintDevice* ptr) {
  return static_cast<QCalendarWidget*>(ptr);
}

QCalendarWidget* qt_widgets_c_QCalendarWidget_G_static_cast_QCalendarWidget_ptr_QWidget(QWidget* ptr) {
  return static_cast<QCalendarWidget*>(ptr);
}

QObject* qt_widgets_c_QCalendarWidget_G_static_cast_QObject_ptr(QCalendarWidget* ptr) {
  return static_cast<QObject*>(ptr);
}

QPaintDevice* qt_widgets_c_QCalendarWidget_G_static_cast_QPaintDevice_ptr(QCalendarWidget* ptr) {
  return static_cast<QPaintDevice*>(ptr);
}

QWidget* qt_widgets_c_QCalendarWidget_G_static_cast_QWidget_ptr(QCalendarWidget* ptr) {
  return static_cast<QWidget*>(ptr);
}

int qt_widgets_c_QCalendarWidget_dateEditAcceptDelay(const QCalendarWidget* this_ptr) {
  return this_ptr->dateEditAcceptDelay();
}

void qt_widgets_c_QCalendarWidget_dateTextFormat_to_output_date(const QCalendarWidget* this_ptr, const QDate* date, QTextCharFormat* output) {
  new(output) QTextCharFormat(this_ptr->dateTextFormat(*date));
}

void qt_widgets_c_QCalendarWidget_dateTextFormat_to_output_no_args(const QCalendarWidget* this_ptr, QMap< QDate, QTextCharFormat >* output) {
  new(output) QMap< QDate, QTextCharFormat >(this_ptr->dateTextFormat());
}

void qt_widgets_c_QCalendarWidget_delete(QCalendarWidget* this_ptr) {
  delete this_ptr;
}

void qt_widgets_c_QCalendarWidget_headerTextFormat_to_output(const QCalendarWidget* this_ptr, QTextCharFormat* output) {
  new(output) QTextCharFormat(this_ptr->headerTextFormat());
}

QCalendarWidget::HorizontalHeaderFormat qt_widgets_c_QCalendarWidget_horizontalHeaderFormat(const QCalendarWidget* this_ptr) {
  return this_ptr->horizontalHeaderFormat();
}

bool qt_widgets_c_QCalendarWidget_isDateEditEnabled(const QCalendarWidget* this_ptr) {
  return this_ptr->isDateEditEnabled();
}

bool qt_widgets_c_QCalendarWidget_isGridVisible(const QCalendarWidget* this_ptr) {
  return this_ptr->isGridVisible();
}

bool qt_widgets_c_QCalendarWidget_isNavigationBarVisible(const QCalendarWidget* this_ptr) {
  return this_ptr->isNavigationBarVisible();
}

void qt_widgets_c_QCalendarWidget_maximumDate_to_output(const QCalendarWidget* this_ptr, QDate* output) {
  new(output) QDate(this_ptr->maximumDate());
}

const QMetaObject* qt_widgets_c_QCalendarWidget_metaObject(const QCalendarWidget* this_ptr) {
  return this_ptr->metaObject();
}

void qt_widgets_c_QCalendarWidget_minimumDate_to_output(const QCalendarWidget* this_ptr, QDate* output) {
  new(output) QDate(this_ptr->minimumDate());
}

void qt_widgets_c_QCalendarWidget_minimumSizeHint_to_output(const QCalendarWidget* this_ptr, QSize* output) {
  new(output) QSize(this_ptr->minimumSizeHint());
}

int qt_widgets_c_QCalendarWidget_monthShown(const QCalendarWidget* this_ptr) {
  return this_ptr->monthShown();
}

QCalendarWidget* qt_widgets_c_QCalendarWidget_new_no_args() {
  return new QCalendarWidget();
}

QCalendarWidget* qt_widgets_c_QCalendarWidget_new_parent(QWidget* parent) {
  return new QCalendarWidget(parent);
}

int qt_widgets_c_QCalendarWidget_qt_metacall(QCalendarWidget* this_ptr, const QMetaObject::Call* arg1, int arg2, void** arg3) {
  return this_ptr->qt_metacall(*arg1, arg2, arg3);
}

void* qt_widgets_c_QCalendarWidget_qt_metacast(QCalendarWidget* this_ptr, const char* arg1) {
  return this_ptr->qt_metacast(arg1);
}

void qt_widgets_c_QCalendarWidget_selectedDate_to_output(const QCalendarWidget* this_ptr, QDate* output) {
  new(output) QDate(this_ptr->selectedDate());
}

QCalendarWidget::SelectionMode qt_widgets_c_QCalendarWidget_selectionMode(const QCalendarWidget* this_ptr) {
  return this_ptr->selectionMode();
}

void qt_widgets_c_QCalendarWidget_setCurrentPage(QCalendarWidget* this_ptr, int year, int month) {
  this_ptr->setCurrentPage(year, month);
}

void qt_widgets_c_QCalendarWidget_setDateEditAcceptDelay(QCalendarWidget* this_ptr, int delay) {
  this_ptr->setDateEditAcceptDelay(delay);
}

void qt_widgets_c_QCalendarWidget_setDateEditEnabled(QCalendarWidget* this_ptr, bool enable) {
  this_ptr->setDateEditEnabled(enable);
}

void qt_widgets_c_QCalendarWidget_setDateRange(QCalendarWidget* this_ptr, const QDate* min, const QDate* max) {
  this_ptr->setDateRange(*min, *max);
}

void qt_widgets_c_QCalendarWidget_setDateTextFormat(QCalendarWidget* this_ptr, const QDate* date, const QTextCharFormat* format) {
  this_ptr->setDateTextFormat(*date, *format);
}

void qt_widgets_c_QCalendarWidget_setFirstDayOfWeek(QCalendarWidget* this_ptr, const Qt::DayOfWeek* dayOfWeek) {
  this_ptr->setFirstDayOfWeek(*dayOfWeek);
}

void qt_widgets_c_QCalendarWidget_setGridVisible(QCalendarWidget* this_ptr, bool show) {
  this_ptr->setGridVisible(show);
}

void qt_widgets_c_QCalendarWidget_setHeaderTextFormat(QCalendarWidget* this_ptr, const QTextCharFormat* format) {
  this_ptr->setHeaderTextFormat(*format);
}

void qt_widgets_c_QCalendarWidget_setHorizontalHeaderFormat(QCalendarWidget* this_ptr, QCalendarWidget::HorizontalHeaderFormat format) {
  this_ptr->setHorizontalHeaderFormat(format);
}

void qt_widgets_c_QCalendarWidget_setMaximumDate(QCalendarWidget* this_ptr, const QDate* date) {
  this_ptr->setMaximumDate(*date);
}

void qt_widgets_c_QCalendarWidget_setMinimumDate(QCalendarWidget* this_ptr, const QDate* date) {
  this_ptr->setMinimumDate(*date);
}

void qt_widgets_c_QCalendarWidget_setNavigationBarVisible(QCalendarWidget* this_ptr, bool visible) {
  this_ptr->setNavigationBarVisible(visible);
}

void qt_widgets_c_QCalendarWidget_setSelectedDate(QCalendarWidget* this_ptr, const QDate* date) {
  this_ptr->setSelectedDate(*date);
}

void qt_widgets_c_QCalendarWidget_setSelectionMode(QCalendarWidget* this_ptr, QCalendarWidget::SelectionMode mode) {
  this_ptr->setSelectionMode(mode);
}

void qt_widgets_c_QCalendarWidget_setVerticalHeaderFormat(QCalendarWidget* this_ptr, QCalendarWidget::VerticalHeaderFormat format) {
  this_ptr->setVerticalHeaderFormat(format);
}

void qt_widgets_c_QCalendarWidget_setWeekdayTextFormat(QCalendarWidget* this_ptr, const Qt::DayOfWeek* dayOfWeek, const QTextCharFormat* format) {
  this_ptr->setWeekdayTextFormat(*dayOfWeek, *format);
}

void qt_widgets_c_QCalendarWidget_showNextMonth(QCalendarWidget* this_ptr) {
  this_ptr->showNextMonth();
}

void qt_widgets_c_QCalendarWidget_showNextYear(QCalendarWidget* this_ptr) {
  this_ptr->showNextYear();
}

void qt_widgets_c_QCalendarWidget_showPreviousMonth(QCalendarWidget* this_ptr) {
  this_ptr->showPreviousMonth();
}

void qt_widgets_c_QCalendarWidget_showPreviousYear(QCalendarWidget* this_ptr) {
  this_ptr->showPreviousYear();
}

void qt_widgets_c_QCalendarWidget_showSelectedDate(QCalendarWidget* this_ptr) {
  this_ptr->showSelectedDate();
}

void qt_widgets_c_QCalendarWidget_showToday(QCalendarWidget* this_ptr) {
  this_ptr->showToday();
}

void qt_widgets_c_QCalendarWidget_sizeHint_to_output(const QCalendarWidget* this_ptr, QSize* output) {
  new(output) QSize(this_ptr->sizeHint());
}

void qt_widgets_c_QCalendarWidget_trUtf8_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(QCalendarWidget::trUtf8(s, c, n));
}

void qt_widgets_c_QCalendarWidget_tr_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(QCalendarWidget::tr(s, c, n));
}

QCalendarWidget::VerticalHeaderFormat qt_widgets_c_QCalendarWidget_verticalHeaderFormat(const QCalendarWidget* this_ptr) {
  return this_ptr->verticalHeaderFormat();
}

void qt_widgets_c_QCalendarWidget_weekdayTextFormat_to_output(const QCalendarWidget* this_ptr, const Qt::DayOfWeek* dayOfWeek, QTextCharFormat* output) {
  new(output) QTextCharFormat(this_ptr->weekdayTextFormat(*dayOfWeek));
}

int qt_widgets_c_QCalendarWidget_yearShown(const QCalendarWidget* this_ptr) {
  return this_ptr->yearShown();
}

