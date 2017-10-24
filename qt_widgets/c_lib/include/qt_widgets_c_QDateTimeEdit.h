#ifndef QT_WIDGETS_C_QDATETIMEEDIT_H
#define QT_WIDGETS_C_QDATETIMEEDIT_H

#include "qt_widgets_c_global.h"

extern "C" {

QT_WIDGETS_C_EXPORT QDateTimeEdit* qt_widgets_c_QDateTimeEdit_G_dynamic_cast_QDateTimeEdit_ptr_QAbstractSpinBox(QAbstractSpinBox* ptr);
QT_WIDGETS_C_EXPORT QDateTimeEdit* qt_widgets_c_QDateTimeEdit_G_dynamic_cast_QDateTimeEdit_ptr_QWidget(QWidget* ptr);
QT_WIDGETS_C_EXPORT QAbstractSpinBox* qt_widgets_c_QDateTimeEdit_G_static_cast_QAbstractSpinBox_ptr(QDateTimeEdit* ptr);
QT_WIDGETS_C_EXPORT QDateTimeEdit* qt_widgets_c_QDateTimeEdit_G_static_cast_QDateTimeEdit_ptr_QAbstractSpinBox(QAbstractSpinBox* ptr);
QT_WIDGETS_C_EXPORT QDateTimeEdit* qt_widgets_c_QDateTimeEdit_G_static_cast_QDateTimeEdit_ptr_QObject(QObject* ptr);
QT_WIDGETS_C_EXPORT QDateTimeEdit* qt_widgets_c_QDateTimeEdit_G_static_cast_QDateTimeEdit_ptr_QPaintDevice(QPaintDevice* ptr);
QT_WIDGETS_C_EXPORT QDateTimeEdit* qt_widgets_c_QDateTimeEdit_G_static_cast_QDateTimeEdit_ptr_QWidget(QWidget* ptr);
QT_WIDGETS_C_EXPORT QObject* qt_widgets_c_QDateTimeEdit_G_static_cast_QObject_ptr(QDateTimeEdit* ptr);
QT_WIDGETS_C_EXPORT QPaintDevice* qt_widgets_c_QDateTimeEdit_G_static_cast_QPaintDevice_ptr(QDateTimeEdit* ptr);
QT_WIDGETS_C_EXPORT QWidget* qt_widgets_c_QDateTimeEdit_G_static_cast_QWidget_ptr(QDateTimeEdit* ptr);
QT_WIDGETS_C_EXPORT bool qt_widgets_c_QDateTimeEdit_calendarPopup(const QDateTimeEdit* this_ptr);
QT_WIDGETS_C_EXPORT QCalendarWidget* qt_widgets_c_QDateTimeEdit_calendarWidget(const QDateTimeEdit* this_ptr);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QDateTimeEdit_clear(QDateTimeEdit* this_ptr);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QDateTimeEdit_clearMaximumDate(QDateTimeEdit* this_ptr);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QDateTimeEdit_clearMaximumDateTime(QDateTimeEdit* this_ptr);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QDateTimeEdit_clearMaximumTime(QDateTimeEdit* this_ptr);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QDateTimeEdit_clearMinimumDate(QDateTimeEdit* this_ptr);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QDateTimeEdit_clearMinimumDateTime(QDateTimeEdit* this_ptr);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QDateTimeEdit_clearMinimumTime(QDateTimeEdit* this_ptr);
QT_WIDGETS_C_EXPORT QDateTimeEdit::Section qt_widgets_c_QDateTimeEdit_currentSection(const QDateTimeEdit* this_ptr);
QT_WIDGETS_C_EXPORT int qt_widgets_c_QDateTimeEdit_currentSectionIndex(const QDateTimeEdit* this_ptr);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QDateTimeEdit_dateTime_to_output(const QDateTimeEdit* this_ptr, QDateTime* output);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QDateTimeEdit_date_to_output(const QDateTimeEdit* this_ptr, QDate* output);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QDateTimeEdit_delete(QDateTimeEdit* this_ptr);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QDateTimeEdit_displayFormat_to_output(const QDateTimeEdit* this_ptr, QString* output);
QT_WIDGETS_C_EXPORT unsigned int qt_widgets_c_QDateTimeEdit_displayedSections(const QDateTimeEdit* this_ptr);
QT_WIDGETS_C_EXPORT bool qt_widgets_c_QDateTimeEdit_event(QDateTimeEdit* this_ptr, QEvent* event);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QDateTimeEdit_maximumDateTime_to_output(const QDateTimeEdit* this_ptr, QDateTime* output);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QDateTimeEdit_maximumDate_to_output(const QDateTimeEdit* this_ptr, QDate* output);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QDateTimeEdit_maximumTime_to_output(const QDateTimeEdit* this_ptr, QTime* output);
QT_WIDGETS_C_EXPORT const QMetaObject* qt_widgets_c_QDateTimeEdit_metaObject(const QDateTimeEdit* this_ptr);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QDateTimeEdit_minimumDateTime_to_output(const QDateTimeEdit* this_ptr, QDateTime* output);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QDateTimeEdit_minimumDate_to_output(const QDateTimeEdit* this_ptr, QDate* output);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QDateTimeEdit_minimumTime_to_output(const QDateTimeEdit* this_ptr, QTime* output);
QT_WIDGETS_C_EXPORT QDateTimeEdit* qt_widgets_c_QDateTimeEdit_new_d(const QDate* d);
QT_WIDGETS_C_EXPORT QDateTimeEdit* qt_widgets_c_QDateTimeEdit_new_d_parent(const QDate* d, QWidget* parent);
QT_WIDGETS_C_EXPORT QDateTimeEdit* qt_widgets_c_QDateTimeEdit_new_dt(const QDateTime* dt);
QT_WIDGETS_C_EXPORT QDateTimeEdit* qt_widgets_c_QDateTimeEdit_new_dt_parent(const QDateTime* dt, QWidget* parent);
QT_WIDGETS_C_EXPORT QDateTimeEdit* qt_widgets_c_QDateTimeEdit_new_no_args();
QT_WIDGETS_C_EXPORT QDateTimeEdit* qt_widgets_c_QDateTimeEdit_new_parent(QWidget* parent);
QT_WIDGETS_C_EXPORT QDateTimeEdit* qt_widgets_c_QDateTimeEdit_new_t(const QTime* t);
QT_WIDGETS_C_EXPORT QDateTimeEdit* qt_widgets_c_QDateTimeEdit_new_t_parent(const QTime* t, QWidget* parent);
QT_WIDGETS_C_EXPORT int qt_widgets_c_QDateTimeEdit_qt_metacall(QDateTimeEdit* this_ptr, const QMetaObject::Call* arg1, int arg2, void** arg3);
QT_WIDGETS_C_EXPORT void* qt_widgets_c_QDateTimeEdit_qt_metacast(QDateTimeEdit* this_ptr, const char* arg1);
QT_WIDGETS_C_EXPORT QDateTimeEdit::Section qt_widgets_c_QDateTimeEdit_sectionAt(const QDateTimeEdit* this_ptr, int index);
QT_WIDGETS_C_EXPORT int qt_widgets_c_QDateTimeEdit_sectionCount(const QDateTimeEdit* this_ptr);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QDateTimeEdit_sectionText_to_output(const QDateTimeEdit* this_ptr, QDateTimeEdit::Section section, QString* output);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QDateTimeEdit_setCalendarPopup(QDateTimeEdit* this_ptr, bool enable);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QDateTimeEdit_setCalendarWidget(QDateTimeEdit* this_ptr, QCalendarWidget* calendarWidget);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QDateTimeEdit_setCurrentSection(QDateTimeEdit* this_ptr, QDateTimeEdit::Section section);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QDateTimeEdit_setCurrentSectionIndex(QDateTimeEdit* this_ptr, int index);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QDateTimeEdit_setDate(QDateTimeEdit* this_ptr, const QDate* date);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QDateTimeEdit_setDateRange(QDateTimeEdit* this_ptr, const QDate* min, const QDate* max);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QDateTimeEdit_setDateTime(QDateTimeEdit* this_ptr, const QDateTime* dateTime);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QDateTimeEdit_setDateTimeRange(QDateTimeEdit* this_ptr, const QDateTime* min, const QDateTime* max);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QDateTimeEdit_setDisplayFormat(QDateTimeEdit* this_ptr, const QString* format);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QDateTimeEdit_setMaximumDate(QDateTimeEdit* this_ptr, const QDate* max);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QDateTimeEdit_setMaximumDateTime(QDateTimeEdit* this_ptr, const QDateTime* dt);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QDateTimeEdit_setMaximumTime(QDateTimeEdit* this_ptr, const QTime* max);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QDateTimeEdit_setMinimumDate(QDateTimeEdit* this_ptr, const QDate* min);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QDateTimeEdit_setMinimumDateTime(QDateTimeEdit* this_ptr, const QDateTime* dt);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QDateTimeEdit_setMinimumTime(QDateTimeEdit* this_ptr, const QTime* min);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QDateTimeEdit_setSelectedSection(QDateTimeEdit* this_ptr, QDateTimeEdit::Section section);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QDateTimeEdit_setTime(QDateTimeEdit* this_ptr, const QTime* time);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QDateTimeEdit_setTimeRange(QDateTimeEdit* this_ptr, const QTime* min, const QTime* max);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QDateTimeEdit_setTimeSpec(QDateTimeEdit* this_ptr, const Qt::TimeSpec* spec);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QDateTimeEdit_sizeHint_to_output(const QDateTimeEdit* this_ptr, QSize* output);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QDateTimeEdit_stepBy(QDateTimeEdit* this_ptr, int steps);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QDateTimeEdit_time_to_output(const QDateTimeEdit* this_ptr, QTime* output);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QDateTimeEdit_trUtf8_to_output(const char* s, const char* c, int n, QString* output);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QDateTimeEdit_tr_to_output(const char* s, const char* c, int n, QString* output);

} // extern "C"

#endif // QT_WIDGETS_C_QDATETIMEEDIT_H
