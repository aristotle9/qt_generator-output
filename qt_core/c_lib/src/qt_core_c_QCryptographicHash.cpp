#include "qt_core_c_QCryptographicHash.h"

void qt_core_c_QCryptographicHash_addData_data(QCryptographicHash* this_ptr, const QByteArray* data) {
  this_ptr->addData(*data);
}

void qt_core_c_QCryptographicHash_addData_data_length(QCryptographicHash* this_ptr, const char* data, int length) {
  this_ptr->addData(data, length);
}

bool qt_core_c_QCryptographicHash_addData_device(QCryptographicHash* this_ptr, QIODevice* device) {
  return this_ptr->addData(device);
}

void qt_core_c_QCryptographicHash_constructor(QCryptographicHash::Algorithm method, QCryptographicHash* output) {
  new(output) QCryptographicHash(method);
}

void qt_core_c_QCryptographicHash_destructor(QCryptographicHash* this_ptr) {
  qt_core_c_call_destructor(this_ptr);
}

void qt_core_c_QCryptographicHash_hash_to_output(const QByteArray* data, QCryptographicHash::Algorithm method, QByteArray* output) {
  new(output) QByteArray(QCryptographicHash::hash(*data, method));
}

void qt_core_c_QCryptographicHash_reset(QCryptographicHash* this_ptr) {
  this_ptr->reset();
}

void qt_core_c_QCryptographicHash_result_to_output(const QCryptographicHash* this_ptr, QByteArray* output) {
  new(output) QByteArray(this_ptr->result());
}

