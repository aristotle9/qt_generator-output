#include "qt_core_c_QMessageAuthenticationCode.h"

void qt_core_c_QMessageAuthenticationCode_addData_data(QMessageAuthenticationCode* this_ptr, const QByteArray* data) {
  this_ptr->addData(*data);
}

void qt_core_c_QMessageAuthenticationCode_addData_data_length(QMessageAuthenticationCode* this_ptr, const char* data, int length) {
  this_ptr->addData(data, length);
}

bool qt_core_c_QMessageAuthenticationCode_addData_device(QMessageAuthenticationCode* this_ptr, QIODevice* device) {
  return this_ptr->addData(device);
}

void qt_core_c_QMessageAuthenticationCode_constructor_method(const QCryptographicHash::Algorithm* method, QMessageAuthenticationCode* output) {
  new(output) QMessageAuthenticationCode(*method);
}

void qt_core_c_QMessageAuthenticationCode_constructor_method_key(const QCryptographicHash::Algorithm* method, const QByteArray* key, QMessageAuthenticationCode* output) {
  new(output) QMessageAuthenticationCode(*method, *key);
}

void qt_core_c_QMessageAuthenticationCode_destructor(QMessageAuthenticationCode* this_ptr) {
  qt_core_c_call_destructor(this_ptr);
}

void qt_core_c_QMessageAuthenticationCode_hash_to_output(const QByteArray* message, const QByteArray* key, const QCryptographicHash::Algorithm* method, QByteArray* output) {
  new(output) QByteArray(QMessageAuthenticationCode::hash(*message, *key, *method));
}

void qt_core_c_QMessageAuthenticationCode_reset(QMessageAuthenticationCode* this_ptr) {
  this_ptr->reset();
}

void qt_core_c_QMessageAuthenticationCode_result_to_output(const QMessageAuthenticationCode* this_ptr, QByteArray* output) {
  new(output) QByteArray(this_ptr->result());
}

void qt_core_c_QMessageAuthenticationCode_setKey(QMessageAuthenticationCode* this_ptr, const QByteArray* key) {
  this_ptr->setKey(*key);
}

