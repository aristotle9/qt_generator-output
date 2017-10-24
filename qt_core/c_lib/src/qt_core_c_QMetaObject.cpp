#include "qt_core_c_QMetaObject.h"

void qt_core_c_QMetaObject_Connection_constructor_no_args(QMetaObject::Connection* output) {
  new(output) QMetaObject::Connection();
}

void qt_core_c_QMetaObject_Connection_constructor_other(const QMetaObject::Connection* other, QMetaObject::Connection* output) {
  new(output) QMetaObject::Connection(*other);
}

void qt_core_c_QMetaObject_Connection_destructor(QMetaObject::Connection* this_ptr) {
  qt_core_c_call_destructor(this_ptr);
}

QMetaObject::Connection* qt_core_c_QMetaObject_Connection_operator_assign(QMetaObject::Connection* this_ptr, const QMetaObject::Connection* other) {
  return &this_ptr->operator=(*other);
}

void qt_core_c_QMetaObject_activate_sender_arg2_local_signal_index_argv(QObject* sender, const QMetaObject* arg2, int local_signal_index, void** argv) {
  QMetaObject::activate(sender, arg2, local_signal_index, argv);
}

void qt_core_c_QMetaObject_activate_sender_signal_index_argv(QObject* sender, int signal_index, void** argv) {
  QMetaObject::activate(sender, signal_index, argv);
}

void qt_core_c_QMetaObject_activate_sender_signal_offset_local_signal_index_argv(QObject* sender, int signal_offset, int local_signal_index, void** argv) {
  QMetaObject::activate(sender, signal_offset, local_signal_index, argv);
}

QObject* qt_core_c_QMetaObject_cast_QObject_ptr(const QMetaObject* this_ptr, QObject* obj) {
  return this_ptr->cast(obj);
}

const QObject* qt_core_c_QMetaObject_cast_const_QObject_ptr(const QMetaObject* this_ptr, const QObject* obj) {
  return this_ptr->cast(obj);
}

bool qt_core_c_QMetaObject_checkConnectArgs_QMetaMethod_QMetaMethod(const QMetaMethod* signal, const QMetaMethod* method) {
  return QMetaObject::checkConnectArgs(*signal, *method);
}

bool qt_core_c_QMetaObject_checkConnectArgs_char_char(const char* signal, const char* method) {
  return QMetaObject::checkConnectArgs(signal, method);
}

int qt_core_c_QMetaObject_classInfoCount(const QMetaObject* this_ptr) {
  return this_ptr->classInfoCount();
}

int qt_core_c_QMetaObject_classInfoOffset(const QMetaObject* this_ptr) {
  return this_ptr->classInfoOffset();
}

void qt_core_c_QMetaObject_classInfo_to_output(const QMetaObject* this_ptr, int index, QMetaClassInfo* output) {
  new(output) QMetaClassInfo(this_ptr->classInfo(index));
}

const char* qt_core_c_QMetaObject_className(const QMetaObject* this_ptr) {
  return this_ptr->className();
}

void qt_core_c_QMetaObject_connectSlotsByName(QObject* o) {
  QMetaObject::connectSlotsByName(o);
}

void qt_core_c_QMetaObject_connect_to_output_sender_signal_index_receiver_method_index(const QObject* sender, int signal_index, const QObject* receiver, int method_index, QMetaObject::Connection* output) {
  new(output) QMetaObject::Connection(QMetaObject::connect(sender, signal_index, receiver, method_index));
}

void qt_core_c_QMetaObject_connect_to_output_sender_signal_index_receiver_method_index_type(const QObject* sender, int signal_index, const QObject* receiver, int method_index, int type, QMetaObject::Connection* output) {
  new(output) QMetaObject::Connection(QMetaObject::connect(sender, signal_index, receiver, method_index, type));
}

void qt_core_c_QMetaObject_connect_to_output_sender_signal_index_receiver_method_index_type_types(const QObject* sender, int signal_index, const QObject* receiver, int method_index, int type, int* types, QMetaObject::Connection* output) {
  new(output) QMetaObject::Connection(QMetaObject::connect(sender, signal_index, receiver, method_index, type, types));
}

int qt_core_c_QMetaObject_constructorCount(const QMetaObject* this_ptr) {
  return this_ptr->constructorCount();
}

void qt_core_c_QMetaObject_constructor_to_output(const QMetaObject* this_ptr, int index, QMetaMethod* output) {
  new(output) QMetaMethod(this_ptr->constructor(index));
}

void qt_core_c_QMetaObject_delete(QMetaObject* this_ptr) {
  delete this_ptr;
}

bool qt_core_c_QMetaObject_disconnect(const QObject* sender, int signal_index, const QObject* receiver, int method_index) {
  return QMetaObject::disconnect(sender, signal_index, receiver, method_index);
}

bool qt_core_c_QMetaObject_disconnectOne(const QObject* sender, int signal_index, const QObject* receiver, int method_index) {
  return QMetaObject::disconnectOne(sender, signal_index, receiver, method_index);
}

int qt_core_c_QMetaObject_enumeratorCount(const QMetaObject* this_ptr) {
  return this_ptr->enumeratorCount();
}

int qt_core_c_QMetaObject_enumeratorOffset(const QMetaObject* this_ptr) {
  return this_ptr->enumeratorOffset();
}

void qt_core_c_QMetaObject_enumerator_to_output(const QMetaObject* this_ptr, int index, QMetaEnum* output) {
  new(output) QMetaEnum(this_ptr->enumerator(index));
}

int qt_core_c_QMetaObject_indexOfClassInfo(const QMetaObject* this_ptr, const char* name) {
  return this_ptr->indexOfClassInfo(name);
}

int qt_core_c_QMetaObject_indexOfConstructor(const QMetaObject* this_ptr, const char* constructor) {
  return this_ptr->indexOfConstructor(constructor);
}

int qt_core_c_QMetaObject_indexOfEnumerator(const QMetaObject* this_ptr, const char* name) {
  return this_ptr->indexOfEnumerator(name);
}

int qt_core_c_QMetaObject_indexOfMethod(const QMetaObject* this_ptr, const char* method) {
  return this_ptr->indexOfMethod(method);
}

int qt_core_c_QMetaObject_indexOfProperty(const QMetaObject* this_ptr, const char* name) {
  return this_ptr->indexOfProperty(name);
}

int qt_core_c_QMetaObject_indexOfSignal(const QMetaObject* this_ptr, const char* signal) {
  return this_ptr->indexOfSignal(signal);
}

int qt_core_c_QMetaObject_indexOfSlot(const QMetaObject* this_ptr, const char* slot) {
  return this_ptr->indexOfSlot(slot);
}

bool qt_core_c_QMetaObject_inherits(const QMetaObject* this_ptr, const QMetaObject* metaObject) {
  return this_ptr->inherits(metaObject);
}

bool qt_core_c_QMetaObject_invokeMethod_obj_member(QObject* obj, const char* member) {
  return QMetaObject::invokeMethod(obj, member);
}

bool qt_core_c_QMetaObject_invokeMethod_obj_member_arg3_ret(QObject* obj, const char* member, const Qt::ConnectionType* arg3, const QGenericReturnArgument* ret) {
  return QMetaObject::invokeMethod(obj, member, *arg3, *ret);
}

bool qt_core_c_QMetaObject_invokeMethod_obj_member_arg3_ret_val0(QObject* obj, const char* member, const Qt::ConnectionType* arg3, const QGenericReturnArgument* ret, const QGenericArgument* val0) {
  return QMetaObject::invokeMethod(obj, member, *arg3, *ret, *val0);
}

bool qt_core_c_QMetaObject_invokeMethod_obj_member_arg3_ret_val0_val1(QObject* obj, const char* member, const Qt::ConnectionType* arg3, const QGenericReturnArgument* ret, const QGenericArgument* val0, const QGenericArgument* val1) {
  return QMetaObject::invokeMethod(obj, member, *arg3, *ret, *val0, *val1);
}

bool qt_core_c_QMetaObject_invokeMethod_obj_member_arg3_ret_val0_val1_val2(QObject* obj, const char* member, const Qt::ConnectionType* arg3, const QGenericReturnArgument* ret, const QGenericArgument* val0, const QGenericArgument* val1, const QGenericArgument* val2) {
  return QMetaObject::invokeMethod(obj, member, *arg3, *ret, *val0, *val1, *val2);
}

bool qt_core_c_QMetaObject_invokeMethod_obj_member_arg3_ret_val0_val1_val2_val3(QObject* obj, const char* member, const Qt::ConnectionType* arg3, const QGenericReturnArgument* ret, const QGenericArgument* val0, const QGenericArgument* val1, const QGenericArgument* val2, const QGenericArgument* val3) {
  return QMetaObject::invokeMethod(obj, member, *arg3, *ret, *val0, *val1, *val2, *val3);
}

bool qt_core_c_QMetaObject_invokeMethod_obj_member_arg3_ret_val0_val1_val2_val3_val4(QObject* obj, const char* member, const Qt::ConnectionType* arg3, const QGenericReturnArgument* ret, const QGenericArgument* val0, const QGenericArgument* val1, const QGenericArgument* val2, const QGenericArgument* val3, const QGenericArgument* val4) {
  return QMetaObject::invokeMethod(obj, member, *arg3, *ret, *val0, *val1, *val2, *val3, *val4);
}

bool qt_core_c_QMetaObject_invokeMethod_obj_member_arg3_ret_val0_val1_val2_val3_val4_val5(QObject* obj, const char* member, const Qt::ConnectionType* arg3, const QGenericReturnArgument* ret, const QGenericArgument* val0, const QGenericArgument* val1, const QGenericArgument* val2, const QGenericArgument* val3, const QGenericArgument* val4, const QGenericArgument* val5) {
  return QMetaObject::invokeMethod(obj, member, *arg3, *ret, *val0, *val1, *val2, *val3, *val4, *val5);
}

bool qt_core_c_QMetaObject_invokeMethod_obj_member_arg3_ret_val0_val1_val2_val3_val4_val5_val6(QObject* obj, const char* member, const Qt::ConnectionType* arg3, const QGenericReturnArgument* ret, const QGenericArgument* val0, const QGenericArgument* val1, const QGenericArgument* val2, const QGenericArgument* val3, const QGenericArgument* val4, const QGenericArgument* val5, const QGenericArgument* val6) {
  return QMetaObject::invokeMethod(obj, member, *arg3, *ret, *val0, *val1, *val2, *val3, *val4, *val5, *val6);
}

bool qt_core_c_QMetaObject_invokeMethod_obj_member_arg3_ret_val0_val1_val2_val3_val4_val5_val6_val7(QObject* obj, const char* member, const Qt::ConnectionType* arg3, const QGenericReturnArgument* ret, const QGenericArgument* val0, const QGenericArgument* val1, const QGenericArgument* val2, const QGenericArgument* val3, const QGenericArgument* val4, const QGenericArgument* val5, const QGenericArgument* val6, const QGenericArgument* val7) {
  return QMetaObject::invokeMethod(obj, member, *arg3, *ret, *val0, *val1, *val2, *val3, *val4, *val5, *val6, *val7);
}

bool qt_core_c_QMetaObject_invokeMethod_obj_member_arg3_ret_val0_val1_val2_val3_val4_val5_val6_val7_val8(QObject* obj, const char* member, const Qt::ConnectionType* arg3, const QGenericReturnArgument* ret, const QGenericArgument* val0, const QGenericArgument* val1, const QGenericArgument* val2, const QGenericArgument* val3, const QGenericArgument* val4, const QGenericArgument* val5, const QGenericArgument* val6, const QGenericArgument* val7, const QGenericArgument* val8) {
  return QMetaObject::invokeMethod(obj, member, *arg3, *ret, *val0, *val1, *val2, *val3, *val4, *val5, *val6, *val7, *val8);
}

bool qt_core_c_QMetaObject_invokeMethod_obj_member_arg3_ret_val0_val1_val2_val3_val4_val5_val6_val7_val8_val9(QObject* obj, const char* member, const Qt::ConnectionType* arg3, const QGenericReturnArgument* ret, const QGenericArgument* val0, const QGenericArgument* val1, const QGenericArgument* val2, const QGenericArgument* val3, const QGenericArgument* val4, const QGenericArgument* val5, const QGenericArgument* val6, const QGenericArgument* val7, const QGenericArgument* val8, const QGenericArgument* val9) {
  return QMetaObject::invokeMethod(obj, member, *arg3, *ret, *val0, *val1, *val2, *val3, *val4, *val5, *val6, *val7, *val8, *val9);
}

bool qt_core_c_QMetaObject_invokeMethod_obj_member_ret(QObject* obj, const char* member, const QGenericReturnArgument* ret) {
  return QMetaObject::invokeMethod(obj, member, *ret);
}

bool qt_core_c_QMetaObject_invokeMethod_obj_member_ret_val0(QObject* obj, const char* member, const QGenericReturnArgument* ret, const QGenericArgument* val0) {
  return QMetaObject::invokeMethod(obj, member, *ret, *val0);
}

bool qt_core_c_QMetaObject_invokeMethod_obj_member_ret_val0_val1(QObject* obj, const char* member, const QGenericReturnArgument* ret, const QGenericArgument* val0, const QGenericArgument* val1) {
  return QMetaObject::invokeMethod(obj, member, *ret, *val0, *val1);
}

bool qt_core_c_QMetaObject_invokeMethod_obj_member_ret_val0_val1_val2(QObject* obj, const char* member, const QGenericReturnArgument* ret, const QGenericArgument* val0, const QGenericArgument* val1, const QGenericArgument* val2) {
  return QMetaObject::invokeMethod(obj, member, *ret, *val0, *val1, *val2);
}

bool qt_core_c_QMetaObject_invokeMethod_obj_member_ret_val0_val1_val2_val3(QObject* obj, const char* member, const QGenericReturnArgument* ret, const QGenericArgument* val0, const QGenericArgument* val1, const QGenericArgument* val2, const QGenericArgument* val3) {
  return QMetaObject::invokeMethod(obj, member, *ret, *val0, *val1, *val2, *val3);
}

bool qt_core_c_QMetaObject_invokeMethod_obj_member_ret_val0_val1_val2_val3_val4(QObject* obj, const char* member, const QGenericReturnArgument* ret, const QGenericArgument* val0, const QGenericArgument* val1, const QGenericArgument* val2, const QGenericArgument* val3, const QGenericArgument* val4) {
  return QMetaObject::invokeMethod(obj, member, *ret, *val0, *val1, *val2, *val3, *val4);
}

bool qt_core_c_QMetaObject_invokeMethod_obj_member_ret_val0_val1_val2_val3_val4_val5(QObject* obj, const char* member, const QGenericReturnArgument* ret, const QGenericArgument* val0, const QGenericArgument* val1, const QGenericArgument* val2, const QGenericArgument* val3, const QGenericArgument* val4, const QGenericArgument* val5) {
  return QMetaObject::invokeMethod(obj, member, *ret, *val0, *val1, *val2, *val3, *val4, *val5);
}

bool qt_core_c_QMetaObject_invokeMethod_obj_member_ret_val0_val1_val2_val3_val4_val5_val6(QObject* obj, const char* member, const QGenericReturnArgument* ret, const QGenericArgument* val0, const QGenericArgument* val1, const QGenericArgument* val2, const QGenericArgument* val3, const QGenericArgument* val4, const QGenericArgument* val5, const QGenericArgument* val6) {
  return QMetaObject::invokeMethod(obj, member, *ret, *val0, *val1, *val2, *val3, *val4, *val5, *val6);
}

bool qt_core_c_QMetaObject_invokeMethod_obj_member_ret_val0_val1_val2_val3_val4_val5_val6_val7(QObject* obj, const char* member, const QGenericReturnArgument* ret, const QGenericArgument* val0, const QGenericArgument* val1, const QGenericArgument* val2, const QGenericArgument* val3, const QGenericArgument* val4, const QGenericArgument* val5, const QGenericArgument* val6, const QGenericArgument* val7) {
  return QMetaObject::invokeMethod(obj, member, *ret, *val0, *val1, *val2, *val3, *val4, *val5, *val6, *val7);
}

bool qt_core_c_QMetaObject_invokeMethod_obj_member_ret_val0_val1_val2_val3_val4_val5_val6_val7_val8(QObject* obj, const char* member, const QGenericReturnArgument* ret, const QGenericArgument* val0, const QGenericArgument* val1, const QGenericArgument* val2, const QGenericArgument* val3, const QGenericArgument* val4, const QGenericArgument* val5, const QGenericArgument* val6, const QGenericArgument* val7, const QGenericArgument* val8) {
  return QMetaObject::invokeMethod(obj, member, *ret, *val0, *val1, *val2, *val3, *val4, *val5, *val6, *val7, *val8);
}

bool qt_core_c_QMetaObject_invokeMethod_obj_member_ret_val0_val1_val2_val3_val4_val5_val6_val7_val8_val9(QObject* obj, const char* member, const QGenericReturnArgument* ret, const QGenericArgument* val0, const QGenericArgument* val1, const QGenericArgument* val2, const QGenericArgument* val3, const QGenericArgument* val4, const QGenericArgument* val5, const QGenericArgument* val6, const QGenericArgument* val7, const QGenericArgument* val8, const QGenericArgument* val9) {
  return QMetaObject::invokeMethod(obj, member, *ret, *val0, *val1, *val2, *val3, *val4, *val5, *val6, *val7, *val8, *val9);
}

bool qt_core_c_QMetaObject_invokeMethod_obj_member_type(QObject* obj, const char* member, const Qt::ConnectionType* type) {
  return QMetaObject::invokeMethod(obj, member, *type);
}

bool qt_core_c_QMetaObject_invokeMethod_obj_member_type_val0(QObject* obj, const char* member, const Qt::ConnectionType* type, const QGenericArgument* val0) {
  return QMetaObject::invokeMethod(obj, member, *type, *val0);
}

bool qt_core_c_QMetaObject_invokeMethod_obj_member_type_val0_val1(QObject* obj, const char* member, const Qt::ConnectionType* type, const QGenericArgument* val0, const QGenericArgument* val1) {
  return QMetaObject::invokeMethod(obj, member, *type, *val0, *val1);
}

bool qt_core_c_QMetaObject_invokeMethod_obj_member_type_val0_val1_val2(QObject* obj, const char* member, const Qt::ConnectionType* type, const QGenericArgument* val0, const QGenericArgument* val1, const QGenericArgument* val2) {
  return QMetaObject::invokeMethod(obj, member, *type, *val0, *val1, *val2);
}

bool qt_core_c_QMetaObject_invokeMethod_obj_member_type_val0_val1_val2_val3(QObject* obj, const char* member, const Qt::ConnectionType* type, const QGenericArgument* val0, const QGenericArgument* val1, const QGenericArgument* val2, const QGenericArgument* val3) {
  return QMetaObject::invokeMethod(obj, member, *type, *val0, *val1, *val2, *val3);
}

bool qt_core_c_QMetaObject_invokeMethod_obj_member_type_val0_val1_val2_val3_val4(QObject* obj, const char* member, const Qt::ConnectionType* type, const QGenericArgument* val0, const QGenericArgument* val1, const QGenericArgument* val2, const QGenericArgument* val3, const QGenericArgument* val4) {
  return QMetaObject::invokeMethod(obj, member, *type, *val0, *val1, *val2, *val3, *val4);
}

bool qt_core_c_QMetaObject_invokeMethod_obj_member_type_val0_val1_val2_val3_val4_val5(QObject* obj, const char* member, const Qt::ConnectionType* type, const QGenericArgument* val0, const QGenericArgument* val1, const QGenericArgument* val2, const QGenericArgument* val3, const QGenericArgument* val4, const QGenericArgument* val5) {
  return QMetaObject::invokeMethod(obj, member, *type, *val0, *val1, *val2, *val3, *val4, *val5);
}

bool qt_core_c_QMetaObject_invokeMethod_obj_member_type_val0_val1_val2_val3_val4_val5_val6(QObject* obj, const char* member, const Qt::ConnectionType* type, const QGenericArgument* val0, const QGenericArgument* val1, const QGenericArgument* val2, const QGenericArgument* val3, const QGenericArgument* val4, const QGenericArgument* val5, const QGenericArgument* val6) {
  return QMetaObject::invokeMethod(obj, member, *type, *val0, *val1, *val2, *val3, *val4, *val5, *val6);
}

bool qt_core_c_QMetaObject_invokeMethod_obj_member_type_val0_val1_val2_val3_val4_val5_val6_val7(QObject* obj, const char* member, const Qt::ConnectionType* type, const QGenericArgument* val0, const QGenericArgument* val1, const QGenericArgument* val2, const QGenericArgument* val3, const QGenericArgument* val4, const QGenericArgument* val5, const QGenericArgument* val6, const QGenericArgument* val7) {
  return QMetaObject::invokeMethod(obj, member, *type, *val0, *val1, *val2, *val3, *val4, *val5, *val6, *val7);
}

bool qt_core_c_QMetaObject_invokeMethod_obj_member_type_val0_val1_val2_val3_val4_val5_val6_val7_val8(QObject* obj, const char* member, const Qt::ConnectionType* type, const QGenericArgument* val0, const QGenericArgument* val1, const QGenericArgument* val2, const QGenericArgument* val3, const QGenericArgument* val4, const QGenericArgument* val5, const QGenericArgument* val6, const QGenericArgument* val7, const QGenericArgument* val8) {
  return QMetaObject::invokeMethod(obj, member, *type, *val0, *val1, *val2, *val3, *val4, *val5, *val6, *val7, *val8);
}

bool qt_core_c_QMetaObject_invokeMethod_obj_member_type_val0_val1_val2_val3_val4_val5_val6_val7_val8_val9(QObject* obj, const char* member, const Qt::ConnectionType* type, const QGenericArgument* val0, const QGenericArgument* val1, const QGenericArgument* val2, const QGenericArgument* val3, const QGenericArgument* val4, const QGenericArgument* val5, const QGenericArgument* val6, const QGenericArgument* val7, const QGenericArgument* val8, const QGenericArgument* val9) {
  return QMetaObject::invokeMethod(obj, member, *type, *val0, *val1, *val2, *val3, *val4, *val5, *val6, *val7, *val8, *val9);
}

bool qt_core_c_QMetaObject_invokeMethod_obj_member_val0(QObject* obj, const char* member, const QGenericArgument* val0) {
  return QMetaObject::invokeMethod(obj, member, *val0);
}

bool qt_core_c_QMetaObject_invokeMethod_obj_member_val0_val1(QObject* obj, const char* member, const QGenericArgument* val0, const QGenericArgument* val1) {
  return QMetaObject::invokeMethod(obj, member, *val0, *val1);
}

bool qt_core_c_QMetaObject_invokeMethod_obj_member_val0_val1_val2(QObject* obj, const char* member, const QGenericArgument* val0, const QGenericArgument* val1, const QGenericArgument* val2) {
  return QMetaObject::invokeMethod(obj, member, *val0, *val1, *val2);
}

bool qt_core_c_QMetaObject_invokeMethod_obj_member_val0_val1_val2_val3(QObject* obj, const char* member, const QGenericArgument* val0, const QGenericArgument* val1, const QGenericArgument* val2, const QGenericArgument* val3) {
  return QMetaObject::invokeMethod(obj, member, *val0, *val1, *val2, *val3);
}

bool qt_core_c_QMetaObject_invokeMethod_obj_member_val0_val1_val2_val3_val4(QObject* obj, const char* member, const QGenericArgument* val0, const QGenericArgument* val1, const QGenericArgument* val2, const QGenericArgument* val3, const QGenericArgument* val4) {
  return QMetaObject::invokeMethod(obj, member, *val0, *val1, *val2, *val3, *val4);
}

bool qt_core_c_QMetaObject_invokeMethod_obj_member_val0_val1_val2_val3_val4_val5(QObject* obj, const char* member, const QGenericArgument* val0, const QGenericArgument* val1, const QGenericArgument* val2, const QGenericArgument* val3, const QGenericArgument* val4, const QGenericArgument* val5) {
  return QMetaObject::invokeMethod(obj, member, *val0, *val1, *val2, *val3, *val4, *val5);
}

bool qt_core_c_QMetaObject_invokeMethod_obj_member_val0_val1_val2_val3_val4_val5_val6(QObject* obj, const char* member, const QGenericArgument* val0, const QGenericArgument* val1, const QGenericArgument* val2, const QGenericArgument* val3, const QGenericArgument* val4, const QGenericArgument* val5, const QGenericArgument* val6) {
  return QMetaObject::invokeMethod(obj, member, *val0, *val1, *val2, *val3, *val4, *val5, *val6);
}

bool qt_core_c_QMetaObject_invokeMethod_obj_member_val0_val1_val2_val3_val4_val5_val6_val7(QObject* obj, const char* member, const QGenericArgument* val0, const QGenericArgument* val1, const QGenericArgument* val2, const QGenericArgument* val3, const QGenericArgument* val4, const QGenericArgument* val5, const QGenericArgument* val6, const QGenericArgument* val7) {
  return QMetaObject::invokeMethod(obj, member, *val0, *val1, *val2, *val3, *val4, *val5, *val6, *val7);
}

bool qt_core_c_QMetaObject_invokeMethod_obj_member_val0_val1_val2_val3_val4_val5_val6_val7_val8(QObject* obj, const char* member, const QGenericArgument* val0, const QGenericArgument* val1, const QGenericArgument* val2, const QGenericArgument* val3, const QGenericArgument* val4, const QGenericArgument* val5, const QGenericArgument* val6, const QGenericArgument* val7, const QGenericArgument* val8) {
  return QMetaObject::invokeMethod(obj, member, *val0, *val1, *val2, *val3, *val4, *val5, *val6, *val7, *val8);
}

bool qt_core_c_QMetaObject_invokeMethod_obj_member_val0_val1_val2_val3_val4_val5_val6_val7_val8_val9(QObject* obj, const char* member, const QGenericArgument* val0, const QGenericArgument* val1, const QGenericArgument* val2, const QGenericArgument* val3, const QGenericArgument* val4, const QGenericArgument* val5, const QGenericArgument* val6, const QGenericArgument* val7, const QGenericArgument* val8, const QGenericArgument* val9) {
  return QMetaObject::invokeMethod(obj, member, *val0, *val1, *val2, *val3, *val4, *val5, *val6, *val7, *val8, *val9);
}

int qt_core_c_QMetaObject_metacall(QObject* arg1, QMetaObject::Call arg2, int arg3, void** arg4) {
  return QMetaObject::metacall(arg1, arg2, arg3, arg4);
}

int qt_core_c_QMetaObject_methodCount(const QMetaObject* this_ptr) {
  return this_ptr->methodCount();
}

int qt_core_c_QMetaObject_methodOffset(const QMetaObject* this_ptr) {
  return this_ptr->methodOffset();
}

void qt_core_c_QMetaObject_method_to_output(const QMetaObject* this_ptr, int index, QMetaMethod* output) {
  new(output) QMetaMethod(this_ptr->method(index));
}

QObject* qt_core_c_QMetaObject_newInstance_no_args(const QMetaObject* this_ptr) {
  return this_ptr->newInstance();
}

QObject* qt_core_c_QMetaObject_newInstance_val0(const QMetaObject* this_ptr, const QGenericArgument* val0) {
  return this_ptr->newInstance(*val0);
}

QObject* qt_core_c_QMetaObject_newInstance_val0_val1(const QMetaObject* this_ptr, const QGenericArgument* val0, const QGenericArgument* val1) {
  return this_ptr->newInstance(*val0, *val1);
}

QObject* qt_core_c_QMetaObject_newInstance_val0_val1_val2(const QMetaObject* this_ptr, const QGenericArgument* val0, const QGenericArgument* val1, const QGenericArgument* val2) {
  return this_ptr->newInstance(*val0, *val1, *val2);
}

QObject* qt_core_c_QMetaObject_newInstance_val0_val1_val2_val3(const QMetaObject* this_ptr, const QGenericArgument* val0, const QGenericArgument* val1, const QGenericArgument* val2, const QGenericArgument* val3) {
  return this_ptr->newInstance(*val0, *val1, *val2, *val3);
}

QObject* qt_core_c_QMetaObject_newInstance_val0_val1_val2_val3_val4(const QMetaObject* this_ptr, const QGenericArgument* val0, const QGenericArgument* val1, const QGenericArgument* val2, const QGenericArgument* val3, const QGenericArgument* val4) {
  return this_ptr->newInstance(*val0, *val1, *val2, *val3, *val4);
}

QObject* qt_core_c_QMetaObject_newInstance_val0_val1_val2_val3_val4_val5(const QMetaObject* this_ptr, const QGenericArgument* val0, const QGenericArgument* val1, const QGenericArgument* val2, const QGenericArgument* val3, const QGenericArgument* val4, const QGenericArgument* val5) {
  return this_ptr->newInstance(*val0, *val1, *val2, *val3, *val4, *val5);
}

QObject* qt_core_c_QMetaObject_newInstance_val0_val1_val2_val3_val4_val5_val6(const QMetaObject* this_ptr, const QGenericArgument* val0, const QGenericArgument* val1, const QGenericArgument* val2, const QGenericArgument* val3, const QGenericArgument* val4, const QGenericArgument* val5, const QGenericArgument* val6) {
  return this_ptr->newInstance(*val0, *val1, *val2, *val3, *val4, *val5, *val6);
}

QObject* qt_core_c_QMetaObject_newInstance_val0_val1_val2_val3_val4_val5_val6_val7(const QMetaObject* this_ptr, const QGenericArgument* val0, const QGenericArgument* val1, const QGenericArgument* val2, const QGenericArgument* val3, const QGenericArgument* val4, const QGenericArgument* val5, const QGenericArgument* val6, const QGenericArgument* val7) {
  return this_ptr->newInstance(*val0, *val1, *val2, *val3, *val4, *val5, *val6, *val7);
}

QObject* qt_core_c_QMetaObject_newInstance_val0_val1_val2_val3_val4_val5_val6_val7_val8(const QMetaObject* this_ptr, const QGenericArgument* val0, const QGenericArgument* val1, const QGenericArgument* val2, const QGenericArgument* val3, const QGenericArgument* val4, const QGenericArgument* val5, const QGenericArgument* val6, const QGenericArgument* val7, const QGenericArgument* val8) {
  return this_ptr->newInstance(*val0, *val1, *val2, *val3, *val4, *val5, *val6, *val7, *val8);
}

QObject* qt_core_c_QMetaObject_newInstance_val0_val1_val2_val3_val4_val5_val6_val7_val8_val9(const QMetaObject* this_ptr, const QGenericArgument* val0, const QGenericArgument* val1, const QGenericArgument* val2, const QGenericArgument* val3, const QGenericArgument* val4, const QGenericArgument* val5, const QGenericArgument* val6, const QGenericArgument* val7, const QGenericArgument* val8, const QGenericArgument* val9) {
  return this_ptr->newInstance(*val0, *val1, *val2, *val3, *val4, *val5, *val6, *val7, *val8, *val9);
}

void qt_core_c_QMetaObject_normalizedSignature_to_output(const char* method, QByteArray* output) {
  new(output) QByteArray(QMetaObject::normalizedSignature(method));
}

void qt_core_c_QMetaObject_normalizedType_to_output(const char* type, QByteArray* output) {
  new(output) QByteArray(QMetaObject::normalizedType(type));
}

int qt_core_c_QMetaObject_propertyCount(const QMetaObject* this_ptr) {
  return this_ptr->propertyCount();
}

int qt_core_c_QMetaObject_propertyOffset(const QMetaObject* this_ptr) {
  return this_ptr->propertyOffset();
}

void qt_core_c_QMetaObject_property_to_output(const QMetaObject* this_ptr, int index, QMetaProperty* output) {
  new(output) QMetaProperty(this_ptr->property(index));
}

int qt_core_c_QMetaObject_static_metacall(const QMetaObject* this_ptr, QMetaObject::Call arg1, int arg2, void** arg3) {
  return this_ptr->static_metacall(arg1, arg2, arg3);
}

const QMetaObject* qt_core_c_QMetaObject_superClass(const QMetaObject* this_ptr) {
  return this_ptr->superClass();
}

void qt_core_c_QMetaObject_tr_to_output_s_c(const QMetaObject* this_ptr, const char* s, const char* c, QString* output) {
  new(output) QString(this_ptr->tr(s, c));
}

void qt_core_c_QMetaObject_tr_to_output_s_c_n(const QMetaObject* this_ptr, const char* s, const char* c, int n, QString* output) {
  new(output) QString(this_ptr->tr(s, c, n));
}

void qt_core_c_QMetaObject_userProperty_to_output(const QMetaObject* this_ptr, QMetaProperty* output) {
  new(output) QMetaProperty(this_ptr->userProperty());
}

