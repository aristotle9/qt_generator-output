#include "qt_core_c_QVersionNumber.h"

bool qt_core_c_QVersionNumber_G_operator_eq(const QVersionNumber* lhs, const QVersionNumber* rhs) {
  return operator==(*lhs, *rhs);
}

bool qt_core_c_QVersionNumber_G_operator_ge(const QVersionNumber* lhs, const QVersionNumber* rhs) {
  return operator>=(*lhs, *rhs);
}

bool qt_core_c_QVersionNumber_G_operator_gt(const QVersionNumber* lhs, const QVersionNumber* rhs) {
  return operator>(*lhs, *rhs);
}

bool qt_core_c_QVersionNumber_G_operator_le(const QVersionNumber* lhs, const QVersionNumber* rhs) {
  return operator<=(*lhs, *rhs);
}

bool qt_core_c_QVersionNumber_G_operator_lt(const QVersionNumber* lhs, const QVersionNumber* rhs) {
  return operator<(*lhs, *rhs);
}

bool qt_core_c_QVersionNumber_G_operator_neq(const QVersionNumber* lhs, const QVersionNumber* rhs) {
  return operator!=(*lhs, *rhs);
}

QDataStream* qt_core_c_QVersionNumber_G_operator_shl(QDataStream* out, const QVersionNumber* version) {
  return &operator<<(*out, *version);
}

void qt_core_c_QVersionNumber_G_operator_shl_to_output(const QDebug* arg1, const QVersionNumber* version, QDebug* output) {
  new(output) QDebug(operator<<(*arg1, *version));
}

QDataStream* qt_core_c_QVersionNumber_G_operator_shr(QDataStream* in, QVersionNumber* version) {
  return &operator>>(*in, *version);
}

unsigned int qt_core_c_QVersionNumber_G_qHash_key(const QVersionNumber* key) {
  return qHash(*key);
}

unsigned int qt_core_c_QVersionNumber_G_qHash_key_seed(const QVersionNumber* key, unsigned int seed) {
  return qHash(*key, seed);
}

void qt_core_c_QVersionNumber_commonPrefix_to_output(const QVersionNumber* v1, const QVersionNumber* v2, QVersionNumber* output) {
  new(output) QVersionNumber(QVersionNumber::commonPrefix(*v1, *v2));
}

int qt_core_c_QVersionNumber_compare(const QVersionNumber* v1, const QVersionNumber* v2) {
  return QVersionNumber::compare(*v1, *v2);
}

void qt_core_c_QVersionNumber_constructor_maj(int maj, QVersionNumber* output) {
  new(output) QVersionNumber(maj);
}

void qt_core_c_QVersionNumber_constructor_maj_min(int maj, int min, QVersionNumber* output) {
  new(output) QVersionNumber(maj, min);
}

void qt_core_c_QVersionNumber_constructor_maj_min_mic(int maj, int min, int mic, QVersionNumber* output) {
  new(output) QVersionNumber(maj, min, mic);
}

void qt_core_c_QVersionNumber_constructor_no_args(QVersionNumber* output) {
  new(output) QVersionNumber();
}

void qt_core_c_QVersionNumber_constructor_seg(const QVector< int >* seg, QVersionNumber* output) {
  new(output) QVersionNumber(*seg);
}

void qt_core_c_QVersionNumber_destructor(QVersionNumber* this_ptr) {
  qt_core_c_call_destructor(this_ptr);
}

void qt_core_c_QVersionNumber_fromString_to_output_string(const QString* string, QVersionNumber* output) {
  new(output) QVersionNumber(QVersionNumber::fromString(*string));
}

void qt_core_c_QVersionNumber_fromString_to_output_string_suffixIndex(const QString* string, int* suffixIndex, QVersionNumber* output) {
  new(output) QVersionNumber(QVersionNumber::fromString(*string, suffixIndex));
}

bool qt_core_c_QVersionNumber_isNormalized(const QVersionNumber* this_ptr) {
  return this_ptr->isNormalized();
}

bool qt_core_c_QVersionNumber_isNull(const QVersionNumber* this_ptr) {
  return this_ptr->isNull();
}

bool qt_core_c_QVersionNumber_isPrefixOf(const QVersionNumber* this_ptr, const QVersionNumber* other) {
  return this_ptr->isPrefixOf(*other);
}

int qt_core_c_QVersionNumber_majorVersion(const QVersionNumber* this_ptr) {
  return this_ptr->majorVersion();
}

int qt_core_c_QVersionNumber_microVersion(const QVersionNumber* this_ptr) {
  return this_ptr->microVersion();
}

int qt_core_c_QVersionNumber_minorVersion(const QVersionNumber* this_ptr) {
  return this_ptr->minorVersion();
}

void qt_core_c_QVersionNumber_normalized_to_output(const QVersionNumber* this_ptr, QVersionNumber* output) {
  new(output) QVersionNumber(this_ptr->normalized());
}

int qt_core_c_QVersionNumber_segmentAt(const QVersionNumber* this_ptr, int index) {
  return this_ptr->segmentAt(index);
}

int qt_core_c_QVersionNumber_segmentCount(const QVersionNumber* this_ptr) {
  return this_ptr->segmentCount();
}

void qt_core_c_QVersionNumber_segments_to_output(const QVersionNumber* this_ptr, QVector< int >* output) {
  new(output) QVector< int >(this_ptr->segments());
}

void qt_core_c_QVersionNumber_toString_to_output(const QVersionNumber* this_ptr, QString* output) {
  new(output) QString(this_ptr->toString());
}

