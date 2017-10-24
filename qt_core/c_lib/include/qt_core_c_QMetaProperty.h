#ifndef QT_CORE_C_QMETAPROPERTY_H
#define QT_CORE_C_QMETAPROPERTY_H

#include "qt_core_c_global.h"

extern "C" {

QT_CORE_C_EXPORT void qt_core_c_QMetaProperty_constructor(QMetaProperty* output);
QT_CORE_C_EXPORT void qt_core_c_QMetaProperty_destructor(QMetaProperty* this_ptr);
QT_CORE_C_EXPORT const QMetaObject* qt_core_c_QMetaProperty_enclosingMetaObject(const QMetaProperty* this_ptr);
QT_CORE_C_EXPORT void qt_core_c_QMetaProperty_enumerator_to_output(const QMetaProperty* this_ptr, QMetaEnum* output);
QT_CORE_C_EXPORT bool qt_core_c_QMetaProperty_hasNotifySignal(const QMetaProperty* this_ptr);
QT_CORE_C_EXPORT bool qt_core_c_QMetaProperty_hasStdCppSet(const QMetaProperty* this_ptr);
QT_CORE_C_EXPORT bool qt_core_c_QMetaProperty_isConstant(const QMetaProperty* this_ptr);
QT_CORE_C_EXPORT bool qt_core_c_QMetaProperty_isDesignable_no_args(const QMetaProperty* this_ptr);
QT_CORE_C_EXPORT bool qt_core_c_QMetaProperty_isDesignable_obj(const QMetaProperty* this_ptr, const QObject* obj);
QT_CORE_C_EXPORT bool qt_core_c_QMetaProperty_isEditable_no_args(const QMetaProperty* this_ptr);
QT_CORE_C_EXPORT bool qt_core_c_QMetaProperty_isEditable_obj(const QMetaProperty* this_ptr, const QObject* obj);
QT_CORE_C_EXPORT bool qt_core_c_QMetaProperty_isEnumType(const QMetaProperty* this_ptr);
QT_CORE_C_EXPORT bool qt_core_c_QMetaProperty_isFinal(const QMetaProperty* this_ptr);
QT_CORE_C_EXPORT bool qt_core_c_QMetaProperty_isFlagType(const QMetaProperty* this_ptr);
QT_CORE_C_EXPORT bool qt_core_c_QMetaProperty_isReadable(const QMetaProperty* this_ptr);
QT_CORE_C_EXPORT bool qt_core_c_QMetaProperty_isResettable(const QMetaProperty* this_ptr);
QT_CORE_C_EXPORT bool qt_core_c_QMetaProperty_isScriptable_no_args(const QMetaProperty* this_ptr);
QT_CORE_C_EXPORT bool qt_core_c_QMetaProperty_isScriptable_obj(const QMetaProperty* this_ptr, const QObject* obj);
QT_CORE_C_EXPORT bool qt_core_c_QMetaProperty_isStored_no_args(const QMetaProperty* this_ptr);
QT_CORE_C_EXPORT bool qt_core_c_QMetaProperty_isStored_obj(const QMetaProperty* this_ptr, const QObject* obj);
QT_CORE_C_EXPORT bool qt_core_c_QMetaProperty_isUser_no_args(const QMetaProperty* this_ptr);
QT_CORE_C_EXPORT bool qt_core_c_QMetaProperty_isUser_obj(const QMetaProperty* this_ptr, const QObject* obj);
QT_CORE_C_EXPORT bool qt_core_c_QMetaProperty_isValid(const QMetaProperty* this_ptr);
QT_CORE_C_EXPORT bool qt_core_c_QMetaProperty_isWritable(const QMetaProperty* this_ptr);
QT_CORE_C_EXPORT const char* qt_core_c_QMetaProperty_name(const QMetaProperty* this_ptr);
QT_CORE_C_EXPORT int qt_core_c_QMetaProperty_notifySignalIndex(const QMetaProperty* this_ptr);
QT_CORE_C_EXPORT void qt_core_c_QMetaProperty_notifySignal_to_output(const QMetaProperty* this_ptr, QMetaMethod* output);
QT_CORE_C_EXPORT int qt_core_c_QMetaProperty_propertyIndex(const QMetaProperty* this_ptr);
QT_CORE_C_EXPORT void qt_core_c_QMetaProperty_readOnGadget_to_output(const QMetaProperty* this_ptr, const void* gadget, QVariant* output);
QT_CORE_C_EXPORT void qt_core_c_QMetaProperty_read_to_output(const QMetaProperty* this_ptr, const QObject* obj, QVariant* output);
QT_CORE_C_EXPORT bool qt_core_c_QMetaProperty_reset(const QMetaProperty* this_ptr, QObject* obj);
QT_CORE_C_EXPORT bool qt_core_c_QMetaProperty_resetOnGadget(const QMetaProperty* this_ptr, void* gadget);
QT_CORE_C_EXPORT int qt_core_c_QMetaProperty_revision(const QMetaProperty* this_ptr);
QT_CORE_C_EXPORT const char* qt_core_c_QMetaProperty_typeName(const QMetaProperty* this_ptr);
QT_CORE_C_EXPORT int qt_core_c_QMetaProperty_userType(const QMetaProperty* this_ptr);
QT_CORE_C_EXPORT bool qt_core_c_QMetaProperty_write(const QMetaProperty* this_ptr, QObject* obj, const QVariant* value);
QT_CORE_C_EXPORT bool qt_core_c_QMetaProperty_writeOnGadget(const QMetaProperty* this_ptr, void* gadget, const QVariant* value);

} // extern "C"

#endif // QT_CORE_C_QMETAPROPERTY_H
