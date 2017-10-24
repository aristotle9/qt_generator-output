#ifndef QT_WIDGETS_C_QCALENDARWIDGET_H
#define QT_WIDGETS_C_QCALENDARWIDGET_H

#include "qt_widgets_c_global.h"

extern "C" {

QT_WIDGETS_C_EXPORT QCalendarWidget* qt_widgets_c_QCalendarWidget_G_dynamic_cast_QCalendarWidget_ptr(QWidget* ptr);
QT_WIDGETS_C_EXPORT QCalendarWidget* qt_widgets_c_QCalendarWidget_G_static_cast_QCalendarWidget_ptr_QObject(QObject* ptr);
QT_WIDGETS_C_EXPORT QCalendarWidget* qt_widgets_c_QCalendarWidget_G_static_cast_QCalendarWidget_ptr_QPaintDevice(QPaintDevice* ptr);
QT_WIDGETS_C_EXPORT QCalendarWidget* qt_widgets_c_QCalendarWidget_G_static_cast_QCalendarWidget_ptr_QWidget(QWidget* ptr);
QT_WIDGETS_C_EXPORT QObject* qt_widgets_c_QCalendarWidget_G_static_cast_QObject_ptr(QCalendarWidget* ptr);
QT_WIDGETS_C_EXPORT QPaintDevice* qt_widgets_c_QCalendarWidget_G_static_cast_QPaintDevice_ptr(QCalendarWidget* ptr);
QT_WIDGETS_C_EXPORT QWidget* qt_widgets_c_QCalendarWidget_G_static_cast_QWidget_ptr(QCalendarWidget* ptr);
QT_WIDGETS_C_EXPORT int qt_widgets_c_QCalendarWidget_dateEditAcceptDelay(const QCalendarWidget* this_ptr);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QCalendarWidget_dateTextFormat_to_output_date(const QCalendarWidget* this_ptr, const QDate* date, QTextCharFormat* output);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QCalendarWidget_dateTextFormat_to_output_no_args(const QCalendarWidget* this_ptr, QMap< QDate, QTextCharFormat >* output);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QCalendarWidget_delete(QCalendarWidget* this_ptr);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QCalendarWidget_headerTextFormat_to_output(const QCalendarWidget* this_ptr, QTextCharFormat* output);
QT_WIDGETS_C_EXPORT QCalendarWidget::HorizontalHeaderFormat qt_widgets_c_QCalendarWidget_horizontalHeaderFormat(const QCalendarWidget* this_ptr);
QT_WIDGETS_C_EXPORT bool qt_widgets_c_QCalendarWidget_isDateEditEnabled(const QCalendarWidget* this_ptr);
QT_WIDGETS_C_EXPORT bool qt_widgets_c_QCalendarWidget_isGridVisible(const QCalendarWidget* this_ptr);
QT_WIDGETS_C_EXPORT bool qt_widgets_c_QCalendarWidget_isNavigationBarVisible(const QCalendarWidget* this_ptr);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QCalendarWidget_maximumDate_to_output(const QCalendarWidget* this_ptr, QDate* output);
QT_WIDGETS_C_EXPORT const QMetaObject* qt_widgets_c_QCalendarWidget_metaObject(const QCalendarWidget* this_ptr);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QCalendarWidget_minimumDate_to_output(const QCalendarWidget* this_ptr, QDate* output);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QCalendarWidget_minimumSizeHint_to_output(const QCalendarWidget* this_ptr, QSize* output);
QT_WIDGETS_C_EXPORT int qt_widgets_c_QCalendarWidget_monthShown(const QCalendarWidget* this_ptr);
QT_WIDGETS_C_EXPORT QCalendarWidget* qt_widgets_c_QCalendarWidget_new_no_args();
QT_WIDGETS_C_EXPORT QCalendarWidget* qt_widgets_c_QCalendarWidget_new_parent(QWidget* parent);
QT_WIDGETS_C_EXPORT int qt_widgets_c_QCalendarWidget_qt_metacall(QCalendarWidget* this_ptr, const QMetaObject::Call* arg1, int arg2, void** arg3);
QT_WIDGETS_C_EXPORT void* qt_widgets_c_QCalendarWidget_qt_metacast(QCalendarWidget* this_ptr, const char* arg1);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QCalendarWidget_selectedDate_to_output(const QCalendarWidget* this_ptr, QDate* output);
QT_WIDGETS_C_EXPORT QCalendarWidget::SelectionMode qt_widgets_c_QCalendarWidget_selectionMode(const QCalendarWidget* this_ptr);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QCalendarWidget_setCurrentPage(QCalendarWidget* this_ptr, int year, int month);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QCalendarWidget_setDateEditAcceptDelay(QCalendarWidget* this_ptr, int delay);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QCalendarWidget_setDateEditEnabled(QCalendarWidget* this_ptr, bool enable);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QCalendarWidget_setDateRange(QCalendarWidget* this_ptr, const QDate* min, const QDate* max);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QCalendarWidget_setDateTextFormat(QCalendarWidget* this_ptr, const QDate* date, const QTextCharFormat* format);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QCalendarWidget_setFirstDayOfWeek(QCalendarWidget* this_ptr, const Qt::DayOfWeek* dayOfWeek);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QCalendarWidget_setGridVisible(QCalendarWidget* this_ptr, bool show);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QCalendarWidget_setHeaderTextFormat(QCalendarWidget* this_ptr, const QTextCharFormat* format);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QCalendarWidget_setHorizontalHeaderFormat(QCalendarWidget* this_ptr, QCalendarWidget::HorizontalHeaderFormat format);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QCalendarWidget_setMaximumDate(QCalendarWidget* this_ptr, const QDate* date);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QCalendarWidget_setMinimumDate(QCalendarWidget* this_ptr, const QDate* date);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QCalendarWidget_setNavigationBarVisible(QCalendarWidget* this_ptr, bool visible);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QCalendarWidget_setSelectedDate(QCalendarWidget* this_ptr, const QDate* date);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QCalendarWidget_setSelectionMode(QCalendarWidget* this_ptr, QCalendarWidget::SelectionMode mode);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QCalendarWidget_setVerticalHeaderFormat(QCalendarWidget* this_ptr, QCalendarWidget::VerticalHeaderFormat format);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QCalendarWidget_setWeekdayTextFormat(QCalendarWidget* this_ptr, const Qt::DayOfWeek* dayOfWeek, const QTextCharFormat* format);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QCalendarWidget_showNextMonth(QCalendarWidget* this_ptr);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QCalendarWidget_showNextYear(QCalendarWidget* this_ptr);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QCalendarWidget_showPreviousMonth(QCalendarWidget* this_ptr);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QCalendarWidget_showPreviousYear(QCalendarWidget* this_ptr);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QCalendarWidget_showSelectedDate(QCalendarWidget* this_ptr);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QCalendarWidget_showToday(QCalendarWidget* this_ptr);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QCalendarWidget_sizeHint_to_output(const QCalendarWidget* this_ptr, QSize* output);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QCalendarWidget_trUtf8_to_output(const char* s, const char* c, int n, QString* output);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QCalendarWidget_tr_to_output(const char* s, const char* c, int n, QString* output);
QT_WIDGETS_C_EXPORT QCalendarWidget::VerticalHeaderFormat qt_widgets_c_QCalendarWidget_verticalHeaderFormat(const QCalendarWidget* this_ptr);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QCalendarWidget_weekdayTextFormat_to_output(const QCalendarWidget* this_ptr, const Qt::DayOfWeek* dayOfWeek, QTextCharFormat* output);
QT_WIDGETS_C_EXPORT int qt_widgets_c_QCalendarWidget_yearShown(const QCalendarWidget* this_ptr);

} // extern "C"

#endif // QT_WIDGETS_C_QCALENDARWIDGET_H
