#ifndef QT_CORE_C_QMETAOBJECT_H
#define QT_CORE_C_QMETAOBJECT_H

#include "qt_core_c_global.h"

extern "C" {

QT_CORE_C_EXPORT void qt_core_c_QMetaObject_Connection_constructor_no_args(QMetaObject::Connection* output);
QT_CORE_C_EXPORT void qt_core_c_QMetaObject_Connection_constructor_other(const QMetaObject::Connection* other, QMetaObject::Connection* output);
QT_CORE_C_EXPORT void qt_core_c_QMetaObject_Connection_destructor(QMetaObject::Connection* this_ptr);
QT_CORE_C_EXPORT QMetaObject::Connection* qt_core_c_QMetaObject_Connection_operator_assign(QMetaObject::Connection* this_ptr, const QMetaObject::Connection* other);
QT_CORE_C_EXPORT void qt_core_c_QMetaObject_activate_sender_arg2_local_signal_index_argv(QObject* sender, const QMetaObject* arg2, int local_signal_index, void** argv);
QT_CORE_C_EXPORT void qt_core_c_QMetaObject_activate_sender_signal_index_argv(QObject* sender, int signal_index, void** argv);
QT_CORE_C_EXPORT void qt_core_c_QMetaObject_activate_sender_signal_offset_local_signal_index_argv(QObject* sender, int signal_offset, int local_signal_index, void** argv);
QT_CORE_C_EXPORT QObject* qt_core_c_QMetaObject_cast_QObject_ptr(const QMetaObject* this_ptr, QObject* obj);
QT_CORE_C_EXPORT const QObject* qt_core_c_QMetaObject_cast_const_QObject_ptr(const QMetaObject* this_ptr, const QObject* obj);
QT_CORE_C_EXPORT bool qt_core_c_QMetaObject_checkConnectArgs_QMetaMethod_QMetaMethod(const QMetaMethod* signal, const QMetaMethod* method);
QT_CORE_C_EXPORT bool qt_core_c_QMetaObject_checkConnectArgs_char_char(const char* signal, const char* method);
QT_CORE_C_EXPORT int qt_core_c_QMetaObject_classInfoCount(const QMetaObject* this_ptr);
QT_CORE_C_EXPORT int qt_core_c_QMetaObject_classInfoOffset(const QMetaObject* this_ptr);
QT_CORE_C_EXPORT void qt_core_c_QMetaObject_classInfo_to_output(const QMetaObject* this_ptr, int index, QMetaClassInfo* output);
QT_CORE_C_EXPORT const char* qt_core_c_QMetaObject_className(const QMetaObject* this_ptr);
QT_CORE_C_EXPORT void qt_core_c_QMetaObject_connectSlotsByName(QObject* o);
QT_CORE_C_EXPORT void qt_core_c_QMetaObject_connect_to_output_sender_signal_index_receiver_method_index(const QObject* sender, int signal_index, const QObject* receiver, int method_index, QMetaObject::Connection* output);
QT_CORE_C_EXPORT void qt_core_c_QMetaObject_connect_to_output_sender_signal_index_receiver_method_index_type(const QObject* sender, int signal_index, const QObject* receiver, int method_index, int type, QMetaObject::Connection* output);
QT_CORE_C_EXPORT void qt_core_c_QMetaObject_connect_to_output_sender_signal_index_receiver_method_index_type_types(const QObject* sender, int signal_index, const QObject* receiver, int method_index, int type, int* types, QMetaObject::Connection* output);
QT_CORE_C_EXPORT int qt_core_c_QMetaObject_constructorCount(const QMetaObject* this_ptr);
QT_CORE_C_EXPORT void qt_core_c_QMetaObject_constructor_to_output(const QMetaObject* this_ptr, int index, QMetaMethod* output);
QT_CORE_C_EXPORT void qt_core_c_QMetaObject_delete(QMetaObject* this_ptr);
QT_CORE_C_EXPORT bool qt_core_c_QMetaObject_disconnect(const QObject* sender, int signal_index, const QObject* receiver, int method_index);
QT_CORE_C_EXPORT bool qt_core_c_QMetaObject_disconnectOne(const QObject* sender, int signal_index, const QObject* receiver, int method_index);
QT_CORE_C_EXPORT int qt_core_c_QMetaObject_enumeratorCount(const QMetaObject* this_ptr);
QT_CORE_C_EXPORT int qt_core_c_QMetaObject_enumeratorOffset(const QMetaObject* this_ptr);
QT_CORE_C_EXPORT void qt_core_c_QMetaObject_enumerator_to_output(const QMetaObject* this_ptr, int index, QMetaEnum* output);
QT_CORE_C_EXPORT int qt_core_c_QMetaObject_indexOfClassInfo(const QMetaObject* this_ptr, const char* name);
QT_CORE_C_EXPORT int qt_core_c_QMetaObject_indexOfConstructor(const QMetaObject* this_ptr, const char* constructor);
QT_CORE_C_EXPORT int qt_core_c_QMetaObject_indexOfEnumerator(const QMetaObject* this_ptr, const char* name);
QT_CORE_C_EXPORT int qt_core_c_QMetaObject_indexOfMethod(const QMetaObject* this_ptr, const char* method);
QT_CORE_C_EXPORT int qt_core_c_QMetaObject_indexOfProperty(const QMetaObject* this_ptr, const char* name);
QT_CORE_C_EXPORT int qt_core_c_QMetaObject_indexOfSignal(const QMetaObject* this_ptr, const char* signal);
QT_CORE_C_EXPORT int qt_core_c_QMetaObject_indexOfSlot(const QMetaObject* this_ptr, const char* slot);
QT_CORE_C_EXPORT bool qt_core_c_QMetaObject_inherits(const QMetaObject* this_ptr, const QMetaObject* metaObject);
QT_CORE_C_EXPORT bool qt_core_c_QMetaObject_invokeMethod_obj_member(QObject* obj, const char* member);
QT_CORE_C_EXPORT bool qt_core_c_QMetaObject_invokeMethod_obj_member_arg3_ret(QObject* obj, const char* member, const Qt::ConnectionType* arg3, const QGenericReturnArgument* ret);
QT_CORE_C_EXPORT bool qt_core_c_QMetaObject_invokeMethod_obj_member_arg3_ret_val0(QObject* obj, const char* member, const Qt::ConnectionType* arg3, const QGenericReturnArgument* ret, const QGenericArgument* val0);
QT_CORE_C_EXPORT bool qt_core_c_QMetaObject_invokeMethod_obj_member_arg3_ret_val0_val1(QObject* obj, const char* member, const Qt::ConnectionType* arg3, const QGenericReturnArgument* ret, const QGenericArgument* val0, const QGenericArgument* val1);
QT_CORE_C_EXPORT bool qt_core_c_QMetaObject_invokeMethod_obj_member_arg3_ret_val0_val1_val2(QObject* obj, const char* member, const Qt::ConnectionType* arg3, const QGenericReturnArgument* ret, const QGenericArgument* val0, const QGenericArgument* val1, const QGenericArgument* val2);
QT_CORE_C_EXPORT bool qt_core_c_QMetaObject_invokeMethod_obj_member_arg3_ret_val0_val1_val2_val3(QObject* obj, const char* member, const Qt::ConnectionType* arg3, const QGenericReturnArgument* ret, const QGenericArgument* val0, const QGenericArgument* val1, const QGenericArgument* val2, const QGenericArgument* val3);
QT_CORE_C_EXPORT bool qt_core_c_QMetaObject_invokeMethod_obj_member_arg3_ret_val0_val1_val2_val3_val4(QObject* obj, const char* member, const Qt::ConnectionType* arg3, const QGenericReturnArgument* ret, const QGenericArgument* val0, const QGenericArgument* val1, const QGenericArgument* val2, const QGenericArgument* val3, const QGenericArgument* val4);
QT_CORE_C_EXPORT bool qt_core_c_QMetaObject_invokeMethod_obj_member_arg3_ret_val0_val1_val2_val3_val4_val5(QObject* obj, const char* member, const Qt::ConnectionType* arg3, const QGenericReturnArgument* ret, const QGenericArgument* val0, const QGenericArgument* val1, const QGenericArgument* val2, const QGenericArgument* val3, const QGenericArgument* val4, const QGenericArgument* val5);
QT_CORE_C_EXPORT bool qt_core_c_QMetaObject_invokeMethod_obj_member_arg3_ret_val0_val1_val2_val3_val4_val5_val6(QObject* obj, const char* member, const Qt::ConnectionType* arg3, const QGenericReturnArgument* ret, const QGenericArgument* val0, const QGenericArgument* val1, const QGenericArgument* val2, const QGenericArgument* val3, const QGenericArgument* val4, const QGenericArgument* val5, const QGenericArgument* val6);
QT_CORE_C_EXPORT bool qt_core_c_QMetaObject_invokeMethod_obj_member_arg3_ret_val0_val1_val2_val3_val4_val5_val6_val7(QObject* obj, const char* member, const Qt::ConnectionType* arg3, const QGenericReturnArgument* ret, const QGenericArgument* val0, const QGenericArgument* val1, const QGenericArgument* val2, const QGenericArgument* val3, const QGenericArgument* val4, const QGenericArgument* val5, const QGenericArgument* val6, const QGenericArgument* val7);
QT_CORE_C_EXPORT bool qt_core_c_QMetaObject_invokeMethod_obj_member_arg3_ret_val0_val1_val2_val3_val4_val5_val6_val7_val8(QObject* obj, const char* member, const Qt::ConnectionType* arg3, const QGenericReturnArgument* ret, const QGenericArgument* val0, const QGenericArgument* val1, const QGenericArgument* val2, const QGenericArgument* val3, const QGenericArgument* val4, const QGenericArgument* val5, const QGenericArgument* val6, const QGenericArgument* val7, const QGenericArgument* val8);
QT_CORE_C_EXPORT bool qt_core_c_QMetaObject_invokeMethod_obj_member_arg3_ret_val0_val1_val2_val3_val4_val5_val6_val7_val8_val9(QObject* obj, const char* member, const Qt::ConnectionType* arg3, const QGenericReturnArgument* ret, const QGenericArgument* val0, const QGenericArgument* val1, const QGenericArgument* val2, const QGenericArgument* val3, const QGenericArgument* val4, const QGenericArgument* val5, const QGenericArgument* val6, const QGenericArgument* val7, const QGenericArgument* val8, const QGenericArgument* val9);
QT_CORE_C_EXPORT bool qt_core_c_QMetaObject_invokeMethod_obj_member_ret(QObject* obj, const char* member, const QGenericReturnArgument* ret);
QT_CORE_C_EXPORT bool qt_core_c_QMetaObject_invokeMethod_obj_member_ret_val0(QObject* obj, const char* member, const QGenericReturnArgument* ret, const QGenericArgument* val0);
QT_CORE_C_EXPORT bool qt_core_c_QMetaObject_invokeMethod_obj_member_ret_val0_val1(QObject* obj, const char* member, const QGenericReturnArgument* ret, const QGenericArgument* val0, const QGenericArgument* val1);
QT_CORE_C_EXPORT bool qt_core_c_QMetaObject_invokeMethod_obj_member_ret_val0_val1_val2(QObject* obj, const char* member, const QGenericReturnArgument* ret, const QGenericArgument* val0, const QGenericArgument* val1, const QGenericArgument* val2);
QT_CORE_C_EXPORT bool qt_core_c_QMetaObject_invokeMethod_obj_member_ret_val0_val1_val2_val3(QObject* obj, const char* member, const QGenericReturnArgument* ret, const QGenericArgument* val0, const QGenericArgument* val1, const QGenericArgument* val2, const QGenericArgument* val3);
QT_CORE_C_EXPORT bool qt_core_c_QMetaObject_invokeMethod_obj_member_ret_val0_val1_val2_val3_val4(QObject* obj, const char* member, const QGenericReturnArgument* ret, const QGenericArgument* val0, const QGenericArgument* val1, const QGenericArgument* val2, const QGenericArgument* val3, const QGenericArgument* val4);
QT_CORE_C_EXPORT bool qt_core_c_QMetaObject_invokeMethod_obj_member_ret_val0_val1_val2_val3_val4_val5(QObject* obj, const char* member, const QGenericReturnArgument* ret, const QGenericArgument* val0, const QGenericArgument* val1, const QGenericArgument* val2, const QGenericArgument* val3, const QGenericArgument* val4, const QGenericArgument* val5);
QT_CORE_C_EXPORT bool qt_core_c_QMetaObject_invokeMethod_obj_member_ret_val0_val1_val2_val3_val4_val5_val6(QObject* obj, const char* member, const QGenericReturnArgument* ret, const QGenericArgument* val0, const QGenericArgument* val1, const QGenericArgument* val2, const QGenericArgument* val3, const QGenericArgument* val4, const QGenericArgument* val5, const QGenericArgument* val6);
QT_CORE_C_EXPORT bool qt_core_c_QMetaObject_invokeMethod_obj_member_ret_val0_val1_val2_val3_val4_val5_val6_val7(QObject* obj, const char* member, const QGenericReturnArgument* ret, const QGenericArgument* val0, const QGenericArgument* val1, const QGenericArgument* val2, const QGenericArgument* val3, const QGenericArgument* val4, const QGenericArgument* val5, const QGenericArgument* val6, const QGenericArgument* val7);
QT_CORE_C_EXPORT bool qt_core_c_QMetaObject_invokeMethod_obj_member_ret_val0_val1_val2_val3_val4_val5_val6_val7_val8(QObject* obj, const char* member, const QGenericReturnArgument* ret, const QGenericArgument* val0, const QGenericArgument* val1, const QGenericArgument* val2, const QGenericArgument* val3, const QGenericArgument* val4, const QGenericArgument* val5, const QGenericArgument* val6, const QGenericArgument* val7, const QGenericArgument* val8);
QT_CORE_C_EXPORT bool qt_core_c_QMetaObject_invokeMethod_obj_member_ret_val0_val1_val2_val3_val4_val5_val6_val7_val8_val9(QObject* obj, const char* member, const QGenericReturnArgument* ret, const QGenericArgument* val0, const QGenericArgument* val1, const QGenericArgument* val2, const QGenericArgument* val3, const QGenericArgument* val4, const QGenericArgument* val5, const QGenericArgument* val6, const QGenericArgument* val7, const QGenericArgument* val8, const QGenericArgument* val9);
QT_CORE_C_EXPORT bool qt_core_c_QMetaObject_invokeMethod_obj_member_type(QObject* obj, const char* member, const Qt::ConnectionType* type);
QT_CORE_C_EXPORT bool qt_core_c_QMetaObject_invokeMethod_obj_member_type_val0(QObject* obj, const char* member, const Qt::ConnectionType* type, const QGenericArgument* val0);
QT_CORE_C_EXPORT bool qt_core_c_QMetaObject_invokeMethod_obj_member_type_val0_val1(QObject* obj, const char* member, const Qt::ConnectionType* type, const QGenericArgument* val0, const QGenericArgument* val1);
QT_CORE_C_EXPORT bool qt_core_c_QMetaObject_invokeMethod_obj_member_type_val0_val1_val2(QObject* obj, const char* member, const Qt::ConnectionType* type, const QGenericArgument* val0, const QGenericArgument* val1, const QGenericArgument* val2);
QT_CORE_C_EXPORT bool qt_core_c_QMetaObject_invokeMethod_obj_member_type_val0_val1_val2_val3(QObject* obj, const char* member, const Qt::ConnectionType* type, const QGenericArgument* val0, const QGenericArgument* val1, const QGenericArgument* val2, const QGenericArgument* val3);
QT_CORE_C_EXPORT bool qt_core_c_QMetaObject_invokeMethod_obj_member_type_val0_val1_val2_val3_val4(QObject* obj, const char* member, const Qt::ConnectionType* type, const QGenericArgument* val0, const QGenericArgument* val1, const QGenericArgument* val2, const QGenericArgument* val3, const QGenericArgument* val4);
QT_CORE_C_EXPORT bool qt_core_c_QMetaObject_invokeMethod_obj_member_type_val0_val1_val2_val3_val4_val5(QObject* obj, const char* member, const Qt::ConnectionType* type, const QGenericArgument* val0, const QGenericArgument* val1, const QGenericArgument* val2, const QGenericArgument* val3, const QGenericArgument* val4, const QGenericArgument* val5);
QT_CORE_C_EXPORT bool qt_core_c_QMetaObject_invokeMethod_obj_member_type_val0_val1_val2_val3_val4_val5_val6(QObject* obj, const char* member, const Qt::ConnectionType* type, const QGenericArgument* val0, const QGenericArgument* val1, const QGenericArgument* val2, const QGenericArgument* val3, const QGenericArgument* val4, const QGenericArgument* val5, const QGenericArgument* val6);
QT_CORE_C_EXPORT bool qt_core_c_QMetaObject_invokeMethod_obj_member_type_val0_val1_val2_val3_val4_val5_val6_val7(QObject* obj, const char* member, const Qt::ConnectionType* type, const QGenericArgument* val0, const QGenericArgument* val1, const QGenericArgument* val2, const QGenericArgument* val3, const QGenericArgument* val4, const QGenericArgument* val5, const QGenericArgument* val6, const QGenericArgument* val7);
QT_CORE_C_EXPORT bool qt_core_c_QMetaObject_invokeMethod_obj_member_type_val0_val1_val2_val3_val4_val5_val6_val7_val8(QObject* obj, const char* member, const Qt::ConnectionType* type, const QGenericArgument* val0, const QGenericArgument* val1, const QGenericArgument* val2, const QGenericArgument* val3, const QGenericArgument* val4, const QGenericArgument* val5, const QGenericArgument* val6, const QGenericArgument* val7, const QGenericArgument* val8);
QT_CORE_C_EXPORT bool qt_core_c_QMetaObject_invokeMethod_obj_member_type_val0_val1_val2_val3_val4_val5_val6_val7_val8_val9(QObject* obj, const char* member, const Qt::ConnectionType* type, const QGenericArgument* val0, const QGenericArgument* val1, const QGenericArgument* val2, const QGenericArgument* val3, const QGenericArgument* val4, const QGenericArgument* val5, const QGenericArgument* val6, const QGenericArgument* val7, const QGenericArgument* val8, const QGenericArgument* val9);
QT_CORE_C_EXPORT bool qt_core_c_QMetaObject_invokeMethod_obj_member_val0(QObject* obj, const char* member, const QGenericArgument* val0);
QT_CORE_C_EXPORT bool qt_core_c_QMetaObject_invokeMethod_obj_member_val0_val1(QObject* obj, const char* member, const QGenericArgument* val0, const QGenericArgument* val1);
QT_CORE_C_EXPORT bool qt_core_c_QMetaObject_invokeMethod_obj_member_val0_val1_val2(QObject* obj, const char* member, const QGenericArgument* val0, const QGenericArgument* val1, const QGenericArgument* val2);
QT_CORE_C_EXPORT bool qt_core_c_QMetaObject_invokeMethod_obj_member_val0_val1_val2_val3(QObject* obj, const char* member, const QGenericArgument* val0, const QGenericArgument* val1, const QGenericArgument* val2, const QGenericArgument* val3);
QT_CORE_C_EXPORT bool qt_core_c_QMetaObject_invokeMethod_obj_member_val0_val1_val2_val3_val4(QObject* obj, const char* member, const QGenericArgument* val0, const QGenericArgument* val1, const QGenericArgument* val2, const QGenericArgument* val3, const QGenericArgument* val4);
QT_CORE_C_EXPORT bool qt_core_c_QMetaObject_invokeMethod_obj_member_val0_val1_val2_val3_val4_val5(QObject* obj, const char* member, const QGenericArgument* val0, const QGenericArgument* val1, const QGenericArgument* val2, const QGenericArgument* val3, const QGenericArgument* val4, const QGenericArgument* val5);
QT_CORE_C_EXPORT bool qt_core_c_QMetaObject_invokeMethod_obj_member_val0_val1_val2_val3_val4_val5_val6(QObject* obj, const char* member, const QGenericArgument* val0, const QGenericArgument* val1, const QGenericArgument* val2, const QGenericArgument* val3, const QGenericArgument* val4, const QGenericArgument* val5, const QGenericArgument* val6);
QT_CORE_C_EXPORT bool qt_core_c_QMetaObject_invokeMethod_obj_member_val0_val1_val2_val3_val4_val5_val6_val7(QObject* obj, const char* member, const QGenericArgument* val0, const QGenericArgument* val1, const QGenericArgument* val2, const QGenericArgument* val3, const QGenericArgument* val4, const QGenericArgument* val5, const QGenericArgument* val6, const QGenericArgument* val7);
QT_CORE_C_EXPORT bool qt_core_c_QMetaObject_invokeMethod_obj_member_val0_val1_val2_val3_val4_val5_val6_val7_val8(QObject* obj, const char* member, const QGenericArgument* val0, const QGenericArgument* val1, const QGenericArgument* val2, const QGenericArgument* val3, const QGenericArgument* val4, const QGenericArgument* val5, const QGenericArgument* val6, const QGenericArgument* val7, const QGenericArgument* val8);
QT_CORE_C_EXPORT bool qt_core_c_QMetaObject_invokeMethod_obj_member_val0_val1_val2_val3_val4_val5_val6_val7_val8_val9(QObject* obj, const char* member, const QGenericArgument* val0, const QGenericArgument* val1, const QGenericArgument* val2, const QGenericArgument* val3, const QGenericArgument* val4, const QGenericArgument* val5, const QGenericArgument* val6, const QGenericArgument* val7, const QGenericArgument* val8, const QGenericArgument* val9);
QT_CORE_C_EXPORT int qt_core_c_QMetaObject_metacall(QObject* arg1, QMetaObject::Call arg2, int arg3, void** arg4);
QT_CORE_C_EXPORT int qt_core_c_QMetaObject_methodCount(const QMetaObject* this_ptr);
QT_CORE_C_EXPORT int qt_core_c_QMetaObject_methodOffset(const QMetaObject* this_ptr);
QT_CORE_C_EXPORT void qt_core_c_QMetaObject_method_to_output(const QMetaObject* this_ptr, int index, QMetaMethod* output);
QT_CORE_C_EXPORT QObject* qt_core_c_QMetaObject_newInstance_no_args(const QMetaObject* this_ptr);
QT_CORE_C_EXPORT QObject* qt_core_c_QMetaObject_newInstance_val0(const QMetaObject* this_ptr, const QGenericArgument* val0);
QT_CORE_C_EXPORT QObject* qt_core_c_QMetaObject_newInstance_val0_val1(const QMetaObject* this_ptr, const QGenericArgument* val0, const QGenericArgument* val1);
QT_CORE_C_EXPORT QObject* qt_core_c_QMetaObject_newInstance_val0_val1_val2(const QMetaObject* this_ptr, const QGenericArgument* val0, const QGenericArgument* val1, const QGenericArgument* val2);
QT_CORE_C_EXPORT QObject* qt_core_c_QMetaObject_newInstance_val0_val1_val2_val3(const QMetaObject* this_ptr, const QGenericArgument* val0, const QGenericArgument* val1, const QGenericArgument* val2, const QGenericArgument* val3);
QT_CORE_C_EXPORT QObject* qt_core_c_QMetaObject_newInstance_val0_val1_val2_val3_val4(const QMetaObject* this_ptr, const QGenericArgument* val0, const QGenericArgument* val1, const QGenericArgument* val2, const QGenericArgument* val3, const QGenericArgument* val4);
QT_CORE_C_EXPORT QObject* qt_core_c_QMetaObject_newInstance_val0_val1_val2_val3_val4_val5(const QMetaObject* this_ptr, const QGenericArgument* val0, const QGenericArgument* val1, const QGenericArgument* val2, const QGenericArgument* val3, const QGenericArgument* val4, const QGenericArgument* val5);
QT_CORE_C_EXPORT QObject* qt_core_c_QMetaObject_newInstance_val0_val1_val2_val3_val4_val5_val6(const QMetaObject* this_ptr, const QGenericArgument* val0, const QGenericArgument* val1, const QGenericArgument* val2, const QGenericArgument* val3, const QGenericArgument* val4, const QGenericArgument* val5, const QGenericArgument* val6);
QT_CORE_C_EXPORT QObject* qt_core_c_QMetaObject_newInstance_val0_val1_val2_val3_val4_val5_val6_val7(const QMetaObject* this_ptr, const QGenericArgument* val0, const QGenericArgument* val1, const QGenericArgument* val2, const QGenericArgument* val3, const QGenericArgument* val4, const QGenericArgument* val5, const QGenericArgument* val6, const QGenericArgument* val7);
QT_CORE_C_EXPORT QObject* qt_core_c_QMetaObject_newInstance_val0_val1_val2_val3_val4_val5_val6_val7_val8(const QMetaObject* this_ptr, const QGenericArgument* val0, const QGenericArgument* val1, const QGenericArgument* val2, const QGenericArgument* val3, const QGenericArgument* val4, const QGenericArgument* val5, const QGenericArgument* val6, const QGenericArgument* val7, const QGenericArgument* val8);
QT_CORE_C_EXPORT QObject* qt_core_c_QMetaObject_newInstance_val0_val1_val2_val3_val4_val5_val6_val7_val8_val9(const QMetaObject* this_ptr, const QGenericArgument* val0, const QGenericArgument* val1, const QGenericArgument* val2, const QGenericArgument* val3, const QGenericArgument* val4, const QGenericArgument* val5, const QGenericArgument* val6, const QGenericArgument* val7, const QGenericArgument* val8, const QGenericArgument* val9);
QT_CORE_C_EXPORT void qt_core_c_QMetaObject_normalizedSignature_to_output(const char* method, QByteArray* output);
QT_CORE_C_EXPORT void qt_core_c_QMetaObject_normalizedType_to_output(const char* type, QByteArray* output);
QT_CORE_C_EXPORT int qt_core_c_QMetaObject_propertyCount(const QMetaObject* this_ptr);
QT_CORE_C_EXPORT int qt_core_c_QMetaObject_propertyOffset(const QMetaObject* this_ptr);
QT_CORE_C_EXPORT void qt_core_c_QMetaObject_property_to_output(const QMetaObject* this_ptr, int index, QMetaProperty* output);
QT_CORE_C_EXPORT int qt_core_c_QMetaObject_static_metacall(const QMetaObject* this_ptr, QMetaObject::Call arg1, int arg2, void** arg3);
QT_CORE_C_EXPORT const QMetaObject* qt_core_c_QMetaObject_superClass(const QMetaObject* this_ptr);
QT_CORE_C_EXPORT void qt_core_c_QMetaObject_tr_to_output_s_c(const QMetaObject* this_ptr, const char* s, const char* c, QString* output);
QT_CORE_C_EXPORT void qt_core_c_QMetaObject_tr_to_output_s_c_n(const QMetaObject* this_ptr, const char* s, const char* c, int n, QString* output);
QT_CORE_C_EXPORT void qt_core_c_QMetaObject_userProperty_to_output(const QMetaObject* this_ptr, QMetaProperty* output);

} // extern "C"

#endif // QT_CORE_C_QMETAOBJECT_H
