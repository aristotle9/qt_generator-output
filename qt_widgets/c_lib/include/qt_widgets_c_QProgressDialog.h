#ifndef QT_WIDGETS_C_QPROGRESSDIALOG_H
#define QT_WIDGETS_C_QPROGRESSDIALOG_H

#include "qt_widgets_c_global.h"

extern "C" {

QT_WIDGETS_C_EXPORT QProgressDialog* qt_widgets_c_QProgressDialog_G_dynamic_cast_QProgressDialog_ptr_QDialog(QDialog* ptr);
QT_WIDGETS_C_EXPORT QProgressDialog* qt_widgets_c_QProgressDialog_G_dynamic_cast_QProgressDialog_ptr_QWidget(QWidget* ptr);
QT_WIDGETS_C_EXPORT QDialog* qt_widgets_c_QProgressDialog_G_static_cast_QDialog_ptr(QProgressDialog* ptr);
QT_WIDGETS_C_EXPORT QObject* qt_widgets_c_QProgressDialog_G_static_cast_QObject_ptr(QProgressDialog* ptr);
QT_WIDGETS_C_EXPORT QPaintDevice* qt_widgets_c_QProgressDialog_G_static_cast_QPaintDevice_ptr(QProgressDialog* ptr);
QT_WIDGETS_C_EXPORT QProgressDialog* qt_widgets_c_QProgressDialog_G_static_cast_QProgressDialog_ptr_QDialog(QDialog* ptr);
QT_WIDGETS_C_EXPORT QProgressDialog* qt_widgets_c_QProgressDialog_G_static_cast_QProgressDialog_ptr_QObject(QObject* ptr);
QT_WIDGETS_C_EXPORT QProgressDialog* qt_widgets_c_QProgressDialog_G_static_cast_QProgressDialog_ptr_QPaintDevice(QPaintDevice* ptr);
QT_WIDGETS_C_EXPORT QProgressDialog* qt_widgets_c_QProgressDialog_G_static_cast_QProgressDialog_ptr_QWidget(QWidget* ptr);
QT_WIDGETS_C_EXPORT QWidget* qt_widgets_c_QProgressDialog_G_static_cast_QWidget_ptr(QProgressDialog* ptr);
QT_WIDGETS_C_EXPORT bool qt_widgets_c_QProgressDialog_autoClose(const QProgressDialog* this_ptr);
QT_WIDGETS_C_EXPORT bool qt_widgets_c_QProgressDialog_autoReset(const QProgressDialog* this_ptr);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QProgressDialog_cancel(QProgressDialog* this_ptr);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QProgressDialog_delete(QProgressDialog* this_ptr);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QProgressDialog_labelText_to_output(const QProgressDialog* this_ptr, QString* output);
QT_WIDGETS_C_EXPORT int qt_widgets_c_QProgressDialog_maximum(const QProgressDialog* this_ptr);
QT_WIDGETS_C_EXPORT const QMetaObject* qt_widgets_c_QProgressDialog_metaObject(const QProgressDialog* this_ptr);
QT_WIDGETS_C_EXPORT int qt_widgets_c_QProgressDialog_minimum(const QProgressDialog* this_ptr);
QT_WIDGETS_C_EXPORT int qt_widgets_c_QProgressDialog_minimumDuration(const QProgressDialog* this_ptr);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QProgressDialog_open(QProgressDialog* this_ptr, QObject* receiver, const char* member);
QT_WIDGETS_C_EXPORT int qt_widgets_c_QProgressDialog_qt_metacall(QProgressDialog* this_ptr, const QMetaObject::Call* arg1, int arg2, void** arg3);
QT_WIDGETS_C_EXPORT void* qt_widgets_c_QProgressDialog_qt_metacast(QProgressDialog* this_ptr, const char* arg1);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QProgressDialog_reset(QProgressDialog* this_ptr);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QProgressDialog_setAutoClose(QProgressDialog* this_ptr, bool close);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QProgressDialog_setAutoReset(QProgressDialog* this_ptr, bool reset);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QProgressDialog_setBar(QProgressDialog* this_ptr, QProgressBar* bar);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QProgressDialog_setCancelButton(QProgressDialog* this_ptr, QPushButton* button);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QProgressDialog_setCancelButtonText(QProgressDialog* this_ptr, const QString* text);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QProgressDialog_setLabel(QProgressDialog* this_ptr, QLabel* label);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QProgressDialog_setLabelText(QProgressDialog* this_ptr, const QString* text);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QProgressDialog_setMaximum(QProgressDialog* this_ptr, int maximum);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QProgressDialog_setMinimum(QProgressDialog* this_ptr, int minimum);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QProgressDialog_setMinimumDuration(QProgressDialog* this_ptr, int ms);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QProgressDialog_setRange(QProgressDialog* this_ptr, int minimum, int maximum);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QProgressDialog_setValue(QProgressDialog* this_ptr, int progress);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QProgressDialog_sizeHint_to_output(const QProgressDialog* this_ptr, QSize* output);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QProgressDialog_trUtf8_to_output(const char* s, const char* c, int n, QString* output);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QProgressDialog_tr_to_output(const char* s, const char* c, int n, QString* output);
QT_WIDGETS_C_EXPORT int qt_widgets_c_QProgressDialog_value(const QProgressDialog* this_ptr);
QT_WIDGETS_C_EXPORT bool qt_widgets_c_QProgressDialog_wasCanceled(const QProgressDialog* this_ptr);

} // extern "C"

#endif // QT_WIDGETS_C_QPROGRESSDIALOG_H
