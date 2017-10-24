#ifndef QT_CORE_C_QSET_H
#define QT_CORE_C_QSET_H

#include "qt_core_c_global.h"

extern "C" {

QT_CORE_C_EXPORT int qt_core_c_QSet_QAbstractState_ptr_capacity(const QSet< QAbstractState* >* this_ptr);
QT_CORE_C_EXPORT void qt_core_c_QSet_QAbstractState_ptr_clear(QSet< QAbstractState* >* this_ptr);
QT_CORE_C_EXPORT void qt_core_c_QSet_QAbstractState_ptr_constructor(QSet< QAbstractState* >* output);
QT_CORE_C_EXPORT bool qt_core_c_QSet_QAbstractState_ptr_contains_set(const QSet< QAbstractState* >* this_ptr, const QSet< QAbstractState* >* set);
QT_CORE_C_EXPORT bool qt_core_c_QSet_QAbstractState_ptr_contains_value(const QSet< QAbstractState* >* this_ptr, QAbstractState* const * value);
QT_CORE_C_EXPORT int qt_core_c_QSet_QAbstractState_ptr_count(const QSet< QAbstractState* >* this_ptr);
QT_CORE_C_EXPORT void qt_core_c_QSet_QAbstractState_ptr_destructor(QSet< QAbstractState* >* this_ptr);
QT_CORE_C_EXPORT bool qt_core_c_QSet_QAbstractState_ptr_empty(const QSet< QAbstractState* >* this_ptr);
QT_CORE_C_EXPORT void qt_core_c_QSet_QAbstractState_ptr_fromList_to_output(const QList< QAbstractState* >* list, QSet< QAbstractState* >* output);
QT_CORE_C_EXPORT QSet< QAbstractState* >* qt_core_c_QSet_QAbstractState_ptr_intersect(QSet< QAbstractState* >* this_ptr, const QSet< QAbstractState* >* other);
QT_CORE_C_EXPORT bool qt_core_c_QSet_QAbstractState_ptr_intersects(const QSet< QAbstractState* >* this_ptr, const QSet< QAbstractState* >* other);
QT_CORE_C_EXPORT bool qt_core_c_QSet_QAbstractState_ptr_isEmpty(const QSet< QAbstractState* >* this_ptr);
QT_CORE_C_EXPORT QSet< QAbstractState* >* qt_core_c_QSet_QAbstractState_ptr_operator_add_assign_other(QSet< QAbstractState* >* this_ptr, const QSet< QAbstractState* >* other);
QT_CORE_C_EXPORT QSet< QAbstractState* >* qt_core_c_QSet_QAbstractState_ptr_operator_add_assign_value(QSet< QAbstractState* >* this_ptr, QAbstractState* const * value);
QT_CORE_C_EXPORT void qt_core_c_QSet_QAbstractState_ptr_operator_add_to_output(const QSet< QAbstractState* >* this_ptr, const QSet< QAbstractState* >* other, QSet< QAbstractState* >* output);
QT_CORE_C_EXPORT QSet< QAbstractState* >* qt_core_c_QSet_QAbstractState_ptr_operator_bit_and_assign_other(QSet< QAbstractState* >* this_ptr, const QSet< QAbstractState* >* other);
QT_CORE_C_EXPORT QSet< QAbstractState* >* qt_core_c_QSet_QAbstractState_ptr_operator_bit_and_assign_value(QSet< QAbstractState* >* this_ptr, QAbstractState* const * value);
QT_CORE_C_EXPORT void qt_core_c_QSet_QAbstractState_ptr_operator_bit_and_to_output(const QSet< QAbstractState* >* this_ptr, const QSet< QAbstractState* >* other, QSet< QAbstractState* >* output);
QT_CORE_C_EXPORT QSet< QAbstractState* >* qt_core_c_QSet_QAbstractState_ptr_operator_bit_or_assign_other(QSet< QAbstractState* >* this_ptr, const QSet< QAbstractState* >* other);
QT_CORE_C_EXPORT QSet< QAbstractState* >* qt_core_c_QSet_QAbstractState_ptr_operator_bit_or_assign_value(QSet< QAbstractState* >* this_ptr, QAbstractState* const * value);
QT_CORE_C_EXPORT void qt_core_c_QSet_QAbstractState_ptr_operator_bit_or_to_output(const QSet< QAbstractState* >* this_ptr, const QSet< QAbstractState* >* other, QSet< QAbstractState* >* output);
QT_CORE_C_EXPORT bool qt_core_c_QSet_QAbstractState_ptr_operator_eq(const QSet< QAbstractState* >* this_ptr, const QSet< QAbstractState* >* other);
QT_CORE_C_EXPORT bool qt_core_c_QSet_QAbstractState_ptr_operator_neq(const QSet< QAbstractState* >* this_ptr, const QSet< QAbstractState* >* other);
QT_CORE_C_EXPORT QSet< QAbstractState* >* qt_core_c_QSet_QAbstractState_ptr_operator_shl(QSet< QAbstractState* >* this_ptr, QAbstractState* const * value);
QT_CORE_C_EXPORT QSet< QAbstractState* >* qt_core_c_QSet_QAbstractState_ptr_operator_sub_assign_other(QSet< QAbstractState* >* this_ptr, const QSet< QAbstractState* >* other);
QT_CORE_C_EXPORT QSet< QAbstractState* >* qt_core_c_QSet_QAbstractState_ptr_operator_sub_assign_value(QSet< QAbstractState* >* this_ptr, QAbstractState* const * value);
QT_CORE_C_EXPORT void qt_core_c_QSet_QAbstractState_ptr_operator_sub_to_output(const QSet< QAbstractState* >* this_ptr, const QSet< QAbstractState* >* other, QSet< QAbstractState* >* output);
QT_CORE_C_EXPORT bool qt_core_c_QSet_QAbstractState_ptr_remove(QSet< QAbstractState* >* this_ptr, QAbstractState* const * value);
QT_CORE_C_EXPORT void qt_core_c_QSet_QAbstractState_ptr_reserve(QSet< QAbstractState* >* this_ptr, int size);
QT_CORE_C_EXPORT int qt_core_c_QSet_QAbstractState_ptr_size(const QSet< QAbstractState* >* this_ptr);
QT_CORE_C_EXPORT void qt_core_c_QSet_QAbstractState_ptr_squeeze(QSet< QAbstractState* >* this_ptr);
QT_CORE_C_EXPORT QSet< QAbstractState* >* qt_core_c_QSet_QAbstractState_ptr_subtract(QSet< QAbstractState* >* this_ptr, const QSet< QAbstractState* >* other);
QT_CORE_C_EXPORT void qt_core_c_QSet_QAbstractState_ptr_swap(QSet< QAbstractState* >* this_ptr, QSet< QAbstractState* >* other);
QT_CORE_C_EXPORT void qt_core_c_QSet_QAbstractState_ptr_toList_to_output(const QSet< QAbstractState* >* this_ptr, QList< QAbstractState* >* output);
QT_CORE_C_EXPORT QSet< QAbstractState* >* qt_core_c_QSet_QAbstractState_ptr_unite(QSet< QAbstractState* >* this_ptr, const QSet< QAbstractState* >* other);
QT_CORE_C_EXPORT void qt_core_c_QSet_QAbstractState_ptr_values_to_output(const QSet< QAbstractState* >* this_ptr, QList< QAbstractState* >* output);

} // extern "C"

#endif // QT_CORE_C_QSET_H
