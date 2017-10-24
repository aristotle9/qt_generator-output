#include "qt_core_c_QUrlTwoFlags.h"

void qt_core_c_QUrlTwoFlags_QUrl_UrlFormattingOption_QUrl_ComponentFormattingOption_constructor_QFlags_QUrl_ComponentFormattingOption(unsigned int f, QUrlTwoFlags< QUrl::UrlFormattingOption, QUrl::ComponentFormattingOption >* output) {
  new(output) QUrlTwoFlags< QUrl::UrlFormattingOption, QUrl::ComponentFormattingOption >(QFlags< QUrl::ComponentFormattingOption >(f));
}

void qt_core_c_QUrlTwoFlags_QUrl_UrlFormattingOption_QUrl_ComponentFormattingOption_constructor_QUrl_ComponentFormattingOption(QUrl::ComponentFormattingOption f, QUrlTwoFlags< QUrl::UrlFormattingOption, QUrl::ComponentFormattingOption >* output) {
  new(output) QUrlTwoFlags< QUrl::UrlFormattingOption, QUrl::ComponentFormattingOption >(f);
}

void qt_core_c_QUrlTwoFlags_QUrl_UrlFormattingOption_QUrl_ComponentFormattingOption_constructor_QUrl_UrlFormattingOption(QUrl::UrlFormattingOption f, QUrlTwoFlags< QUrl::UrlFormattingOption, QUrl::ComponentFormattingOption >* output) {
  new(output) QUrlTwoFlags< QUrl::UrlFormattingOption, QUrl::ComponentFormattingOption >(f);
}

unsigned int qt_core_c_QUrlTwoFlags_QUrl_UrlFormattingOption_QUrl_ComponentFormattingOption_convert_to_QFlags_QUrl_ComponentFormattingOption(const QUrlTwoFlags< QUrl::UrlFormattingOption, QUrl::ComponentFormattingOption >* this_ptr) {
  return uint(this_ptr->operator QFlags< QUrl::ComponentFormattingOption >());
}

int qt_core_c_QUrlTwoFlags_QUrl_UrlFormattingOption_QUrl_ComponentFormattingOption_convert_to_int(const QUrlTwoFlags< QUrl::UrlFormattingOption, QUrl::ComponentFormattingOption >* this_ptr) {
  return this_ptr->operator int();
}

void qt_core_c_QUrlTwoFlags_QUrl_UrlFormattingOption_QUrl_ComponentFormattingOption_destructor(QUrlTwoFlags< QUrl::UrlFormattingOption, QUrl::ComponentFormattingOption >* this_ptr) {
  qt_core_c_call_destructor(this_ptr);
}

QUrlTwoFlags< QUrl::UrlFormattingOption, QUrl::ComponentFormattingOption >* qt_core_c_QUrlTwoFlags_QUrl_UrlFormattingOption_QUrl_ComponentFormattingOption_operator_bit_and_assign_int(QUrlTwoFlags< QUrl::UrlFormattingOption, QUrl::ComponentFormattingOption >* this_ptr, int mask) {
  return &this_ptr->operator&=(mask);
}

QUrlTwoFlags< QUrl::UrlFormattingOption, QUrl::ComponentFormattingOption >* qt_core_c_QUrlTwoFlags_QUrl_UrlFormattingOption_QUrl_ComponentFormattingOption_operator_bit_and_assign_unsigned_int(QUrlTwoFlags< QUrl::UrlFormattingOption, QUrl::ComponentFormattingOption >* this_ptr, unsigned int mask) {
  return &this_ptr->operator&=(mask);
}

void qt_core_c_QUrlTwoFlags_QUrl_UrlFormattingOption_QUrl_ComponentFormattingOption_operator_bit_and_to_output_QUrl_ComponentFormattingOption(const QUrlTwoFlags< QUrl::UrlFormattingOption, QUrl::ComponentFormattingOption >* this_ptr, QUrl::ComponentFormattingOption f, QUrlTwoFlags< QUrl::UrlFormattingOption, QUrl::ComponentFormattingOption >* output) {
  new(output) QUrlTwoFlags< QUrl::UrlFormattingOption, QUrl::ComponentFormattingOption >(this_ptr->operator&(f));
}

void qt_core_c_QUrlTwoFlags_QUrl_UrlFormattingOption_QUrl_ComponentFormattingOption_operator_bit_and_to_output_QUrl_UrlFormattingOption(const QUrlTwoFlags< QUrl::UrlFormattingOption, QUrl::ComponentFormattingOption >* this_ptr, QUrl::UrlFormattingOption f, QUrlTwoFlags< QUrl::UrlFormattingOption, QUrl::ComponentFormattingOption >* output) {
  new(output) QUrlTwoFlags< QUrl::UrlFormattingOption, QUrl::ComponentFormattingOption >(this_ptr->operator&(f));
}

void qt_core_c_QUrlTwoFlags_QUrl_UrlFormattingOption_QUrl_ComponentFormattingOption_operator_bit_and_to_output_int(const QUrlTwoFlags< QUrl::UrlFormattingOption, QUrl::ComponentFormattingOption >* this_ptr, int mask, QUrlTwoFlags< QUrl::UrlFormattingOption, QUrl::ComponentFormattingOption >* output) {
  new(output) QUrlTwoFlags< QUrl::UrlFormattingOption, QUrl::ComponentFormattingOption >(this_ptr->operator&(mask));
}

void qt_core_c_QUrlTwoFlags_QUrl_UrlFormattingOption_QUrl_ComponentFormattingOption_operator_bit_and_to_output_unsigned_int(const QUrlTwoFlags< QUrl::UrlFormattingOption, QUrl::ComponentFormattingOption >* this_ptr, unsigned int mask, QUrlTwoFlags< QUrl::UrlFormattingOption, QUrl::ComponentFormattingOption >* output) {
  new(output) QUrlTwoFlags< QUrl::UrlFormattingOption, QUrl::ComponentFormattingOption >(this_ptr->operator&(mask));
}

void qt_core_c_QUrlTwoFlags_QUrl_UrlFormattingOption_QUrl_ComponentFormattingOption_operator_bit_not_to_output(const QUrlTwoFlags< QUrl::UrlFormattingOption, QUrl::ComponentFormattingOption >* this_ptr, QUrlTwoFlags< QUrl::UrlFormattingOption, QUrl::ComponentFormattingOption >* output) {
  new(output) QUrlTwoFlags< QUrl::UrlFormattingOption, QUrl::ComponentFormattingOption >(this_ptr->operator~());
}

QUrlTwoFlags< QUrl::UrlFormattingOption, QUrl::ComponentFormattingOption >* qt_core_c_QUrlTwoFlags_QUrl_UrlFormattingOption_QUrl_ComponentFormattingOption_operator_bit_or_assign_QUrlTwoFlags_QUrl_UrlFormattingOption_QUrl_ComponentFormattingOption(QUrlTwoFlags< QUrl::UrlFormattingOption, QUrl::ComponentFormattingOption >* this_ptr, const QUrlTwoFlags< QUrl::UrlFormattingOption, QUrl::ComponentFormattingOption >* f) {
  return &this_ptr->operator|=(*f);
}

QUrlTwoFlags< QUrl::UrlFormattingOption, QUrl::ComponentFormattingOption >* qt_core_c_QUrlTwoFlags_QUrl_UrlFormattingOption_QUrl_ComponentFormattingOption_operator_bit_or_assign_QUrl_ComponentFormattingOption(QUrlTwoFlags< QUrl::UrlFormattingOption, QUrl::ComponentFormattingOption >* this_ptr, QUrl::ComponentFormattingOption f) {
  return &this_ptr->operator|=(f);
}

QUrlTwoFlags< QUrl::UrlFormattingOption, QUrl::ComponentFormattingOption >* qt_core_c_QUrlTwoFlags_QUrl_UrlFormattingOption_QUrl_ComponentFormattingOption_operator_bit_or_assign_QUrl_UrlFormattingOption(QUrlTwoFlags< QUrl::UrlFormattingOption, QUrl::ComponentFormattingOption >* this_ptr, QUrl::UrlFormattingOption f) {
  return &this_ptr->operator|=(f);
}

void qt_core_c_QUrlTwoFlags_QUrl_UrlFormattingOption_QUrl_ComponentFormattingOption_operator_bit_or_to_output_QUrlTwoFlags_QUrl_UrlFormattingOption_QUrl_ComponentFormattingOption(const QUrlTwoFlags< QUrl::UrlFormattingOption, QUrl::ComponentFormattingOption >* this_ptr, const QUrlTwoFlags< QUrl::UrlFormattingOption, QUrl::ComponentFormattingOption >* f, QUrlTwoFlags< QUrl::UrlFormattingOption, QUrl::ComponentFormattingOption >* output) {
  new(output) QUrlTwoFlags< QUrl::UrlFormattingOption, QUrl::ComponentFormattingOption >(this_ptr->operator|(*f));
}

void qt_core_c_QUrlTwoFlags_QUrl_UrlFormattingOption_QUrl_ComponentFormattingOption_operator_bit_or_to_output_QUrl_ComponentFormattingOption(const QUrlTwoFlags< QUrl::UrlFormattingOption, QUrl::ComponentFormattingOption >* this_ptr, QUrl::ComponentFormattingOption f, QUrlTwoFlags< QUrl::UrlFormattingOption, QUrl::ComponentFormattingOption >* output) {
  new(output) QUrlTwoFlags< QUrl::UrlFormattingOption, QUrl::ComponentFormattingOption >(this_ptr->operator|(f));
}

void qt_core_c_QUrlTwoFlags_QUrl_UrlFormattingOption_QUrl_ComponentFormattingOption_operator_bit_or_to_output_QUrl_UrlFormattingOption(const QUrlTwoFlags< QUrl::UrlFormattingOption, QUrl::ComponentFormattingOption >* this_ptr, QUrl::UrlFormattingOption f, QUrlTwoFlags< QUrl::UrlFormattingOption, QUrl::ComponentFormattingOption >* output) {
  new(output) QUrlTwoFlags< QUrl::UrlFormattingOption, QUrl::ComponentFormattingOption >(this_ptr->operator|(f));
}

QUrlTwoFlags< QUrl::UrlFormattingOption, QUrl::ComponentFormattingOption >* qt_core_c_QUrlTwoFlags_QUrl_UrlFormattingOption_QUrl_ComponentFormattingOption_operator_bit_xor_assign_QUrlTwoFlags_QUrl_UrlFormattingOption_QUrl_ComponentFormattingOption(QUrlTwoFlags< QUrl::UrlFormattingOption, QUrl::ComponentFormattingOption >* this_ptr, const QUrlTwoFlags< QUrl::UrlFormattingOption, QUrl::ComponentFormattingOption >* f) {
  return &this_ptr->operator^=(*f);
}

QUrlTwoFlags< QUrl::UrlFormattingOption, QUrl::ComponentFormattingOption >* qt_core_c_QUrlTwoFlags_QUrl_UrlFormattingOption_QUrl_ComponentFormattingOption_operator_bit_xor_assign_QUrl_ComponentFormattingOption(QUrlTwoFlags< QUrl::UrlFormattingOption, QUrl::ComponentFormattingOption >* this_ptr, QUrl::ComponentFormattingOption f) {
  return &this_ptr->operator^=(f);
}

QUrlTwoFlags< QUrl::UrlFormattingOption, QUrl::ComponentFormattingOption >* qt_core_c_QUrlTwoFlags_QUrl_UrlFormattingOption_QUrl_ComponentFormattingOption_operator_bit_xor_assign_QUrl_UrlFormattingOption(QUrlTwoFlags< QUrl::UrlFormattingOption, QUrl::ComponentFormattingOption >* this_ptr, QUrl::UrlFormattingOption f) {
  return &this_ptr->operator^=(f);
}

void qt_core_c_QUrlTwoFlags_QUrl_UrlFormattingOption_QUrl_ComponentFormattingOption_operator_bit_xor_to_output_QUrlTwoFlags_QUrl_UrlFormattingOption_QUrl_ComponentFormattingOption(const QUrlTwoFlags< QUrl::UrlFormattingOption, QUrl::ComponentFormattingOption >* this_ptr, const QUrlTwoFlags< QUrl::UrlFormattingOption, QUrl::ComponentFormattingOption >* f, QUrlTwoFlags< QUrl::UrlFormattingOption, QUrl::ComponentFormattingOption >* output) {
  new(output) QUrlTwoFlags< QUrl::UrlFormattingOption, QUrl::ComponentFormattingOption >(this_ptr->operator^(*f));
}

void qt_core_c_QUrlTwoFlags_QUrl_UrlFormattingOption_QUrl_ComponentFormattingOption_operator_bit_xor_to_output_QUrl_ComponentFormattingOption(const QUrlTwoFlags< QUrl::UrlFormattingOption, QUrl::ComponentFormattingOption >* this_ptr, QUrl::ComponentFormattingOption f, QUrlTwoFlags< QUrl::UrlFormattingOption, QUrl::ComponentFormattingOption >* output) {
  new(output) QUrlTwoFlags< QUrl::UrlFormattingOption, QUrl::ComponentFormattingOption >(this_ptr->operator^(f));
}

void qt_core_c_QUrlTwoFlags_QUrl_UrlFormattingOption_QUrl_ComponentFormattingOption_operator_bit_xor_to_output_QUrl_UrlFormattingOption(const QUrlTwoFlags< QUrl::UrlFormattingOption, QUrl::ComponentFormattingOption >* this_ptr, QUrl::UrlFormattingOption f, QUrlTwoFlags< QUrl::UrlFormattingOption, QUrl::ComponentFormattingOption >* output) {
  new(output) QUrlTwoFlags< QUrl::UrlFormattingOption, QUrl::ComponentFormattingOption >(this_ptr->operator^(f));
}

bool qt_core_c_QUrlTwoFlags_QUrl_UrlFormattingOption_QUrl_ComponentFormattingOption_operator_not(const QUrlTwoFlags< QUrl::UrlFormattingOption, QUrl::ComponentFormattingOption >* this_ptr) {
  return this_ptr->operator!();
}

bool qt_core_c_QUrlTwoFlags_QUrl_UrlFormattingOption_QUrl_ComponentFormattingOption_testFlag_QUrl_ComponentFormattingOption(const QUrlTwoFlags< QUrl::UrlFormattingOption, QUrl::ComponentFormattingOption >* this_ptr, QUrl::ComponentFormattingOption f) {
  return this_ptr->testFlag(f);
}

bool qt_core_c_QUrlTwoFlags_QUrl_UrlFormattingOption_QUrl_ComponentFormattingOption_testFlag_QUrl_UrlFormattingOption(const QUrlTwoFlags< QUrl::UrlFormattingOption, QUrl::ComponentFormattingOption >* this_ptr, QUrl::UrlFormattingOption f) {
  return this_ptr->testFlag(f);
}

