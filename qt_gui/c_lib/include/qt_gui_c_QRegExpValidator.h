#ifndef QT_GUI_C_QREGEXPVALIDATOR_H
#define QT_GUI_C_QREGEXPVALIDATOR_H

#include "qt_gui_c_global.h"

extern "C" {

QT_GUI_C_EXPORT QRegExpValidator* qt_gui_c_QRegExpValidator_G_dynamic_cast_QRegExpValidator_ptr(QValidator* ptr);
QT_GUI_C_EXPORT QObject* qt_gui_c_QRegExpValidator_G_static_cast_QObject_ptr(QRegExpValidator* ptr);
QT_GUI_C_EXPORT QRegExpValidator* qt_gui_c_QRegExpValidator_G_static_cast_QRegExpValidator_ptr_QObject(QObject* ptr);
QT_GUI_C_EXPORT QRegExpValidator* qt_gui_c_QRegExpValidator_G_static_cast_QRegExpValidator_ptr_QValidator(QValidator* ptr);
QT_GUI_C_EXPORT QValidator* qt_gui_c_QRegExpValidator_G_static_cast_QValidator_ptr(QRegExpValidator* ptr);
QT_GUI_C_EXPORT void qt_gui_c_QRegExpValidator_delete(QRegExpValidator* this_ptr);
QT_GUI_C_EXPORT const QMetaObject* qt_gui_c_QRegExpValidator_metaObject(const QRegExpValidator* this_ptr);
QT_GUI_C_EXPORT QRegExpValidator* qt_gui_c_QRegExpValidator_new_no_args();
QT_GUI_C_EXPORT QRegExpValidator* qt_gui_c_QRegExpValidator_new_parent(QObject* parent);
QT_GUI_C_EXPORT QRegExpValidator* qt_gui_c_QRegExpValidator_new_rx(const QRegExp* rx);
QT_GUI_C_EXPORT QRegExpValidator* qt_gui_c_QRegExpValidator_new_rx_parent(const QRegExp* rx, QObject* parent);
QT_GUI_C_EXPORT int qt_gui_c_QRegExpValidator_qt_metacall(QRegExpValidator* this_ptr, const QMetaObject::Call* arg1, int arg2, void** arg3);
QT_GUI_C_EXPORT void* qt_gui_c_QRegExpValidator_qt_metacast(QRegExpValidator* this_ptr, const char* arg1);
QT_GUI_C_EXPORT const QRegExp* qt_gui_c_QRegExpValidator_regExp(const QRegExpValidator* this_ptr);
QT_GUI_C_EXPORT void qt_gui_c_QRegExpValidator_setRegExp(QRegExpValidator* this_ptr, const QRegExp* rx);
QT_GUI_C_EXPORT void qt_gui_c_QRegExpValidator_trUtf8_to_output(const char* s, const char* c, int n, QString* output);
QT_GUI_C_EXPORT void qt_gui_c_QRegExpValidator_tr_to_output(const char* s, const char* c, int n, QString* output);

} // extern "C"

#endif // QT_GUI_C_QREGEXPVALIDATOR_H
