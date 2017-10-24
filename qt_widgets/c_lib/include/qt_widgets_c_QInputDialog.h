#ifndef QT_WIDGETS_C_QINPUTDIALOG_H
#define QT_WIDGETS_C_QINPUTDIALOG_H

#include "qt_widgets_c_global.h"

extern "C" {

QT_WIDGETS_C_EXPORT QInputDialog* qt_widgets_c_QInputDialog_G_dynamic_cast_QInputDialog_ptr_QDialog(QDialog* ptr);
QT_WIDGETS_C_EXPORT QInputDialog* qt_widgets_c_QInputDialog_G_dynamic_cast_QInputDialog_ptr_QWidget(QWidget* ptr);
QT_WIDGETS_C_EXPORT QDialog* qt_widgets_c_QInputDialog_G_static_cast_QDialog_ptr(QInputDialog* ptr);
QT_WIDGETS_C_EXPORT QInputDialog* qt_widgets_c_QInputDialog_G_static_cast_QInputDialog_ptr_QDialog(QDialog* ptr);
QT_WIDGETS_C_EXPORT QInputDialog* qt_widgets_c_QInputDialog_G_static_cast_QInputDialog_ptr_QObject(QObject* ptr);
QT_WIDGETS_C_EXPORT QInputDialog* qt_widgets_c_QInputDialog_G_static_cast_QInputDialog_ptr_QPaintDevice(QPaintDevice* ptr);
QT_WIDGETS_C_EXPORT QInputDialog* qt_widgets_c_QInputDialog_G_static_cast_QInputDialog_ptr_QWidget(QWidget* ptr);
QT_WIDGETS_C_EXPORT QObject* qt_widgets_c_QInputDialog_G_static_cast_QObject_ptr(QInputDialog* ptr);
QT_WIDGETS_C_EXPORT QPaintDevice* qt_widgets_c_QInputDialog_G_static_cast_QPaintDevice_ptr(QInputDialog* ptr);
QT_WIDGETS_C_EXPORT QWidget* qt_widgets_c_QInputDialog_G_static_cast_QWidget_ptr(QInputDialog* ptr);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QInputDialog_cancelButtonText_to_output(const QInputDialog* this_ptr, QString* output);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QInputDialog_comboBoxItems_to_output(const QInputDialog* this_ptr, QStringList* output);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QInputDialog_delete(QInputDialog* this_ptr);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QInputDialog_done(QInputDialog* this_ptr, int result);
QT_WIDGETS_C_EXPORT int qt_widgets_c_QInputDialog_doubleDecimals(const QInputDialog* this_ptr);
QT_WIDGETS_C_EXPORT double qt_widgets_c_QInputDialog_doubleMaximum(const QInputDialog* this_ptr);
QT_WIDGETS_C_EXPORT double qt_widgets_c_QInputDialog_doubleMinimum(const QInputDialog* this_ptr);
QT_WIDGETS_C_EXPORT double qt_widgets_c_QInputDialog_doubleValue(const QInputDialog* this_ptr);
QT_WIDGETS_C_EXPORT QInputDialog::InputMode qt_widgets_c_QInputDialog_inputMode(const QInputDialog* this_ptr);
QT_WIDGETS_C_EXPORT int qt_widgets_c_QInputDialog_intMaximum(const QInputDialog* this_ptr);
QT_WIDGETS_C_EXPORT int qt_widgets_c_QInputDialog_intMinimum(const QInputDialog* this_ptr);
QT_WIDGETS_C_EXPORT int qt_widgets_c_QInputDialog_intStep(const QInputDialog* this_ptr);
QT_WIDGETS_C_EXPORT int qt_widgets_c_QInputDialog_intValue(const QInputDialog* this_ptr);
QT_WIDGETS_C_EXPORT bool qt_widgets_c_QInputDialog_isComboBoxEditable(const QInputDialog* this_ptr);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QInputDialog_labelText_to_output(const QInputDialog* this_ptr, QString* output);
QT_WIDGETS_C_EXPORT const QMetaObject* qt_widgets_c_QInputDialog_metaObject(const QInputDialog* this_ptr);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QInputDialog_minimumSizeHint_to_output(const QInputDialog* this_ptr, QSize* output);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QInputDialog_okButtonText_to_output(const QInputDialog* this_ptr, QString* output);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QInputDialog_open(QInputDialog* this_ptr, QObject* receiver, const char* member);
QT_WIDGETS_C_EXPORT unsigned int qt_widgets_c_QInputDialog_options(const QInputDialog* this_ptr);
QT_WIDGETS_C_EXPORT int qt_widgets_c_QInputDialog_qt_metacall(QInputDialog* this_ptr, const QMetaObject::Call* arg1, int arg2, void** arg3);
QT_WIDGETS_C_EXPORT void* qt_widgets_c_QInputDialog_qt_metacast(QInputDialog* this_ptr, const char* arg1);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QInputDialog_setCancelButtonText(QInputDialog* this_ptr, const QString* text);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QInputDialog_setComboBoxEditable(QInputDialog* this_ptr, bool editable);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QInputDialog_setComboBoxItems(QInputDialog* this_ptr, const QStringList* items);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QInputDialog_setDoubleDecimals(QInputDialog* this_ptr, int decimals);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QInputDialog_setDoubleMaximum(QInputDialog* this_ptr, double max);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QInputDialog_setDoubleMinimum(QInputDialog* this_ptr, double min);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QInputDialog_setDoubleRange(QInputDialog* this_ptr, double min, double max);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QInputDialog_setDoubleValue(QInputDialog* this_ptr, double value);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QInputDialog_setInputMode(QInputDialog* this_ptr, QInputDialog::InputMode mode);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QInputDialog_setIntMaximum(QInputDialog* this_ptr, int max);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QInputDialog_setIntMinimum(QInputDialog* this_ptr, int min);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QInputDialog_setIntRange(QInputDialog* this_ptr, int min, int max);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QInputDialog_setIntStep(QInputDialog* this_ptr, int step);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QInputDialog_setIntValue(QInputDialog* this_ptr, int value);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QInputDialog_setLabelText(QInputDialog* this_ptr, const QString* text);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QInputDialog_setOkButtonText(QInputDialog* this_ptr, const QString* text);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QInputDialog_setOption_option(QInputDialog* this_ptr, QInputDialog::InputDialogOption option);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QInputDialog_setOption_option_on(QInputDialog* this_ptr, QInputDialog::InputDialogOption option, bool on);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QInputDialog_setOptions(QInputDialog* this_ptr, unsigned int options);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QInputDialog_setTextEchoMode(QInputDialog* this_ptr, const QLineEdit::EchoMode* mode);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QInputDialog_setTextValue(QInputDialog* this_ptr, const QString* text);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QInputDialog_setVisible(QInputDialog* this_ptr, bool visible);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QInputDialog_sizeHint_to_output(const QInputDialog* this_ptr, QSize* output);
QT_WIDGETS_C_EXPORT bool qt_widgets_c_QInputDialog_testOption(const QInputDialog* this_ptr, QInputDialog::InputDialogOption option);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QInputDialog_textValue_to_output(const QInputDialog* this_ptr, QString* output);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QInputDialog_trUtf8_to_output(const char* s, const char* c, int n, QString* output);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QInputDialog_tr_to_output(const char* s, const char* c, int n, QString* output);

} // extern "C"

#endif // QT_WIDGETS_C_QINPUTDIALOG_H
