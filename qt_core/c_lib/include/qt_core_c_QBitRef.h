#ifndef QT_CORE_C_QBITREF_H
#define QT_CORE_C_QBITREF_H

#include "qt_core_c_global.h"

extern "C" {

QT_CORE_C_EXPORT bool qt_core_c_QBitRef_convert_to_bool(const QBitRef* this_ptr);
QT_CORE_C_EXPORT void qt_core_c_QBitRef_destructor(QBitRef* this_ptr);
QT_CORE_C_EXPORT QBitRef* qt_core_c_QBitRef_operator_assign_QBitRef(QBitRef* this_ptr, const QBitRef* val);
QT_CORE_C_EXPORT QBitRef* qt_core_c_QBitRef_operator_assign_bool(QBitRef* this_ptr, bool val);
QT_CORE_C_EXPORT bool qt_core_c_QBitRef_operator_not(const QBitRef* this_ptr);

} // extern "C"

#endif // QT_CORE_C_QBITREF_H
