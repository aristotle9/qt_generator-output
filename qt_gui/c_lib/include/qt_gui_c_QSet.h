#ifndef QT_GUI_C_QSET_H
#define QT_GUI_C_QSET_H

#include "qt_gui_c_global.h"

extern "C" {

QT_GUI_C_EXPORT int qt_gui_c_QSet_QByteArray_capacity(const QSet< QByteArray >* this_ptr);
QT_GUI_C_EXPORT void qt_gui_c_QSet_QByteArray_clear(QSet< QByteArray >* this_ptr);
QT_GUI_C_EXPORT void qt_gui_c_QSet_QByteArray_constructor(QSet< QByteArray >* output);
QT_GUI_C_EXPORT bool qt_gui_c_QSet_QByteArray_contains_set(const QSet< QByteArray >* this_ptr, const QSet< QByteArray >* set);
QT_GUI_C_EXPORT bool qt_gui_c_QSet_QByteArray_contains_value(const QSet< QByteArray >* this_ptr, const QByteArray* value);
QT_GUI_C_EXPORT int qt_gui_c_QSet_QByteArray_count(const QSet< QByteArray >* this_ptr);
QT_GUI_C_EXPORT void qt_gui_c_QSet_QByteArray_destructor(QSet< QByteArray >* this_ptr);
QT_GUI_C_EXPORT bool qt_gui_c_QSet_QByteArray_empty(const QSet< QByteArray >* this_ptr);
QT_GUI_C_EXPORT void qt_gui_c_QSet_QByteArray_fromList_to_output(const QList< QByteArray >* list, QSet< QByteArray >* output);
QT_GUI_C_EXPORT QSet< QByteArray >* qt_gui_c_QSet_QByteArray_intersect(QSet< QByteArray >* this_ptr, const QSet< QByteArray >* other);
QT_GUI_C_EXPORT bool qt_gui_c_QSet_QByteArray_intersects(const QSet< QByteArray >* this_ptr, const QSet< QByteArray >* other);
QT_GUI_C_EXPORT bool qt_gui_c_QSet_QByteArray_isEmpty(const QSet< QByteArray >* this_ptr);
QT_GUI_C_EXPORT QSet< QByteArray >* qt_gui_c_QSet_QByteArray_operator_add_assign_other(QSet< QByteArray >* this_ptr, const QSet< QByteArray >* other);
QT_GUI_C_EXPORT QSet< QByteArray >* qt_gui_c_QSet_QByteArray_operator_add_assign_value(QSet< QByteArray >* this_ptr, const QByteArray* value);
QT_GUI_C_EXPORT void qt_gui_c_QSet_QByteArray_operator_add_to_output(const QSet< QByteArray >* this_ptr, const QSet< QByteArray >* other, QSet< QByteArray >* output);
QT_GUI_C_EXPORT QSet< QByteArray >* qt_gui_c_QSet_QByteArray_operator_bit_and_assign_other(QSet< QByteArray >* this_ptr, const QSet< QByteArray >* other);
QT_GUI_C_EXPORT QSet< QByteArray >* qt_gui_c_QSet_QByteArray_operator_bit_and_assign_value(QSet< QByteArray >* this_ptr, const QByteArray* value);
QT_GUI_C_EXPORT void qt_gui_c_QSet_QByteArray_operator_bit_and_to_output(const QSet< QByteArray >* this_ptr, const QSet< QByteArray >* other, QSet< QByteArray >* output);
QT_GUI_C_EXPORT QSet< QByteArray >* qt_gui_c_QSet_QByteArray_operator_bit_or_assign_other(QSet< QByteArray >* this_ptr, const QSet< QByteArray >* other);
QT_GUI_C_EXPORT QSet< QByteArray >* qt_gui_c_QSet_QByteArray_operator_bit_or_assign_value(QSet< QByteArray >* this_ptr, const QByteArray* value);
QT_GUI_C_EXPORT void qt_gui_c_QSet_QByteArray_operator_bit_or_to_output(const QSet< QByteArray >* this_ptr, const QSet< QByteArray >* other, QSet< QByteArray >* output);
QT_GUI_C_EXPORT bool qt_gui_c_QSet_QByteArray_operator_eq(const QSet< QByteArray >* this_ptr, const QSet< QByteArray >* other);
QT_GUI_C_EXPORT bool qt_gui_c_QSet_QByteArray_operator_neq(const QSet< QByteArray >* this_ptr, const QSet< QByteArray >* other);
QT_GUI_C_EXPORT QSet< QByteArray >* qt_gui_c_QSet_QByteArray_operator_shl(QSet< QByteArray >* this_ptr, const QByteArray* value);
QT_GUI_C_EXPORT QSet< QByteArray >* qt_gui_c_QSet_QByteArray_operator_sub_assign_other(QSet< QByteArray >* this_ptr, const QSet< QByteArray >* other);
QT_GUI_C_EXPORT QSet< QByteArray >* qt_gui_c_QSet_QByteArray_operator_sub_assign_value(QSet< QByteArray >* this_ptr, const QByteArray* value);
QT_GUI_C_EXPORT void qt_gui_c_QSet_QByteArray_operator_sub_to_output(const QSet< QByteArray >* this_ptr, const QSet< QByteArray >* other, QSet< QByteArray >* output);
QT_GUI_C_EXPORT bool qt_gui_c_QSet_QByteArray_remove(QSet< QByteArray >* this_ptr, const QByteArray* value);
QT_GUI_C_EXPORT void qt_gui_c_QSet_QByteArray_reserve(QSet< QByteArray >* this_ptr, int size);
QT_GUI_C_EXPORT int qt_gui_c_QSet_QByteArray_size(const QSet< QByteArray >* this_ptr);
QT_GUI_C_EXPORT void qt_gui_c_QSet_QByteArray_squeeze(QSet< QByteArray >* this_ptr);
QT_GUI_C_EXPORT QSet< QByteArray >* qt_gui_c_QSet_QByteArray_subtract(QSet< QByteArray >* this_ptr, const QSet< QByteArray >* other);
QT_GUI_C_EXPORT void qt_gui_c_QSet_QByteArray_swap(QSet< QByteArray >* this_ptr, QSet< QByteArray >* other);
QT_GUI_C_EXPORT void qt_gui_c_QSet_QByteArray_toList_to_output(const QSet< QByteArray >* this_ptr, QList< QByteArray >* output);
QT_GUI_C_EXPORT QSet< QByteArray >* qt_gui_c_QSet_QByteArray_unite(QSet< QByteArray >* this_ptr, const QSet< QByteArray >* other);
QT_GUI_C_EXPORT void qt_gui_c_QSet_QByteArray_values_to_output(const QSet< QByteArray >* this_ptr, QList< QByteArray >* output);

} // extern "C"

#endif // QT_GUI_C_QSET_H
