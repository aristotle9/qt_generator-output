#ifndef QT_CORE_C_QGENERICARGUMENT_H
#define QT_CORE_C_QGENERICARGUMENT_H

#include "qt_core_c_global.h"

extern "C" {

QT_CORE_C_EXPORT void qt_core_c_QGenericArgument_constructor_aName(const char* aName, QGenericArgument* output);
QT_CORE_C_EXPORT void qt_core_c_QGenericArgument_constructor_aName_aData(const char* aName, const void* aData, QGenericArgument* output);
QT_CORE_C_EXPORT void qt_core_c_QGenericArgument_constructor_no_args(QGenericArgument* output);
QT_CORE_C_EXPORT void* qt_core_c_QGenericArgument_data(const QGenericArgument* this_ptr);
QT_CORE_C_EXPORT void qt_core_c_QGenericArgument_destructor(QGenericArgument* this_ptr);
QT_CORE_C_EXPORT const char* qt_core_c_QGenericArgument_name(const QGenericArgument* this_ptr);

} // extern "C"

#endif // QT_CORE_C_QGENERICARGUMENT_H
