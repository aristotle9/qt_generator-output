#ifndef QT_CORE_C_QMETACLASSINFO_H
#define QT_CORE_C_QMETACLASSINFO_H

#include "qt_core_c_global.h"

extern "C" {

QT_CORE_C_EXPORT void qt_core_c_QMetaClassInfo_constructor(QMetaClassInfo* output);
QT_CORE_C_EXPORT void qt_core_c_QMetaClassInfo_destructor(QMetaClassInfo* this_ptr);
QT_CORE_C_EXPORT const QMetaObject* qt_core_c_QMetaClassInfo_enclosingMetaObject(const QMetaClassInfo* this_ptr);
QT_CORE_C_EXPORT const char* qt_core_c_QMetaClassInfo_name(const QMetaClassInfo* this_ptr);
QT_CORE_C_EXPORT const char* qt_core_c_QMetaClassInfo_value(const QMetaClassInfo* this_ptr);

} // extern "C"

#endif // QT_CORE_C_QMETACLASSINFO_H
