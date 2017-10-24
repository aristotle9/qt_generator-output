#ifndef QT_GUI_C_QDOUBLEVALIDATOR_H
#define QT_GUI_C_QDOUBLEVALIDATOR_H

#include "qt_gui_c_global.h"

extern "C" {

QT_GUI_C_EXPORT QDoubleValidator* qt_gui_c_QDoubleValidator_G_dynamic_cast_QDoubleValidator_ptr(QValidator* ptr);
QT_GUI_C_EXPORT QDoubleValidator* qt_gui_c_QDoubleValidator_G_static_cast_QDoubleValidator_ptr_QObject(QObject* ptr);
QT_GUI_C_EXPORT QDoubleValidator* qt_gui_c_QDoubleValidator_G_static_cast_QDoubleValidator_ptr_QValidator(QValidator* ptr);
QT_GUI_C_EXPORT QObject* qt_gui_c_QDoubleValidator_G_static_cast_QObject_ptr(QDoubleValidator* ptr);
QT_GUI_C_EXPORT QValidator* qt_gui_c_QDoubleValidator_G_static_cast_QValidator_ptr(QDoubleValidator* ptr);
QT_GUI_C_EXPORT double qt_gui_c_QDoubleValidator_bottom(const QDoubleValidator* this_ptr);
QT_GUI_C_EXPORT int qt_gui_c_QDoubleValidator_decimals(const QDoubleValidator* this_ptr);
QT_GUI_C_EXPORT void qt_gui_c_QDoubleValidator_delete(QDoubleValidator* this_ptr);
QT_GUI_C_EXPORT const QMetaObject* qt_gui_c_QDoubleValidator_metaObject(const QDoubleValidator* this_ptr);
QT_GUI_C_EXPORT QDoubleValidator* qt_gui_c_QDoubleValidator_new_bottom_top_decimals(double bottom, double top, int decimals);
QT_GUI_C_EXPORT QDoubleValidator* qt_gui_c_QDoubleValidator_new_bottom_top_decimals_parent(double bottom, double top, int decimals, QObject* parent);
QT_GUI_C_EXPORT QDoubleValidator* qt_gui_c_QDoubleValidator_new_no_args();
QT_GUI_C_EXPORT QDoubleValidator* qt_gui_c_QDoubleValidator_new_parent(QObject* parent);
QT_GUI_C_EXPORT QDoubleValidator::Notation qt_gui_c_QDoubleValidator_notation(const QDoubleValidator* this_ptr);
QT_GUI_C_EXPORT int qt_gui_c_QDoubleValidator_qt_metacall(QDoubleValidator* this_ptr, const QMetaObject::Call* arg1, int arg2, void** arg3);
QT_GUI_C_EXPORT void* qt_gui_c_QDoubleValidator_qt_metacast(QDoubleValidator* this_ptr, const char* arg1);
QT_GUI_C_EXPORT void qt_gui_c_QDoubleValidator_setBottom(QDoubleValidator* this_ptr, double arg1);
QT_GUI_C_EXPORT void qt_gui_c_QDoubleValidator_setDecimals(QDoubleValidator* this_ptr, int arg1);
QT_GUI_C_EXPORT void qt_gui_c_QDoubleValidator_setNotation(QDoubleValidator* this_ptr, QDoubleValidator::Notation arg1);
QT_GUI_C_EXPORT void qt_gui_c_QDoubleValidator_setRange_bottom_top(QDoubleValidator* this_ptr, double bottom, double top);
QT_GUI_C_EXPORT void qt_gui_c_QDoubleValidator_setRange_bottom_top_decimals(QDoubleValidator* this_ptr, double bottom, double top, int decimals);
QT_GUI_C_EXPORT void qt_gui_c_QDoubleValidator_setTop(QDoubleValidator* this_ptr, double arg1);
QT_GUI_C_EXPORT double qt_gui_c_QDoubleValidator_top(const QDoubleValidator* this_ptr);
QT_GUI_C_EXPORT void qt_gui_c_QDoubleValidator_trUtf8_to_output(const char* s, const char* c, int n, QString* output);
QT_GUI_C_EXPORT void qt_gui_c_QDoubleValidator_tr_to_output(const char* s, const char* c, int n, QString* output);

} // extern "C"

#endif // QT_GUI_C_QDOUBLEVALIDATOR_H
