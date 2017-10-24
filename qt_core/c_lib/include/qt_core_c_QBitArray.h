#ifndef QT_CORE_C_QBITARRAY_H
#define QT_CORE_C_QBITARRAY_H

#include "qt_core_c_global.h"

extern "C" {

QT_CORE_C_EXPORT void qt_core_c_QBitArray_G_operator_bit_and_to_output(const QBitArray* arg1, const QBitArray* arg2, QBitArray* output);
QT_CORE_C_EXPORT void qt_core_c_QBitArray_G_operator_bit_or_to_output(const QBitArray* arg1, const QBitArray* arg2, QBitArray* output);
QT_CORE_C_EXPORT void qt_core_c_QBitArray_G_operator_bit_xor_to_output(const QBitArray* arg1, const QBitArray* arg2, QBitArray* output);
QT_CORE_C_EXPORT void qt_core_c_QBitArray_G_operator_shl_to_output(const QDebug* arg1, const QBitArray* arg2, QDebug* output);
QT_CORE_C_EXPORT void qt_core_c_QBitArray_G_swap(QBitArray* value1, QBitArray* value2);
QT_CORE_C_EXPORT bool qt_core_c_QBitArray_at(const QBitArray* this_ptr, int i);
QT_CORE_C_EXPORT void qt_core_c_QBitArray_clear(QBitArray* this_ptr);
QT_CORE_C_EXPORT void qt_core_c_QBitArray_clearBit(QBitArray* this_ptr, int i);
QT_CORE_C_EXPORT void qt_core_c_QBitArray_constructor_no_args(QBitArray* output);
QT_CORE_C_EXPORT void qt_core_c_QBitArray_constructor_other(const QBitArray* other, QBitArray* output);
QT_CORE_C_EXPORT void qt_core_c_QBitArray_constructor_size(int size, QBitArray* output);
QT_CORE_C_EXPORT void qt_core_c_QBitArray_constructor_size_val(int size, bool val, QBitArray* output);
QT_CORE_C_EXPORT int qt_core_c_QBitArray_count_no_args(const QBitArray* this_ptr);
QT_CORE_C_EXPORT int qt_core_c_QBitArray_count_on(const QBitArray* this_ptr, bool on);
QT_CORE_C_EXPORT void qt_core_c_QBitArray_destructor(QBitArray* this_ptr);
QT_CORE_C_EXPORT bool qt_core_c_QBitArray_fill_val(QBitArray* this_ptr, bool val);
QT_CORE_C_EXPORT void qt_core_c_QBitArray_fill_val_first_last(QBitArray* this_ptr, bool val, int first, int last);
QT_CORE_C_EXPORT bool qt_core_c_QBitArray_fill_val_size(QBitArray* this_ptr, bool val, int size);
QT_CORE_C_EXPORT bool qt_core_c_QBitArray_isEmpty(const QBitArray* this_ptr);
QT_CORE_C_EXPORT bool qt_core_c_QBitArray_isNull(const QBitArray* this_ptr);
QT_CORE_C_EXPORT QBitArray* qt_core_c_QBitArray_operator_assign(QBitArray* this_ptr, const QBitArray* other);
QT_CORE_C_EXPORT QBitArray* qt_core_c_QBitArray_operator_bit_and_assign(QBitArray* this_ptr, const QBitArray* arg1);
QT_CORE_C_EXPORT void qt_core_c_QBitArray_operator_bit_not_to_output(const QBitArray* this_ptr, QBitArray* output);
QT_CORE_C_EXPORT QBitArray* qt_core_c_QBitArray_operator_bit_or_assign(QBitArray* this_ptr, const QBitArray* arg1);
QT_CORE_C_EXPORT QBitArray* qt_core_c_QBitArray_operator_bit_xor_assign(QBitArray* this_ptr, const QBitArray* arg1);
QT_CORE_C_EXPORT bool qt_core_c_QBitArray_operator_eq(const QBitArray* this_ptr, const QBitArray* other);
QT_CORE_C_EXPORT bool qt_core_c_QBitArray_operator_index_int(const QBitArray* this_ptr, int i);
QT_CORE_C_EXPORT void qt_core_c_QBitArray_operator_index_to_output_int(QBitArray* this_ptr, int i, QBitRef* output);
QT_CORE_C_EXPORT void qt_core_c_QBitArray_operator_index_to_output_unsigned_int(QBitArray* this_ptr, unsigned int i, QBitRef* output);
QT_CORE_C_EXPORT bool qt_core_c_QBitArray_operator_index_unsigned_int(const QBitArray* this_ptr, unsigned int i);
QT_CORE_C_EXPORT bool qt_core_c_QBitArray_operator_neq(const QBitArray* this_ptr, const QBitArray* other);
QT_CORE_C_EXPORT void qt_core_c_QBitArray_resize(QBitArray* this_ptr, int size);
QT_CORE_C_EXPORT void qt_core_c_QBitArray_setBit_i(QBitArray* this_ptr, int i);
QT_CORE_C_EXPORT void qt_core_c_QBitArray_setBit_i_val(QBitArray* this_ptr, int i, bool val);
QT_CORE_C_EXPORT int qt_core_c_QBitArray_size(const QBitArray* this_ptr);
QT_CORE_C_EXPORT void qt_core_c_QBitArray_swap(QBitArray* this_ptr, QBitArray* other);
QT_CORE_C_EXPORT bool qt_core_c_QBitArray_testBit(const QBitArray* this_ptr, int i);
QT_CORE_C_EXPORT bool qt_core_c_QBitArray_toggleBit(QBitArray* this_ptr, int i);
QT_CORE_C_EXPORT void qt_core_c_QBitArray_truncate(QBitArray* this_ptr, int pos);

} // extern "C"

#endif // QT_CORE_C_QBITARRAY_H
