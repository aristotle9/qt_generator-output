#ifndef QT_CORE_C_QASSOCIATIVEITERABLE_H
#define QT_CORE_C_QASSOCIATIVEITERABLE_H

#include "qt_core_c_global.h"

extern "C" {

QT_CORE_C_EXPORT void qt_core_c_QAssociativeIterable_begin_to_output(const QAssociativeIterable* this_ptr, QAssociativeIterable::const_iterator* output);
QT_CORE_C_EXPORT void qt_core_c_QAssociativeIterable_const_iterator_constructor(const QAssociativeIterable::const_iterator* other, QAssociativeIterable::const_iterator* output);
QT_CORE_C_EXPORT void qt_core_c_QAssociativeIterable_const_iterator_destructor(QAssociativeIterable::const_iterator* this_ptr);
QT_CORE_C_EXPORT void qt_core_c_QAssociativeIterable_const_iterator_key_to_output(const QAssociativeIterable::const_iterator* this_ptr, QVariant* output);
QT_CORE_C_EXPORT QAssociativeIterable::const_iterator* qt_core_c_QAssociativeIterable_const_iterator_operator_add_assign(QAssociativeIterable::const_iterator* this_ptr, int j);
QT_CORE_C_EXPORT void qt_core_c_QAssociativeIterable_const_iterator_operator_add_to_output(const QAssociativeIterable::const_iterator* this_ptr, int j, QAssociativeIterable::const_iterator* output);
QT_CORE_C_EXPORT QAssociativeIterable::const_iterator* qt_core_c_QAssociativeIterable_const_iterator_operator_assign(QAssociativeIterable::const_iterator* this_ptr, const QAssociativeIterable::const_iterator* other);
QT_CORE_C_EXPORT QAssociativeIterable::const_iterator* qt_core_c_QAssociativeIterable_const_iterator_operator_dec(QAssociativeIterable::const_iterator* this_ptr);
QT_CORE_C_EXPORT void qt_core_c_QAssociativeIterable_const_iterator_operator_dec_postfix_to_output(QAssociativeIterable::const_iterator* this_ptr, int arg1, QAssociativeIterable::const_iterator* output);
QT_CORE_C_EXPORT bool qt_core_c_QAssociativeIterable_const_iterator_operator_eq(const QAssociativeIterable::const_iterator* this_ptr, const QAssociativeIterable::const_iterator* o);
QT_CORE_C_EXPORT QAssociativeIterable::const_iterator* qt_core_c_QAssociativeIterable_const_iterator_operator_inc(QAssociativeIterable::const_iterator* this_ptr);
QT_CORE_C_EXPORT void qt_core_c_QAssociativeIterable_const_iterator_operator_inc_postfix_to_output(QAssociativeIterable::const_iterator* this_ptr, int arg1, QAssociativeIterable::const_iterator* output);
QT_CORE_C_EXPORT void qt_core_c_QAssociativeIterable_const_iterator_operator_indirection_to_output(const QAssociativeIterable::const_iterator* this_ptr, QVariant* output);
QT_CORE_C_EXPORT bool qt_core_c_QAssociativeIterable_const_iterator_operator_neq(const QAssociativeIterable::const_iterator* this_ptr, const QAssociativeIterable::const_iterator* o);
QT_CORE_C_EXPORT QAssociativeIterable::const_iterator* qt_core_c_QAssociativeIterable_const_iterator_operator_sub_assign(QAssociativeIterable::const_iterator* this_ptr, int j);
QT_CORE_C_EXPORT void qt_core_c_QAssociativeIterable_const_iterator_operator_sub_to_output(const QAssociativeIterable::const_iterator* this_ptr, int j, QAssociativeIterable::const_iterator* output);
QT_CORE_C_EXPORT void qt_core_c_QAssociativeIterable_const_iterator_value_to_output(const QAssociativeIterable::const_iterator* this_ptr, QVariant* output);
QT_CORE_C_EXPORT void qt_core_c_QAssociativeIterable_destructor(QAssociativeIterable* this_ptr);
QT_CORE_C_EXPORT void qt_core_c_QAssociativeIterable_end_to_output(const QAssociativeIterable* this_ptr, QAssociativeIterable::const_iterator* output);
QT_CORE_C_EXPORT void qt_core_c_QAssociativeIterable_find_to_output(const QAssociativeIterable* this_ptr, const QVariant* key, QAssociativeIterable::const_iterator* output);
QT_CORE_C_EXPORT int qt_core_c_QAssociativeIterable_size(const QAssociativeIterable* this_ptr);
QT_CORE_C_EXPORT void qt_core_c_QAssociativeIterable_value_to_output(const QAssociativeIterable* this_ptr, const QVariant* key, QVariant* output);

} // extern "C"

#endif // QT_CORE_C_QASSOCIATIVEITERABLE_H
