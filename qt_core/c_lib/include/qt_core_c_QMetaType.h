#ifndef QT_CORE_C_QMETATYPE_H
#define QT_CORE_C_QMETATYPE_H

#include "qt_core_c_global.h"

extern "C" {

QT_CORE_C_EXPORT bool qt_core_c_QMetaType_compare(const void* lhs, const void* rhs, int typeId, int* result);
QT_CORE_C_EXPORT void* qt_core_c_QMetaType_construct_type_where_copy(int type, void* where, const void* copy);
QT_CORE_C_EXPORT void* qt_core_c_QMetaType_construct_where(const QMetaType* this_ptr, void* where);
QT_CORE_C_EXPORT void* qt_core_c_QMetaType_construct_where_copy(const QMetaType* this_ptr, void* where, const void* copy);
QT_CORE_C_EXPORT void qt_core_c_QMetaType_constructor(const int type, QMetaType* output);
QT_CORE_C_EXPORT bool qt_core_c_QMetaType_convert(const void* from, int fromTypeId, void* to, int toTypeId);
QT_CORE_C_EXPORT void* qt_core_c_QMetaType_create_copy(const QMetaType* this_ptr, const void* copy);
QT_CORE_C_EXPORT void* qt_core_c_QMetaType_create_no_args(const QMetaType* this_ptr);
QT_CORE_C_EXPORT void* qt_core_c_QMetaType_create_type(int type);
QT_CORE_C_EXPORT void* qt_core_c_QMetaType_create_type_copy(int type, const void* copy);
QT_CORE_C_EXPORT bool qt_core_c_QMetaType_debugStream(QDebug* dbg, const void* rhs, int typeId);
QT_CORE_C_EXPORT void qt_core_c_QMetaType_destroy_data(const QMetaType* this_ptr, void* data);
QT_CORE_C_EXPORT void qt_core_c_QMetaType_destroy_type_data(int type, void* data);
QT_CORE_C_EXPORT void qt_core_c_QMetaType_destruct_data(const QMetaType* this_ptr, void* data);
QT_CORE_C_EXPORT void qt_core_c_QMetaType_destruct_type_where(int type, void* where);
QT_CORE_C_EXPORT void qt_core_c_QMetaType_destructor(QMetaType* this_ptr);
QT_CORE_C_EXPORT bool qt_core_c_QMetaType_equals(const void* lhs, const void* rhs, int typeId, int* result);
QT_CORE_C_EXPORT unsigned int qt_core_c_QMetaType_flags(const QMetaType* this_ptr);
QT_CORE_C_EXPORT bool qt_core_c_QMetaType_hasRegisteredComparators(int typeId);
QT_CORE_C_EXPORT bool qt_core_c_QMetaType_hasRegisteredConverterFunction(int fromTypeId, int toTypeId);
QT_CORE_C_EXPORT bool qt_core_c_QMetaType_hasRegisteredDebugStreamOperator(int typeId);
QT_CORE_C_EXPORT bool qt_core_c_QMetaType_isRegistered_no_args(const QMetaType* this_ptr);
QT_CORE_C_EXPORT bool qt_core_c_QMetaType_isRegistered_type(int type);
QT_CORE_C_EXPORT bool qt_core_c_QMetaType_isValid(const QMetaType* this_ptr);
QT_CORE_C_EXPORT bool qt_core_c_QMetaType_load(QDataStream* stream, int type, void* data);
QT_CORE_C_EXPORT const QMetaObject* qt_core_c_QMetaType_metaObject(const QMetaType* this_ptr);
QT_CORE_C_EXPORT const QMetaObject* qt_core_c_QMetaType_metaObjectForType(int type);
QT_CORE_C_EXPORT int qt_core_c_QMetaType_registerNormalizedTypedef(const QByteArray* normalizedTypeName, int aliasId);
QT_CORE_C_EXPORT int qt_core_c_QMetaType_registerType(const char* typeName, void (*deleter)(void*), void* (*creator)(const void*));
QT_CORE_C_EXPORT int qt_core_c_QMetaType_registerTypedef(const char* typeName, int aliasId);
QT_CORE_C_EXPORT bool qt_core_c_QMetaType_save(QDataStream* stream, int type, const void* data);
QT_CORE_C_EXPORT int qt_core_c_QMetaType_sizeOf_no_args(const QMetaType* this_ptr);
QT_CORE_C_EXPORT int qt_core_c_QMetaType_sizeOf_type(int type);
QT_CORE_C_EXPORT unsigned int qt_core_c_QMetaType_typeFlags(int type);
QT_CORE_C_EXPORT const char* qt_core_c_QMetaType_typeName(int type);
QT_CORE_C_EXPORT int qt_core_c_QMetaType_type_QByteArray(const QByteArray* typeName);
QT_CORE_C_EXPORT int qt_core_c_QMetaType_type_char(const char* typeName);
QT_CORE_C_EXPORT bool qt_core_c_QMetaType_unregisterType(int type);

} // extern "C"

#endif // QT_CORE_C_QMETATYPE_H
