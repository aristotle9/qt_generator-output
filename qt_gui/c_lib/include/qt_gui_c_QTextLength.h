#ifndef QT_GUI_C_QTEXTLENGTH_H
#define QT_GUI_C_QTEXTLENGTH_H

#include "qt_gui_c_global.h"

extern "C" {

QT_GUI_C_EXPORT void qt_gui_c_QTextLength_constructor_no_args(QTextLength* output);
QT_GUI_C_EXPORT void qt_gui_c_QTextLength_constructor_type_value(QTextLength::Type type, double value, QTextLength* output);
QT_GUI_C_EXPORT void qt_gui_c_QTextLength_convert_to_QVariant_to_output(const QTextLength* this_ptr, QVariant* output);
QT_GUI_C_EXPORT void qt_gui_c_QTextLength_destructor(QTextLength* this_ptr);
QT_GUI_C_EXPORT bool qt_gui_c_QTextLength_operator_eq(const QTextLength* this_ptr, const QTextLength* other);
QT_GUI_C_EXPORT bool qt_gui_c_QTextLength_operator_neq(const QTextLength* this_ptr, const QTextLength* other);
QT_GUI_C_EXPORT double qt_gui_c_QTextLength_rawValue(const QTextLength* this_ptr);
QT_GUI_C_EXPORT QTextLength::Type qt_gui_c_QTextLength_type(const QTextLength* this_ptr);
QT_GUI_C_EXPORT double qt_gui_c_QTextLength_value(const QTextLength* this_ptr, double maximumLength);

} // extern "C"

#endif // QT_GUI_C_QTEXTLENGTH_H
