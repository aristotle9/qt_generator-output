#include "qt_core_c_QUuid.h"

bool qt_core_c_QUuid_G_operator_ge(const QUuid* lhs, const QUuid* rhs) {
  return operator>=(*lhs, *rhs);
}

bool qt_core_c_QUuid_G_operator_le(const QUuid* lhs, const QUuid* rhs) {
  return operator<=(*lhs, *rhs);
}

QDataStream* qt_core_c_QUuid_G_operator_shl(QDataStream* arg1, const QUuid* arg2) {
  return &operator<<(*arg1, *arg2);
}

void qt_core_c_QUuid_G_operator_shl_to_output(const QDebug* arg1, const QUuid* arg2, QDebug* output) {
  new(output) QDebug(operator<<(*arg1, *arg2));
}

QDataStream* qt_core_c_QUuid_G_operator_shr(QDataStream* arg1, QUuid* arg2) {
  return &operator>>(*arg1, *arg2);
}

unsigned int qt_core_c_QUuid_G_qHash_uuid(const QUuid* uuid) {
  return qHash(*uuid);
}

unsigned int qt_core_c_QUuid_G_qHash_uuid_seed(const QUuid* uuid, unsigned int seed) {
  return qHash(*uuid, seed);
}

void qt_core_c_QUuid_constructor_QByteArray(const QByteArray* arg1, QUuid* output) {
  new(output) QUuid(*arg1);
}

void qt_core_c_QUuid_constructor_QString(const QString* arg1, QUuid* output) {
  new(output) QUuid(*arg1);
}

void qt_core_c_QUuid_constructor_char(const char* arg1, QUuid* output) {
  new(output) QUuid(arg1);
}

void qt_core_c_QUuid_constructor_no_args(QUuid* output) {
  new(output) QUuid();
}

void qt_core_c_QUuid_constructor_unsigned_int_unsigned_short_unsigned_short_unsigned_char_unsigned_char_unsigned_char_unsigned_char_unsigned_char_unsigned_char_unsigned_char_unsigned_char(unsigned int l, unsigned short w1, unsigned short w2, unsigned char b1, unsigned char b2, unsigned char b3, unsigned char b4, unsigned char b5, unsigned char b6, unsigned char b7, unsigned char b8, QUuid* output) {
  new(output) QUuid(l, w1, w2, b1, b2, b3, b4, b5, b6, b7, b8);
}

void qt_core_c_QUuid_createUuidV3_to_output_QUuid_QByteArray(const QUuid* ns, const QByteArray* baseData, QUuid* output) {
  new(output) QUuid(QUuid::createUuidV3(*ns, *baseData));
}

void qt_core_c_QUuid_createUuidV3_to_output_QUuid_QString(const QUuid* ns, const QString* baseData, QUuid* output) {
  new(output) QUuid(QUuid::createUuidV3(*ns, *baseData));
}

void qt_core_c_QUuid_createUuidV5_to_output_QUuid_QByteArray(const QUuid* ns, const QByteArray* baseData, QUuid* output) {
  new(output) QUuid(QUuid::createUuidV5(*ns, *baseData));
}

void qt_core_c_QUuid_createUuidV5_to_output_QUuid_QString(const QUuid* ns, const QString* baseData, QUuid* output) {
  new(output) QUuid(QUuid::createUuidV5(*ns, *baseData));
}

void qt_core_c_QUuid_createUuid_to_output(QUuid* output) {
  new(output) QUuid(QUuid::createUuid());
}

unsigned int qt_core_c_QUuid_data1(const QUuid* this_ptr) {
  return this_ptr->data1;
}

unsigned short qt_core_c_QUuid_data2(const QUuid* this_ptr) {
  return this_ptr->data2;
}

unsigned short qt_core_c_QUuid_data3(const QUuid* this_ptr) {
  return this_ptr->data3;
}

void qt_core_c_QUuid_destructor(QUuid* this_ptr) {
  qt_core_c_call_destructor(this_ptr);
}

void qt_core_c_QUuid_fromRfc4122_to_output(const QByteArray* arg1, QUuid* output) {
  new(output) QUuid(QUuid::fromRfc4122(*arg1));
}

bool qt_core_c_QUuid_isNull(const QUuid* this_ptr) {
  return this_ptr->isNull();
}

bool qt_core_c_QUuid_operator_eq(const QUuid* this_ptr, const QUuid* orig) {
  return this_ptr->operator==(*orig);
}

bool qt_core_c_QUuid_operator_gt(const QUuid* this_ptr, const QUuid* other) {
  return this_ptr->operator>(*other);
}

bool qt_core_c_QUuid_operator_lt(const QUuid* this_ptr, const QUuid* other) {
  return this_ptr->operator<(*other);
}

bool qt_core_c_QUuid_operator_neq(const QUuid* this_ptr, const QUuid* orig) {
  return this_ptr->operator!=(*orig);
}

void qt_core_c_QUuid_set_data1(QUuid* this_ptr, unsigned int value) {
  this_ptr->data1 = value;
}

void qt_core_c_QUuid_set_data2(QUuid* this_ptr, unsigned short value) {
  this_ptr->data2 = value;
}

void qt_core_c_QUuid_set_data3(QUuid* this_ptr, unsigned short value) {
  this_ptr->data3 = value;
}

void qt_core_c_QUuid_toByteArray_to_output(const QUuid* this_ptr, QByteArray* output) {
  new(output) QByteArray(this_ptr->toByteArray());
}

void qt_core_c_QUuid_toRfc4122_to_output(const QUuid* this_ptr, QByteArray* output) {
  new(output) QByteArray(this_ptr->toRfc4122());
}

void qt_core_c_QUuid_toString_to_output(const QUuid* this_ptr, QString* output) {
  new(output) QString(this_ptr->toString());
}

