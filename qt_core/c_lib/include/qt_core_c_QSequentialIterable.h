#ifndef QT_CORE_C_QSEQUENTIALITERABLE_H
#define QT_CORE_C_QSEQUENTIALITERABLE_H

#include "qt_core_c_global.h"

extern "C" {

QT_CORE_C_EXPORT void qt_core_c_QSequentialIterable_at_to_output(const QSequentialIterable* this_ptr, int idx, QVariant* output);
QT_CORE_C_EXPORT void qt_core_c_QSequentialIterable_begin_to_output(const QSequentialIterable* this_ptr, QSequentialIterable::const_iterator* output);
QT_CORE_C_EXPORT bool qt_core_c_QSequentialIterable_canReverseIterate(const QSequentialIterable* this_ptr);
QT_CORE_C_EXPORT void qt_core_c_QSequentialIterable_const_iterator_constructor(const QSequentialIterable::const_iterator* other, QSequentialIterable::const_iterator* output);
QT_CORE_C_EXPORT void qt_core_c_QSequentialIterable_const_iterator_destructor(QSequentialIterable::const_iterator* this_ptr);
QT_CORE_C_EXPORT QSequentialIterable::const_iterator* qt_core_c_QSequentialIterable_const_iterator_operator_add_assign(QSequentialIterable::const_iterator* this_ptr, int j);
QT_CORE_C_EXPORT void qt_core_c_QSequentialIterable_const_iterator_operator_add_to_output(const QSequentialIterable::const_iterator* this_ptr, int j, QSequentialIterable::const_iterator* output);
QT_CORE_C_EXPORT QSequentialIterable::const_iterator* qt_core_c_QSequentialIterable_const_iterator_operator_assign(QSequentialIterable::const_iterator* this_ptr, const QSequentialIterable::const_iterator* other);
QT_CORE_C_EXPORT QSequentialIterable::const_iterator* qt_core_c_QSequentialIterable_const_iterator_operator_dec(QSequentialIterable::const_iterator* this_ptr);
QT_CORE_C_EXPORT void qt_core_c_QSequentialIterable_const_iterator_operator_dec_postfix_to_output(QSequentialIterable::const_iterator* this_ptr, int arg1, QSequentialIterable::const_iterator* output);
QT_CORE_C_EXPORT bool qt_core_c_QSequentialIterable_const_iterator_operator_eq(const QSequentialIterable::const_iterator* this_ptr, const QSequentialIterable::const_iterator* o);
QT_CORE_C_EXPORT QSequentialIterable::const_iterator* qt_core_c_QSequentialIterable_const_iterator_operator_inc(QSequentialIterable::const_iterator* this_ptr);
QT_CORE_C_EXPORT void qt_core_c_QSequentialIterable_const_iterator_operator_inc_postfix_to_output(QSequentialIterable::const_iterator* this_ptr, int arg1, QSequentialIterable::const_iterator* output);
QT_CORE_C_EXPORT void qt_core_c_QSequentialIterable_const_iterator_operator_indirection_to_output(const QSequentialIterable::const_iterator* this_ptr, QVariant* output);
QT_CORE_C_EXPORT bool qt_core_c_QSequentialIterable_const_iterator_operator_neq(const QSequentialIterable::const_iterator* this_ptr, const QSequentialIterable::const_iterator* o);
QT_CORE_C_EXPORT QSequentialIterable::const_iterator* qt_core_c_QSequentialIterable_const_iterator_operator_sub_assign(QSequentialIterable::const_iterator* this_ptr, int j);
QT_CORE_C_EXPORT void qt_core_c_QSequentialIterable_const_iterator_operator_sub_to_output(const QSequentialIterable::const_iterator* this_ptr, int j, QSequentialIterable::const_iterator* output);
QT_CORE_C_EXPORT void qt_core_c_QSequentialIterable_destructor(QSequentialIterable* this_ptr);
QT_CORE_C_EXPORT void qt_core_c_QSequentialIterable_end_to_output(const QSequentialIterable* this_ptr, QSequentialIterable::const_iterator* output);
QT_CORE_C_EXPORT int qt_core_c_QSequentialIterable_size(const QSequentialIterable* this_ptr);

} // extern "C"

#endif // QT_CORE_C_QSEQUENTIALITERABLE_H
