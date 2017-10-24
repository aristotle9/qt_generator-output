#ifndef QT_GUI_C_QVALIDATOR_H
#define QT_GUI_C_QVALIDATOR_H

#include "qt_gui_c_global.h"

extern "C" {

QT_GUI_C_EXPORT QObject* qt_gui_c_QValidator_G_static_cast_QObject_ptr(QValidator* ptr);
QT_GUI_C_EXPORT QValidator* qt_gui_c_QValidator_G_static_cast_QValidator_ptr(QObject* ptr);
QT_GUI_C_EXPORT void qt_gui_c_QValidator_delete(QValidator* this_ptr);
QT_GUI_C_EXPORT void qt_gui_c_QValidator_fixup(const QValidator* this_ptr, QString* arg1);
QT_GUI_C_EXPORT void qt_gui_c_QValidator_locale_to_output(const QValidator* this_ptr, QLocale* output);
QT_GUI_C_EXPORT const QMetaObject* qt_gui_c_QValidator_metaObject(const QValidator* this_ptr);
QT_GUI_C_EXPORT int qt_gui_c_QValidator_qt_metacall(QValidator* this_ptr, const QMetaObject::Call* arg1, int arg2, void** arg3);
QT_GUI_C_EXPORT void* qt_gui_c_QValidator_qt_metacast(QValidator* this_ptr, const char* arg1);
QT_GUI_C_EXPORT void qt_gui_c_QValidator_setLocale(QValidator* this_ptr, const QLocale* locale);
QT_GUI_C_EXPORT void qt_gui_c_QValidator_trUtf8_to_output(const char* s, const char* c, int n, QString* output);
QT_GUI_C_EXPORT void qt_gui_c_QValidator_tr_to_output(const char* s, const char* c, int n, QString* output);
QT_GUI_C_EXPORT QValidator::State qt_gui_c_QValidator_validate(const QValidator* this_ptr, QString* arg1, int* arg2);

} // extern "C"

#endif // QT_GUI_C_QVALIDATOR_H
