#ifndef QT_CORE_C_QJSONARRAY_H
#define QT_CORE_C_QJSONARRAY_H

#include "qt_core_c_global.h"

extern "C" {

QT_CORE_C_EXPORT void qt_core_c_QJsonArray_append(QJsonArray* this_ptr, const QJsonValue* value);
QT_CORE_C_EXPORT void qt_core_c_QJsonArray_at_to_output(const QJsonArray* this_ptr, int i, QJsonValue* output);
QT_CORE_C_EXPORT void qt_core_c_QJsonArray_begin_to_output(QJsonArray* this_ptr, QJsonArray::iterator* output);
QT_CORE_C_EXPORT void qt_core_c_QJsonArray_begin_to_output_const(const QJsonArray* this_ptr, QJsonArray::const_iterator* output);
QT_CORE_C_EXPORT void qt_core_c_QJsonArray_constBegin_to_output(const QJsonArray* this_ptr, QJsonArray::const_iterator* output);
QT_CORE_C_EXPORT void qt_core_c_QJsonArray_constEnd_to_output(const QJsonArray* this_ptr, QJsonArray::const_iterator* output);
QT_CORE_C_EXPORT const QJsonArray* qt_core_c_QJsonArray_const_iterator_a(const QJsonArray::const_iterator* this_ptr);
QT_CORE_C_EXPORT void qt_core_c_QJsonArray_const_iterator_constructor_QJsonArray_const_iterator(const QJsonArray::const_iterator* o, QJsonArray::const_iterator* output);
QT_CORE_C_EXPORT void qt_core_c_QJsonArray_const_iterator_constructor_QJsonArray_int(const QJsonArray* array, int index, QJsonArray::const_iterator* output);
QT_CORE_C_EXPORT void qt_core_c_QJsonArray_const_iterator_constructor_QJsonArray_iterator(const QJsonArray::iterator* o, QJsonArray::const_iterator* output);
QT_CORE_C_EXPORT void qt_core_c_QJsonArray_const_iterator_constructor_no_args(QJsonArray::const_iterator* output);
QT_CORE_C_EXPORT void qt_core_c_QJsonArray_const_iterator_destructor(QJsonArray::const_iterator* this_ptr);
QT_CORE_C_EXPORT int qt_core_c_QJsonArray_const_iterator_i(const QJsonArray::const_iterator* this_ptr);
QT_CORE_C_EXPORT QJsonArray::const_iterator* qt_core_c_QJsonArray_const_iterator_operator_add_assign(QJsonArray::const_iterator* this_ptr, int j);
QT_CORE_C_EXPORT void qt_core_c_QJsonArray_const_iterator_operator_add_to_output(const QJsonArray::const_iterator* this_ptr, int j, QJsonArray::const_iterator* output);
QT_CORE_C_EXPORT QJsonArray::const_iterator* qt_core_c_QJsonArray_const_iterator_operator_dec(QJsonArray::const_iterator* this_ptr);
QT_CORE_C_EXPORT void qt_core_c_QJsonArray_const_iterator_operator_dec_postfix_to_output(QJsonArray::const_iterator* this_ptr, int arg1, QJsonArray::const_iterator* output);
QT_CORE_C_EXPORT bool qt_core_c_QJsonArray_const_iterator_operator_eq(const QJsonArray::const_iterator* this_ptr, const QJsonArray::const_iterator* o);
QT_CORE_C_EXPORT bool qt_core_c_QJsonArray_const_iterator_operator_ge(const QJsonArray::const_iterator* this_ptr, const QJsonArray::const_iterator* other);
QT_CORE_C_EXPORT bool qt_core_c_QJsonArray_const_iterator_operator_gt(const QJsonArray::const_iterator* this_ptr, const QJsonArray::const_iterator* other);
QT_CORE_C_EXPORT QJsonArray::const_iterator* qt_core_c_QJsonArray_const_iterator_operator_inc(QJsonArray::const_iterator* this_ptr);
QT_CORE_C_EXPORT void qt_core_c_QJsonArray_const_iterator_operator_inc_postfix_to_output(QJsonArray::const_iterator* this_ptr, int arg1, QJsonArray::const_iterator* output);
QT_CORE_C_EXPORT void qt_core_c_QJsonArray_const_iterator_operator_index_to_output(const QJsonArray::const_iterator* this_ptr, int j, QJsonValue* output);
QT_CORE_C_EXPORT void qt_core_c_QJsonArray_const_iterator_operator_indirection_to_output(const QJsonArray::const_iterator* this_ptr, QJsonValue* output);
QT_CORE_C_EXPORT bool qt_core_c_QJsonArray_const_iterator_operator_le(const QJsonArray::const_iterator* this_ptr, const QJsonArray::const_iterator* other);
QT_CORE_C_EXPORT bool qt_core_c_QJsonArray_const_iterator_operator_lt(const QJsonArray::const_iterator* this_ptr, const QJsonArray::const_iterator* other);
QT_CORE_C_EXPORT bool qt_core_c_QJsonArray_const_iterator_operator_neq(const QJsonArray::const_iterator* this_ptr, const QJsonArray::const_iterator* o);
QT_CORE_C_EXPORT int qt_core_c_QJsonArray_const_iterator_operator_sub(const QJsonArray::const_iterator* this_ptr, const QJsonArray::const_iterator* j);
QT_CORE_C_EXPORT QJsonArray::const_iterator* qt_core_c_QJsonArray_const_iterator_operator_sub_assign(QJsonArray::const_iterator* this_ptr, int j);
QT_CORE_C_EXPORT void qt_core_c_QJsonArray_const_iterator_operator_sub_to_output(const QJsonArray::const_iterator* this_ptr, int j, QJsonArray::const_iterator* output);
QT_CORE_C_EXPORT void qt_core_c_QJsonArray_const_iterator_set_a(QJsonArray::const_iterator* this_ptr, const QJsonArray* value);
QT_CORE_C_EXPORT void qt_core_c_QJsonArray_const_iterator_set_i(QJsonArray::const_iterator* this_ptr, int value);
QT_CORE_C_EXPORT void qt_core_c_QJsonArray_constructor_no_args(QJsonArray* output);
QT_CORE_C_EXPORT void qt_core_c_QJsonArray_constructor_other(const QJsonArray* other, QJsonArray* output);
QT_CORE_C_EXPORT bool qt_core_c_QJsonArray_contains(const QJsonArray* this_ptr, const QJsonValue* element);
QT_CORE_C_EXPORT int qt_core_c_QJsonArray_count(const QJsonArray* this_ptr);
QT_CORE_C_EXPORT void qt_core_c_QJsonArray_destructor(QJsonArray* this_ptr);
QT_CORE_C_EXPORT bool qt_core_c_QJsonArray_empty(const QJsonArray* this_ptr);
QT_CORE_C_EXPORT void qt_core_c_QJsonArray_end_to_output(QJsonArray* this_ptr, QJsonArray::iterator* output);
QT_CORE_C_EXPORT void qt_core_c_QJsonArray_end_to_output_const(const QJsonArray* this_ptr, QJsonArray::const_iterator* output);
QT_CORE_C_EXPORT void qt_core_c_QJsonArray_erase_to_output(QJsonArray* this_ptr, const QJsonArray::iterator* it, QJsonArray::iterator* output);
QT_CORE_C_EXPORT void qt_core_c_QJsonArray_first_to_output(const QJsonArray* this_ptr, QJsonValue* output);
QT_CORE_C_EXPORT void qt_core_c_QJsonArray_fromStringList_to_output(const QStringList* list, QJsonArray* output);
QT_CORE_C_EXPORT void qt_core_c_QJsonArray_fromVariantList_to_output(const QList< QVariant >* list, QJsonArray* output);
QT_CORE_C_EXPORT void qt_core_c_QJsonArray_insert(QJsonArray* this_ptr, int i, const QJsonValue* value);
QT_CORE_C_EXPORT void qt_core_c_QJsonArray_insert_to_output(QJsonArray* this_ptr, const QJsonArray::iterator* before, const QJsonValue* value, QJsonArray::iterator* output);
QT_CORE_C_EXPORT bool qt_core_c_QJsonArray_isEmpty(const QJsonArray* this_ptr);
QT_CORE_C_EXPORT QJsonArray* qt_core_c_QJsonArray_iterator_a(const QJsonArray::iterator* this_ptr);
QT_CORE_C_EXPORT void qt_core_c_QJsonArray_iterator_constructor_array_index(QJsonArray* array, int index, QJsonArray::iterator* output);
QT_CORE_C_EXPORT void qt_core_c_QJsonArray_iterator_constructor_no_args(QJsonArray::iterator* output);
QT_CORE_C_EXPORT void qt_core_c_QJsonArray_iterator_destructor(QJsonArray::iterator* this_ptr);
QT_CORE_C_EXPORT int qt_core_c_QJsonArray_iterator_i(const QJsonArray::iterator* this_ptr);
QT_CORE_C_EXPORT QJsonArray::iterator* qt_core_c_QJsonArray_iterator_operator_add_assign(QJsonArray::iterator* this_ptr, int j);
QT_CORE_C_EXPORT void qt_core_c_QJsonArray_iterator_operator_add_to_output(const QJsonArray::iterator* this_ptr, int j, QJsonArray::iterator* output);
QT_CORE_C_EXPORT QJsonArray::iterator* qt_core_c_QJsonArray_iterator_operator_dec(QJsonArray::iterator* this_ptr);
QT_CORE_C_EXPORT void qt_core_c_QJsonArray_iterator_operator_dec_postfix_to_output(QJsonArray::iterator* this_ptr, int arg1, QJsonArray::iterator* output);
QT_CORE_C_EXPORT bool qt_core_c_QJsonArray_iterator_operator_eq_QJsonArray_const_iterator(const QJsonArray::iterator* this_ptr, const QJsonArray::const_iterator* o);
QT_CORE_C_EXPORT bool qt_core_c_QJsonArray_iterator_operator_eq_QJsonArray_iterator(const QJsonArray::iterator* this_ptr, const QJsonArray::iterator* o);
QT_CORE_C_EXPORT bool qt_core_c_QJsonArray_iterator_operator_ge_QJsonArray_const_iterator(const QJsonArray::iterator* this_ptr, const QJsonArray::const_iterator* other);
QT_CORE_C_EXPORT bool qt_core_c_QJsonArray_iterator_operator_ge_QJsonArray_iterator(const QJsonArray::iterator* this_ptr, const QJsonArray::iterator* other);
QT_CORE_C_EXPORT bool qt_core_c_QJsonArray_iterator_operator_gt_QJsonArray_const_iterator(const QJsonArray::iterator* this_ptr, const QJsonArray::const_iterator* other);
QT_CORE_C_EXPORT bool qt_core_c_QJsonArray_iterator_operator_gt_QJsonArray_iterator(const QJsonArray::iterator* this_ptr, const QJsonArray::iterator* other);
QT_CORE_C_EXPORT QJsonArray::iterator* qt_core_c_QJsonArray_iterator_operator_inc(QJsonArray::iterator* this_ptr);
QT_CORE_C_EXPORT void qt_core_c_QJsonArray_iterator_operator_inc_postfix_to_output(QJsonArray::iterator* this_ptr, int arg1, QJsonArray::iterator* output);
QT_CORE_C_EXPORT void qt_core_c_QJsonArray_iterator_operator_index_to_output(const QJsonArray::iterator* this_ptr, int j, QJsonValueRef* output);
QT_CORE_C_EXPORT void qt_core_c_QJsonArray_iterator_operator_indirection_to_output(const QJsonArray::iterator* this_ptr, QJsonValueRef* output);
QT_CORE_C_EXPORT bool qt_core_c_QJsonArray_iterator_operator_le_QJsonArray_const_iterator(const QJsonArray::iterator* this_ptr, const QJsonArray::const_iterator* other);
QT_CORE_C_EXPORT bool qt_core_c_QJsonArray_iterator_operator_le_QJsonArray_iterator(const QJsonArray::iterator* this_ptr, const QJsonArray::iterator* other);
QT_CORE_C_EXPORT bool qt_core_c_QJsonArray_iterator_operator_lt_QJsonArray_const_iterator(const QJsonArray::iterator* this_ptr, const QJsonArray::const_iterator* other);
QT_CORE_C_EXPORT bool qt_core_c_QJsonArray_iterator_operator_lt_QJsonArray_iterator(const QJsonArray::iterator* this_ptr, const QJsonArray::iterator* other);
QT_CORE_C_EXPORT bool qt_core_c_QJsonArray_iterator_operator_neq_QJsonArray_const_iterator(const QJsonArray::iterator* this_ptr, const QJsonArray::const_iterator* o);
QT_CORE_C_EXPORT bool qt_core_c_QJsonArray_iterator_operator_neq_QJsonArray_iterator(const QJsonArray::iterator* this_ptr, const QJsonArray::iterator* o);
QT_CORE_C_EXPORT int qt_core_c_QJsonArray_iterator_operator_sub(const QJsonArray::iterator* this_ptr, const QJsonArray::iterator* j);
QT_CORE_C_EXPORT QJsonArray::iterator* qt_core_c_QJsonArray_iterator_operator_sub_assign(QJsonArray::iterator* this_ptr, int j);
QT_CORE_C_EXPORT void qt_core_c_QJsonArray_iterator_operator_sub_to_output(const QJsonArray::iterator* this_ptr, int j, QJsonArray::iterator* output);
QT_CORE_C_EXPORT void qt_core_c_QJsonArray_iterator_set_a(QJsonArray::iterator* this_ptr, QJsonArray* value);
QT_CORE_C_EXPORT void qt_core_c_QJsonArray_iterator_set_i(QJsonArray::iterator* this_ptr, int value);
QT_CORE_C_EXPORT void qt_core_c_QJsonArray_last_to_output(const QJsonArray* this_ptr, QJsonValue* output);
QT_CORE_C_EXPORT QJsonArray* qt_core_c_QJsonArray_operator_add_assign(QJsonArray* this_ptr, const QJsonValue* v);
QT_CORE_C_EXPORT void qt_core_c_QJsonArray_operator_add_to_output(const QJsonArray* this_ptr, const QJsonValue* v, QJsonArray* output);
QT_CORE_C_EXPORT QJsonArray* qt_core_c_QJsonArray_operator_assign(QJsonArray* this_ptr, const QJsonArray* other);
QT_CORE_C_EXPORT bool qt_core_c_QJsonArray_operator_eq(const QJsonArray* this_ptr, const QJsonArray* other);
QT_CORE_C_EXPORT void qt_core_c_QJsonArray_operator_index_to_output(QJsonArray* this_ptr, int i, QJsonValueRef* output);
QT_CORE_C_EXPORT void qt_core_c_QJsonArray_operator_index_to_output_const(const QJsonArray* this_ptr, int i, QJsonValue* output);
QT_CORE_C_EXPORT bool qt_core_c_QJsonArray_operator_neq(const QJsonArray* this_ptr, const QJsonArray* other);
QT_CORE_C_EXPORT QJsonArray* qt_core_c_QJsonArray_operator_shl(QJsonArray* this_ptr, const QJsonValue* v);
QT_CORE_C_EXPORT void qt_core_c_QJsonArray_pop_back(QJsonArray* this_ptr);
QT_CORE_C_EXPORT void qt_core_c_QJsonArray_pop_front(QJsonArray* this_ptr);
QT_CORE_C_EXPORT void qt_core_c_QJsonArray_prepend(QJsonArray* this_ptr, const QJsonValue* value);
QT_CORE_C_EXPORT void qt_core_c_QJsonArray_push_back(QJsonArray* this_ptr, const QJsonValue* t);
QT_CORE_C_EXPORT void qt_core_c_QJsonArray_push_front(QJsonArray* this_ptr, const QJsonValue* t);
QT_CORE_C_EXPORT void qt_core_c_QJsonArray_removeAt(QJsonArray* this_ptr, int i);
QT_CORE_C_EXPORT void qt_core_c_QJsonArray_removeFirst(QJsonArray* this_ptr);
QT_CORE_C_EXPORT void qt_core_c_QJsonArray_removeLast(QJsonArray* this_ptr);
QT_CORE_C_EXPORT void qt_core_c_QJsonArray_replace(QJsonArray* this_ptr, int i, const QJsonValue* value);
QT_CORE_C_EXPORT int qt_core_c_QJsonArray_size(const QJsonArray* this_ptr);
QT_CORE_C_EXPORT void qt_core_c_QJsonArray_takeAt_to_output(QJsonArray* this_ptr, int i, QJsonValue* output);
QT_CORE_C_EXPORT void qt_core_c_QJsonArray_toVariantList_to_output(const QJsonArray* this_ptr, QList< QVariant >* output);

} // extern "C"

#endif // QT_CORE_C_QJSONARRAY_H
