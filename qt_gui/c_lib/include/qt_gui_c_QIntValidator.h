#ifndef QT_GUI_C_QINTVALIDATOR_H
#define QT_GUI_C_QINTVALIDATOR_H

#include "qt_gui_c_global.h"

extern "C" {

QT_GUI_C_EXPORT QIntValidator* qt_gui_c_QIntValidator_G_dynamic_cast_QIntValidator_ptr(QValidator* ptr);
QT_GUI_C_EXPORT QIntValidator* qt_gui_c_QIntValidator_G_static_cast_QIntValidator_ptr_QObject(QObject* ptr);
QT_GUI_C_EXPORT QIntValidator* qt_gui_c_QIntValidator_G_static_cast_QIntValidator_ptr_QValidator(QValidator* ptr);
QT_GUI_C_EXPORT QObject* qt_gui_c_QIntValidator_G_static_cast_QObject_ptr(QIntValidator* ptr);
QT_GUI_C_EXPORT QValidator* qt_gui_c_QIntValidator_G_static_cast_QValidator_ptr(QIntValidator* ptr);
QT_GUI_C_EXPORT int qt_gui_c_QIntValidator_bottom(const QIntValidator* this_ptr);
QT_GUI_C_EXPORT void qt_gui_c_QIntValidator_delete(QIntValidator* this_ptr);
QT_GUI_C_EXPORT void qt_gui_c_QIntValidator_fixup(const QIntValidator* this_ptr, QString* input);
QT_GUI_C_EXPORT const QMetaObject* qt_gui_c_QIntValidator_metaObject(const QIntValidator* this_ptr);
QT_GUI_C_EXPORT QIntValidator* qt_gui_c_QIntValidator_new_bottom_top(int bottom, int top);
QT_GUI_C_EXPORT QIntValidator* qt_gui_c_QIntValidator_new_bottom_top_parent(int bottom, int top, QObject* parent);
QT_GUI_C_EXPORT QIntValidator* qt_gui_c_QIntValidator_new_no_args();
QT_GUI_C_EXPORT QIntValidator* qt_gui_c_QIntValidator_new_parent(QObject* parent);
QT_GUI_C_EXPORT int qt_gui_c_QIntValidator_qt_metacall(QIntValidator* this_ptr, const QMetaObject::Call* arg1, int arg2, void** arg3);
QT_GUI_C_EXPORT void* qt_gui_c_QIntValidator_qt_metacast(QIntValidator* this_ptr, const char* arg1);
QT_GUI_C_EXPORT void qt_gui_c_QIntValidator_setBottom(QIntValidator* this_ptr, int arg1);
QT_GUI_C_EXPORT void qt_gui_c_QIntValidator_setRange(QIntValidator* this_ptr, int bottom, int top);
QT_GUI_C_EXPORT void qt_gui_c_QIntValidator_setTop(QIntValidator* this_ptr, int arg1);
QT_GUI_C_EXPORT int qt_gui_c_QIntValidator_top(const QIntValidator* this_ptr);
QT_GUI_C_EXPORT void qt_gui_c_QIntValidator_trUtf8_to_output(const char* s, const char* c, int n, QString* output);
QT_GUI_C_EXPORT void qt_gui_c_QIntValidator_tr_to_output(const char* s, const char* c, int n, QString* output);

} // extern "C"

#endif // QT_GUI_C_QINTVALIDATOR_H
