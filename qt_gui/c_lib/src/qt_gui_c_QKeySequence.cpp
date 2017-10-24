#include "qt_gui_c_QKeySequence.h"

QDataStream* qt_gui_c_QKeySequence_G_operator_shl(QDataStream* in, const QKeySequence* ks) {
  return &operator<<(*in, *ks);
}

void qt_gui_c_QKeySequence_G_operator_shl_to_output(const QDebug* arg1, const QKeySequence* arg2, QDebug* output) {
  new(output) QDebug(operator<<(*arg1, *arg2));
}

QDataStream* qt_gui_c_QKeySequence_G_operator_shr(QDataStream* out, QKeySequence* ks) {
  return &operator>>(*out, *ks);
}

unsigned int qt_gui_c_QKeySequence_G_qHash_key(const QKeySequence* key) {
  return qHash(*key);
}

unsigned int qt_gui_c_QKeySequence_G_qHash_key_seed(const QKeySequence* key, unsigned int seed) {
  return qHash(*key, seed);
}

void qt_gui_c_QKeySequence_G_swap(QKeySequence* value1, QKeySequence* value2) {
  swap(*value1, *value2);
}

void qt_gui_c_QKeySequence_constructor_QKeySequence(const QKeySequence* ks, QKeySequence* output) {
  new(output) QKeySequence(*ks);
}

void qt_gui_c_QKeySequence_constructor_QKeySequence_StandardKey(QKeySequence::StandardKey key, QKeySequence* output) {
  new(output) QKeySequence(key);
}

void qt_gui_c_QKeySequence_constructor_QString(const QString* key, QKeySequence* output) {
  new(output) QKeySequence(*key);
}

void qt_gui_c_QKeySequence_constructor_QString_QKeySequence_SequenceFormat(const QString* key, QKeySequence::SequenceFormat format, QKeySequence* output) {
  new(output) QKeySequence(*key, format);
}

void qt_gui_c_QKeySequence_constructor_int(int k1, QKeySequence* output) {
  new(output) QKeySequence(k1);
}

void qt_gui_c_QKeySequence_constructor_int_int(int k1, int k2, QKeySequence* output) {
  new(output) QKeySequence(k1, k2);
}

void qt_gui_c_QKeySequence_constructor_int_int_int(int k1, int k2, int k3, QKeySequence* output) {
  new(output) QKeySequence(k1, k2, k3);
}

void qt_gui_c_QKeySequence_constructor_int_int_int_int(int k1, int k2, int k3, int k4, QKeySequence* output) {
  new(output) QKeySequence(k1, k2, k3, k4);
}

void qt_gui_c_QKeySequence_constructor_no_args(QKeySequence* output) {
  new(output) QKeySequence();
}

void qt_gui_c_QKeySequence_convert_to_QVariant_to_output(const QKeySequence* this_ptr, QVariant* output) {
  new(output) QVariant(this_ptr->operator QVariant());
}

int qt_gui_c_QKeySequence_count(const QKeySequence* this_ptr) {
  return this_ptr->count();
}

void qt_gui_c_QKeySequence_destructor(QKeySequence* this_ptr) {
  qt_gui_c_call_destructor(this_ptr);
}

void qt_gui_c_QKeySequence_fromString_to_output_str(const QString* str, QKeySequence* output) {
  new(output) QKeySequence(QKeySequence::fromString(*str));
}

void qt_gui_c_QKeySequence_fromString_to_output_str_format(const QString* str, QKeySequence::SequenceFormat format, QKeySequence* output) {
  new(output) QKeySequence(QKeySequence::fromString(*str, format));
}

bool qt_gui_c_QKeySequence_isEmpty(const QKeySequence* this_ptr) {
  return this_ptr->isEmpty();
}

void qt_gui_c_QKeySequence_keyBindings_to_output(QKeySequence::StandardKey key, QList< QKeySequence >* output) {
  new(output) QList< QKeySequence >(QKeySequence::keyBindings(key));
}

void qt_gui_c_QKeySequence_listFromString_to_output_str(const QString* str, QList< QKeySequence >* output) {
  new(output) QList< QKeySequence >(QKeySequence::listFromString(*str));
}

void qt_gui_c_QKeySequence_listFromString_to_output_str_format(const QString* str, QKeySequence::SequenceFormat format, QList< QKeySequence >* output) {
  new(output) QList< QKeySequence >(QKeySequence::listFromString(*str, format));
}

void qt_gui_c_QKeySequence_listToString_to_output_list(const QList< QKeySequence >* list, QString* output) {
  new(output) QString(QKeySequence::listToString(*list));
}

void qt_gui_c_QKeySequence_listToString_to_output_list_format(const QList< QKeySequence >* list, QKeySequence::SequenceFormat format, QString* output) {
  new(output) QString(QKeySequence::listToString(*list, format));
}

QKeySequence::SequenceMatch qt_gui_c_QKeySequence_matches(const QKeySequence* this_ptr, const QKeySequence* seq) {
  return this_ptr->matches(*seq);
}

void qt_gui_c_QKeySequence_mnemonic_to_output(const QString* text, QKeySequence* output) {
  new(output) QKeySequence(QKeySequence::mnemonic(*text));
}

QKeySequence* qt_gui_c_QKeySequence_operator_assign(QKeySequence* this_ptr, const QKeySequence* other) {
  return &this_ptr->operator=(*other);
}

bool qt_gui_c_QKeySequence_operator_eq(const QKeySequence* this_ptr, const QKeySequence* other) {
  return this_ptr->operator==(*other);
}

bool qt_gui_c_QKeySequence_operator_ge(const QKeySequence* this_ptr, const QKeySequence* other) {
  return this_ptr->operator>=(*other);
}

bool qt_gui_c_QKeySequence_operator_gt(const QKeySequence* this_ptr, const QKeySequence* other) {
  return this_ptr->operator>(*other);
}

int qt_gui_c_QKeySequence_operator_index(const QKeySequence* this_ptr, unsigned int i) {
  return this_ptr->operator[](i);
}

bool qt_gui_c_QKeySequence_operator_le(const QKeySequence* this_ptr, const QKeySequence* other) {
  return this_ptr->operator<=(*other);
}

bool qt_gui_c_QKeySequence_operator_lt(const QKeySequence* this_ptr, const QKeySequence* ks) {
  return this_ptr->operator<(*ks);
}

bool qt_gui_c_QKeySequence_operator_neq(const QKeySequence* this_ptr, const QKeySequence* other) {
  return this_ptr->operator!=(*other);
}

void qt_gui_c_QKeySequence_swap(QKeySequence* this_ptr, QKeySequence* other) {
  this_ptr->swap(*other);
}

void qt_gui_c_QKeySequence_toString_to_output_format(const QKeySequence* this_ptr, QKeySequence::SequenceFormat format, QString* output) {
  new(output) QString(this_ptr->toString(format));
}

void qt_gui_c_QKeySequence_toString_to_output_no_args(const QKeySequence* this_ptr, QString* output) {
  new(output) QString(this_ptr->toString());
}

