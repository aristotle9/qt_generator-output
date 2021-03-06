#ifndef QT_GUI_C_QREGULAREXPRESSIONVALIDATOR_H
#define QT_GUI_C_QREGULAREXPRESSIONVALIDATOR_H

#include "qt_gui_c_global.h"

extern "C" {

QT_GUI_C_EXPORT QRegularExpressionValidator* qt_gui_c_QRegularExpressionValidator_G_dynamic_cast_QRegularExpressionValidator_ptr(QValidator* ptr);
QT_GUI_C_EXPORT QObject* qt_gui_c_QRegularExpressionValidator_G_static_cast_QObject_ptr(QRegularExpressionValidator* ptr);
QT_GUI_C_EXPORT QRegularExpressionValidator* qt_gui_c_QRegularExpressionValidator_G_static_cast_QRegularExpressionValidator_ptr_QObject(QObject* ptr);
QT_GUI_C_EXPORT QRegularExpressionValidator* qt_gui_c_QRegularExpressionValidator_G_static_cast_QRegularExpressionValidator_ptr_QValidator(QValidator* ptr);
QT_GUI_C_EXPORT QValidator* qt_gui_c_QRegularExpressionValidator_G_static_cast_QValidator_ptr(QRegularExpressionValidator* ptr);
QT_GUI_C_EXPORT void qt_gui_c_QRegularExpressionValidator_delete(QRegularExpressionValidator* this_ptr);
QT_GUI_C_EXPORT const QMetaObject* qt_gui_c_QRegularExpressionValidator_metaObject(const QRegularExpressionValidator* this_ptr);
QT_GUI_C_EXPORT QRegularExpressionValidator* qt_gui_c_QRegularExpressionValidator_new_no_args();
QT_GUI_C_EXPORT QRegularExpressionValidator* qt_gui_c_QRegularExpressionValidator_new_parent(QObject* parent);
QT_GUI_C_EXPORT QRegularExpressionValidator* qt_gui_c_QRegularExpressionValidator_new_re(const QRegularExpression* re);
QT_GUI_C_EXPORT QRegularExpressionValidator* qt_gui_c_QRegularExpressionValidator_new_re_parent(const QRegularExpression* re, QObject* parent);
QT_GUI_C_EXPORT int qt_gui_c_QRegularExpressionValidator_qt_metacall(QRegularExpressionValidator* this_ptr, const QMetaObject::Call* arg1, int arg2, void** arg3);
QT_GUI_C_EXPORT void* qt_gui_c_QRegularExpressionValidator_qt_metacast(QRegularExpressionValidator* this_ptr, const char* arg1);
QT_GUI_C_EXPORT void qt_gui_c_QRegularExpressionValidator_regularExpression_to_output(const QRegularExpressionValidator* this_ptr, QRegularExpression* output);
QT_GUI_C_EXPORT void qt_gui_c_QRegularExpressionValidator_setRegularExpression(QRegularExpressionValidator* this_ptr, const QRegularExpression* re);
QT_GUI_C_EXPORT void qt_gui_c_QRegularExpressionValidator_trUtf8_to_output(const char* s, const char* c, int n, QString* output);
QT_GUI_C_EXPORT void qt_gui_c_QRegularExpressionValidator_tr_to_output(const char* s, const char* c, int n, QString* output);

} // extern "C"

#endif // QT_GUI_C_QREGULAREXPRESSIONVALIDATOR_H
