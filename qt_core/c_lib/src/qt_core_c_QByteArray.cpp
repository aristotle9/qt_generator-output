#include "qt_core_c_QByteArray.h"

void qt_core_c_QByteArray_G_operator_add_to_output_char_const_QByteArray_ref(char a1, const QByteArray* a2, QByteArray* output) {
  new(output) QByteArray(operator+(a1, *a2));
}

void qt_core_c_QByteArray_G_operator_add_to_output_const_QByteArray_ref_char(const QByteArray* a1, char a2, QByteArray* output) {
  new(output) QByteArray(operator+(*a1, a2));
}

void qt_core_c_QByteArray_G_operator_add_to_output_const_QByteArray_ref_const_QByteArray_ref(const QByteArray* a1, const QByteArray* a2, QByteArray* output) {
  new(output) QByteArray(operator+(*a1, *a2));
}

void qt_core_c_QByteArray_G_operator_add_to_output_const_QByteArray_ref_const_char_ptr(const QByteArray* a1, const char* a2, QByteArray* output) {
  new(output) QByteArray(operator+(*a1, a2));
}

void qt_core_c_QByteArray_G_operator_add_to_output_const_char_ptr_const_QByteArray_ref(const char* a1, const QByteArray* a2, QByteArray* output) {
  new(output) QByteArray(operator+(a1, *a2));
}

bool qt_core_c_QByteArray_G_operator_eq_QByteArray_QByteArray(const QByteArray* a1, const QByteArray* a2) {
  return operator==(*a1, *a2);
}

bool qt_core_c_QByteArray_G_operator_eq_QByteArray_char(const QByteArray* a1, const char* a2) {
  return operator==(*a1, a2);
}

bool qt_core_c_QByteArray_G_operator_eq_char_QByteArray(const char* a1, const QByteArray* a2) {
  return operator==(a1, *a2);
}

bool qt_core_c_QByteArray_G_operator_ge_QByteArray_QByteArray(const QByteArray* a1, const QByteArray* a2) {
  return operator>=(*a1, *a2);
}

bool qt_core_c_QByteArray_G_operator_ge_QByteArray_char(const QByteArray* a1, const char* a2) {
  return operator>=(*a1, a2);
}

bool qt_core_c_QByteArray_G_operator_ge_char_QByteArray(const char* a1, const QByteArray* a2) {
  return operator>=(a1, *a2);
}

bool qt_core_c_QByteArray_G_operator_gt_QByteArray_QByteArray(const QByteArray* a1, const QByteArray* a2) {
  return operator>(*a1, *a2);
}

bool qt_core_c_QByteArray_G_operator_gt_QByteArray_char(const QByteArray* a1, const char* a2) {
  return operator>(*a1, a2);
}

bool qt_core_c_QByteArray_G_operator_gt_char_QByteArray(const char* a1, const QByteArray* a2) {
  return operator>(a1, *a2);
}

bool qt_core_c_QByteArray_G_operator_le_QByteArray_QByteArray(const QByteArray* a1, const QByteArray* a2) {
  return operator<=(*a1, *a2);
}

bool qt_core_c_QByteArray_G_operator_le_QByteArray_char(const QByteArray* a1, const char* a2) {
  return operator<=(*a1, a2);
}

bool qt_core_c_QByteArray_G_operator_le_char_QByteArray(const char* a1, const QByteArray* a2) {
  return operator<=(a1, *a2);
}

bool qt_core_c_QByteArray_G_operator_lt_QByteArray_QByteArray(const QByteArray* a1, const QByteArray* a2) {
  return operator<(*a1, *a2);
}

bool qt_core_c_QByteArray_G_operator_lt_QByteArray_char(const QByteArray* a1, const char* a2) {
  return operator<(*a1, a2);
}

bool qt_core_c_QByteArray_G_operator_lt_char_QByteArray(const char* a1, const QByteArray* a2) {
  return operator<(a1, *a2);
}

bool qt_core_c_QByteArray_G_operator_neq_QByteArray_QByteArray(const QByteArray* a1, const QByteArray* a2) {
  return operator!=(*a1, *a2);
}

bool qt_core_c_QByteArray_G_operator_neq_QByteArray_char(const QByteArray* a1, const char* a2) {
  return operator!=(*a1, a2);
}

bool qt_core_c_QByteArray_G_operator_neq_char_QByteArray(const char* a1, const QByteArray* a2) {
  return operator!=(a1, *a2);
}

QDataStream* qt_core_c_QByteArray_G_operator_shl(QDataStream* arg1, const QByteArray* arg2) {
  return &operator<<(*arg1, *arg2);
}

QDataStream* qt_core_c_QByteArray_G_operator_shr(QDataStream* arg1, QByteArray* arg2) {
  return &operator>>(*arg1, *arg2);
}

quint16 qt_core_c_QByteArray_G_qChecksum_s_len(const char* s, unsigned int len) {
  return qChecksum(s, len);
}

quint16 qt_core_c_QByteArray_G_qChecksum_s_len_standard(const char* s, unsigned int len, const Qt::ChecksumType* standard) {
  return qChecksum(s, len, *standard);
}

void qt_core_c_QByteArray_G_qCompress_to_output_data(const QByteArray* data, QByteArray* output) {
  new(output) QByteArray(qCompress(*data));
}

void qt_core_c_QByteArray_G_qCompress_to_output_data_compressionLevel(const QByteArray* data, int compressionLevel, QByteArray* output) {
  new(output) QByteArray(qCompress(*data, compressionLevel));
}

void qt_core_c_QByteArray_G_qCompress_to_output_data_nbytes(const unsigned char* data, int nbytes, QByteArray* output) {
  new(output) QByteArray(qCompress(data, nbytes));
}

void qt_core_c_QByteArray_G_qCompress_to_output_data_nbytes_compressionLevel(const unsigned char* data, int nbytes, int compressionLevel, QByteArray* output) {
  new(output) QByteArray(qCompress(data, nbytes, compressionLevel));
}

void qt_core_c_QByteArray_G_qUncompress_to_output(const QByteArray* data, QByteArray* output) {
  new(output) QByteArray(qUncompress(*data));
}

int qt_core_c_QByteArray_G_qstrcmp_QByteArray_QByteArray(const QByteArray* str1, const QByteArray* str2) {
  return qstrcmp(*str1, *str2);
}

int qt_core_c_QByteArray_G_qstrcmp_QByteArray_char(const QByteArray* str1, const char* str2) {
  return qstrcmp(*str1, str2);
}

int qt_core_c_QByteArray_G_qstrcmp_char_QByteArray(const char* str1, const QByteArray* str2) {
  return qstrcmp(str1, *str2);
}

int qt_core_c_QByteArray_G_qstrcmp_char_char(const char* str1, const char* str2) {
  return qstrcmp(str1, str2);
}

char* qt_core_c_QByteArray_G_qstrcpy(char* dst, const char* src) {
  return qstrcpy(dst, src);
}

char* qt_core_c_QByteArray_G_qstrdup(const char* arg1) {
  return qstrdup(arg1);
}

int qt_core_c_QByteArray_G_qstricmp(const char* arg1, const char* arg2) {
  return qstricmp(arg1, arg2);
}

unsigned int qt_core_c_QByteArray_G_qstrlen(const char* str) {
  return qstrlen(str);
}

int qt_core_c_QByteArray_G_qstrncmp(const char* str1, const char* str2, unsigned int len) {
  return qstrncmp(str1, str2, len);
}

char* qt_core_c_QByteArray_G_qstrncpy(char* dst, const char* src, unsigned int len) {
  return qstrncpy(dst, src, len);
}

int qt_core_c_QByteArray_G_qstrnicmp(const char* arg1, const char* arg2, unsigned int len) {
  return qstrnicmp(arg1, arg2, len);
}

unsigned int qt_core_c_QByteArray_G_qstrnlen(const char* str, unsigned int maxlen) {
  return qstrnlen(str, maxlen);
}

void qt_core_c_QByteArray_G_swap(QByteArray* value1, QByteArray* value2) {
  swap(*value1, *value2);
}

QByteArray* qt_core_c_QByteArray_append_QByteArray_a(QByteArray* this_ptr, const QByteArray* a) {
  return &this_ptr->append(*a);
}

QByteArray* qt_core_c_QByteArray_append_QString_s(QByteArray* this_ptr, const QString* s) {
  return &this_ptr->append(*s);
}

QByteArray* qt_core_c_QByteArray_append_char_c(QByteArray* this_ptr, char c) {
  return &this_ptr->append(c);
}

QByteArray* qt_core_c_QByteArray_append_char_s(QByteArray* this_ptr, const char* s) {
  return &this_ptr->append(s);
}

QByteArray* qt_core_c_QByteArray_append_char_s_int_len(QByteArray* this_ptr, const char* s, int len) {
  return &this_ptr->append(s, len);
}

QByteArray* qt_core_c_QByteArray_append_int_count_char_c(QByteArray* this_ptr, int count, char c) {
  return &this_ptr->append(count, c);
}

char qt_core_c_QByteArray_at(const QByteArray* this_ptr, int i) {
  return this_ptr->at(i);
}

char* qt_core_c_QByteArray_begin(QByteArray* this_ptr) {
  return this_ptr->begin();
}

const char* qt_core_c_QByteArray_begin_const(const QByteArray* this_ptr) {
  return this_ptr->begin();
}

int qt_core_c_QByteArray_capacity(const QByteArray* this_ptr) {
  return this_ptr->capacity();
}

const char* qt_core_c_QByteArray_cbegin(const QByteArray* this_ptr) {
  return this_ptr->cbegin();
}

const char* qt_core_c_QByteArray_cend(const QByteArray* this_ptr) {
  return this_ptr->cend();
}

void qt_core_c_QByteArray_chop(QByteArray* this_ptr, int n) {
  this_ptr->chop(n);
}

void qt_core_c_QByteArray_clear(QByteArray* this_ptr) {
  this_ptr->clear();
}

const char* qt_core_c_QByteArray_constBegin(const QByteArray* this_ptr) {
  return this_ptr->constBegin();
}

const char* qt_core_c_QByteArray_constData(const QByteArray* this_ptr) {
  return this_ptr->constData();
}

const char* qt_core_c_QByteArray_constEnd(const QByteArray* this_ptr) {
  return this_ptr->constEnd();
}

void qt_core_c_QByteArray_constructor_QByteArray(const QByteArray* arg1, QByteArray* output) {
  new(output) QByteArray(*arg1);
}

void qt_core_c_QByteArray_constructor_char(const char* arg1, QByteArray* output) {
  new(output) QByteArray(arg1);
}

void qt_core_c_QByteArray_constructor_char_int(const char* arg1, int size, QByteArray* output) {
  new(output) QByteArray(arg1, size);
}

void qt_core_c_QByteArray_constructor_int_char(int size, char c, QByteArray* output) {
  new(output) QByteArray(size, c);
}

void qt_core_c_QByteArray_constructor_no_args(QByteArray* output) {
  new(output) QByteArray();
}

bool qt_core_c_QByteArray_contains_QByteArray_a(const QByteArray* this_ptr, const QByteArray* a) {
  return this_ptr->contains(*a);
}

bool qt_core_c_QByteArray_contains_char_a(const QByteArray* this_ptr, const char* a) {
  return this_ptr->contains(a);
}

bool qt_core_c_QByteArray_contains_char_c(const QByteArray* this_ptr, char c) {
  return this_ptr->contains(c);
}

const char* qt_core_c_QByteArray_convert_to_const_char_ptr(const QByteArray* this_ptr) {
  return this_ptr->operator const char *();
}

const void* qt_core_c_QByteArray_convert_to_const_void_ptr(const QByteArray* this_ptr) {
  return this_ptr->operator const void *();
}

int qt_core_c_QByteArray_count_QByteArray_a(const QByteArray* this_ptr, const QByteArray* a) {
  return this_ptr->count(*a);
}

int qt_core_c_QByteArray_count_char_a(const QByteArray* this_ptr, const char* a) {
  return this_ptr->count(a);
}

int qt_core_c_QByteArray_count_char_c(const QByteArray* this_ptr, char c) {
  return this_ptr->count(c);
}

int qt_core_c_QByteArray_count_no_args(const QByteArray* this_ptr) {
  return this_ptr->count();
}

char* qt_core_c_QByteArray_data(QByteArray* this_ptr) {
  return this_ptr->data();
}

const char* qt_core_c_QByteArray_data_const(const QByteArray* this_ptr) {
  return this_ptr->data();
}

void qt_core_c_QByteArray_destructor(QByteArray* this_ptr) {
  qt_core_c_call_destructor(this_ptr);
}

char* qt_core_c_QByteArray_end(QByteArray* this_ptr) {
  return this_ptr->end();
}

const char* qt_core_c_QByteArray_end_const(const QByteArray* this_ptr) {
  return this_ptr->end();
}

bool qt_core_c_QByteArray_endsWith_char(const QByteArray* this_ptr, char c) {
  return this_ptr->endsWith(c);
}

bool qt_core_c_QByteArray_endsWith_const_QByteArray_ref(const QByteArray* this_ptr, const QByteArray* a) {
  return this_ptr->endsWith(*a);
}

bool qt_core_c_QByteArray_endsWith_const_char_ptr(const QByteArray* this_ptr, const char* c) {
  return this_ptr->endsWith(c);
}

QByteArray* qt_core_c_QByteArray_fill_c(QByteArray* this_ptr, char c) {
  return &this_ptr->fill(c);
}

QByteArray* qt_core_c_QByteArray_fill_c_size(QByteArray* this_ptr, char c, int size) {
  return &this_ptr->fill(c, size);
}

void qt_core_c_QByteArray_fromBase64_to_output_base64(const QByteArray* base64, QByteArray* output) {
  new(output) QByteArray(QByteArray::fromBase64(*base64));
}

void qt_core_c_QByteArray_fromBase64_to_output_base64_options(const QByteArray* base64, unsigned int options, QByteArray* output) {
  new(output) QByteArray(QByteArray::fromBase64(*base64, QFlags< QByteArray::Base64Option >(options)));
}

void qt_core_c_QByteArray_fromHex_to_output(const QByteArray* hexEncoded, QByteArray* output) {
  new(output) QByteArray(QByteArray::fromHex(*hexEncoded));
}

void qt_core_c_QByteArray_fromPercentEncoding_to_output_pctEncoded(const QByteArray* pctEncoded, QByteArray* output) {
  new(output) QByteArray(QByteArray::fromPercentEncoding(*pctEncoded));
}

void qt_core_c_QByteArray_fromPercentEncoding_to_output_pctEncoded_percent(const QByteArray* pctEncoded, char percent, QByteArray* output) {
  new(output) QByteArray(QByteArray::fromPercentEncoding(*pctEncoded, percent));
}

void qt_core_c_QByteArray_fromRawData_to_output(const char* arg1, int size, QByteArray* output) {
  new(output) QByteArray(QByteArray::fromRawData(arg1, size));
}

int qt_core_c_QByteArray_indexOf_char(const QByteArray* this_ptr, char c) {
  return this_ptr->indexOf(c);
}

int qt_core_c_QByteArray_indexOf_char_int(const QByteArray* this_ptr, char c, int from) {
  return this_ptr->indexOf(c, from);
}

int qt_core_c_QByteArray_indexOf_const_QByteArray_ref(const QByteArray* this_ptr, const QByteArray* a) {
  return this_ptr->indexOf(*a);
}

int qt_core_c_QByteArray_indexOf_const_QByteArray_ref_int(const QByteArray* this_ptr, const QByteArray* a, int from) {
  return this_ptr->indexOf(*a, from);
}

int qt_core_c_QByteArray_indexOf_const_QString_ref(const QByteArray* this_ptr, const QString* s) {
  return this_ptr->indexOf(*s);
}

int qt_core_c_QByteArray_indexOf_const_QString_ref_int(const QByteArray* this_ptr, const QString* s, int from) {
  return this_ptr->indexOf(*s, from);
}

int qt_core_c_QByteArray_indexOf_const_char_ptr(const QByteArray* this_ptr, const char* c) {
  return this_ptr->indexOf(c);
}

int qt_core_c_QByteArray_indexOf_const_char_ptr_int(const QByteArray* this_ptr, const char* c, int from) {
  return this_ptr->indexOf(c, from);
}

QByteArray* qt_core_c_QByteArray_insert_int_i_QByteArray_a(QByteArray* this_ptr, int i, const QByteArray* a) {
  return &this_ptr->insert(i, *a);
}

QByteArray* qt_core_c_QByteArray_insert_int_i_QString_s(QByteArray* this_ptr, int i, const QString* s) {
  return &this_ptr->insert(i, *s);
}

QByteArray* qt_core_c_QByteArray_insert_int_i_char_c(QByteArray* this_ptr, int i, char c) {
  return &this_ptr->insert(i, c);
}

QByteArray* qt_core_c_QByteArray_insert_int_i_char_s(QByteArray* this_ptr, int i, const char* s) {
  return &this_ptr->insert(i, s);
}

QByteArray* qt_core_c_QByteArray_insert_int_i_char_s_int_len(QByteArray* this_ptr, int i, const char* s, int len) {
  return &this_ptr->insert(i, s, len);
}

QByteArray* qt_core_c_QByteArray_insert_int_i_int_count_char_c(QByteArray* this_ptr, int i, int count, char c) {
  return &this_ptr->insert(i, count, c);
}

bool qt_core_c_QByteArray_isEmpty(const QByteArray* this_ptr) {
  return this_ptr->isEmpty();
}

bool qt_core_c_QByteArray_isNull(const QByteArray* this_ptr) {
  return this_ptr->isNull();
}

int qt_core_c_QByteArray_lastIndexOf_char(const QByteArray* this_ptr, char c) {
  return this_ptr->lastIndexOf(c);
}

int qt_core_c_QByteArray_lastIndexOf_char_int(const QByteArray* this_ptr, char c, int from) {
  return this_ptr->lastIndexOf(c, from);
}

int qt_core_c_QByteArray_lastIndexOf_const_QByteArray_ref(const QByteArray* this_ptr, const QByteArray* a) {
  return this_ptr->lastIndexOf(*a);
}

int qt_core_c_QByteArray_lastIndexOf_const_QByteArray_ref_int(const QByteArray* this_ptr, const QByteArray* a, int from) {
  return this_ptr->lastIndexOf(*a, from);
}

int qt_core_c_QByteArray_lastIndexOf_const_QString_ref(const QByteArray* this_ptr, const QString* s) {
  return this_ptr->lastIndexOf(*s);
}

int qt_core_c_QByteArray_lastIndexOf_const_QString_ref_int(const QByteArray* this_ptr, const QString* s, int from) {
  return this_ptr->lastIndexOf(*s, from);
}

int qt_core_c_QByteArray_lastIndexOf_const_char_ptr(const QByteArray* this_ptr, const char* c) {
  return this_ptr->lastIndexOf(c);
}

int qt_core_c_QByteArray_lastIndexOf_const_char_ptr_int(const QByteArray* this_ptr, const char* c, int from) {
  return this_ptr->lastIndexOf(c, from);
}

void qt_core_c_QByteArray_leftJustified_to_output_width(const QByteArray* this_ptr, int width, QByteArray* output) {
  new(output) QByteArray(this_ptr->leftJustified(width));
}

void qt_core_c_QByteArray_leftJustified_to_output_width_fill(const QByteArray* this_ptr, int width, char fill, QByteArray* output) {
  new(output) QByteArray(this_ptr->leftJustified(width, fill));
}

void qt_core_c_QByteArray_leftJustified_to_output_width_fill_truncate(const QByteArray* this_ptr, int width, char fill, bool truncate, QByteArray* output) {
  new(output) QByteArray(this_ptr->leftJustified(width, fill, truncate));
}

void qt_core_c_QByteArray_left_to_output(const QByteArray* this_ptr, int len, QByteArray* output) {
  new(output) QByteArray(this_ptr->left(len));
}

int qt_core_c_QByteArray_length(const QByteArray* this_ptr) {
  return this_ptr->length();
}

void qt_core_c_QByteArray_mid_to_output_index(const QByteArray* this_ptr, int index, QByteArray* output) {
  new(output) QByteArray(this_ptr->mid(index));
}

void qt_core_c_QByteArray_mid_to_output_index_len(const QByteArray* this_ptr, int index, int len, QByteArray* output) {
  new(output) QByteArray(this_ptr->mid(index, len));
}

void qt_core_c_QByteArray_number_to_output_double(double arg1, QByteArray* output) {
  new(output) QByteArray(QByteArray::number(arg1));
}

void qt_core_c_QByteArray_number_to_output_double_char(double arg1, char f, QByteArray* output) {
  new(output) QByteArray(QByteArray::number(arg1, f));
}

void qt_core_c_QByteArray_number_to_output_double_char_int(double arg1, char f, int prec, QByteArray* output) {
  new(output) QByteArray(QByteArray::number(arg1, f, prec));
}

void qt_core_c_QByteArray_number_to_output_int(int arg1, QByteArray* output) {
  new(output) QByteArray(QByteArray::number(arg1));
}

void qt_core_c_QByteArray_number_to_output_int_int(int arg1, int base, QByteArray* output) {
  new(output) QByteArray(QByteArray::number(arg1, base));
}

void qt_core_c_QByteArray_number_to_output_qlonglong(qlonglong arg1, QByteArray* output) {
  new(output) QByteArray(QByteArray::number(arg1));
}

void qt_core_c_QByteArray_number_to_output_qlonglong_int(qlonglong arg1, int base, QByteArray* output) {
  new(output) QByteArray(QByteArray::number(arg1, base));
}

void qt_core_c_QByteArray_number_to_output_qulonglong(qulonglong arg1, QByteArray* output) {
  new(output) QByteArray(QByteArray::number(arg1));
}

void qt_core_c_QByteArray_number_to_output_qulonglong_int(qulonglong arg1, int base, QByteArray* output) {
  new(output) QByteArray(QByteArray::number(arg1, base));
}

void qt_core_c_QByteArray_number_to_output_unsigned_int(unsigned int arg1, QByteArray* output) {
  new(output) QByteArray(QByteArray::number(arg1));
}

void qt_core_c_QByteArray_number_to_output_unsigned_int_int(unsigned int arg1, int base, QByteArray* output) {
  new(output) QByteArray(QByteArray::number(arg1, base));
}

QByteArray* qt_core_c_QByteArray_operator_add_assign_QByteArray_a(QByteArray* this_ptr, const QByteArray* a) {
  return &this_ptr->operator+=(*a);
}

QByteArray* qt_core_c_QByteArray_operator_add_assign_QString_s(QByteArray* this_ptr, const QString* s) {
  return &this_ptr->operator+=(*s);
}

QByteArray* qt_core_c_QByteArray_operator_add_assign_char_c(QByteArray* this_ptr, char c) {
  return &this_ptr->operator+=(c);
}

QByteArray* qt_core_c_QByteArray_operator_add_assign_char_s(QByteArray* this_ptr, const char* s) {
  return &this_ptr->operator+=(s);
}

QByteArray* qt_core_c_QByteArray_operator_assign_arg1(QByteArray* this_ptr, const QByteArray* arg1) {
  return &this_ptr->operator=(*arg1);
}

QByteArray* qt_core_c_QByteArray_operator_assign_str(QByteArray* this_ptr, const char* str) {
  return &this_ptr->operator=(str);
}

bool qt_core_c_QByteArray_operator_eq(const QByteArray* this_ptr, const QString* s2) {
  return this_ptr->operator==(*s2);
}

bool qt_core_c_QByteArray_operator_ge(const QByteArray* this_ptr, const QString* s2) {
  return this_ptr->operator>=(*s2);
}

bool qt_core_c_QByteArray_operator_gt(const QByteArray* this_ptr, const QString* s2) {
  return this_ptr->operator>(*s2);
}

char qt_core_c_QByteArray_operator_index_int(const QByteArray* this_ptr, int i) {
  return this_ptr->operator[](i);
}

void qt_core_c_QByteArray_operator_index_to_output_int(QByteArray* this_ptr, int i, QByteRef* output) {
  new(output) QByteRef(this_ptr->operator[](i));
}

void qt_core_c_QByteArray_operator_index_to_output_unsigned_int(QByteArray* this_ptr, unsigned int i, QByteRef* output) {
  new(output) QByteRef(this_ptr->operator[](i));
}

char qt_core_c_QByteArray_operator_index_unsigned_int(const QByteArray* this_ptr, unsigned int i) {
  return this_ptr->operator[](i);
}

bool qt_core_c_QByteArray_operator_le(const QByteArray* this_ptr, const QString* s2) {
  return this_ptr->operator<=(*s2);
}

bool qt_core_c_QByteArray_operator_lt(const QByteArray* this_ptr, const QString* s2) {
  return this_ptr->operator<(*s2);
}

bool qt_core_c_QByteArray_operator_neq(const QByteArray* this_ptr, const QString* s2) {
  return this_ptr->operator!=(*s2);
}

QByteArray* qt_core_c_QByteArray_prepend_a(QByteArray* this_ptr, const QByteArray* a) {
  return &this_ptr->prepend(*a);
}

QByteArray* qt_core_c_QByteArray_prepend_c(QByteArray* this_ptr, char c) {
  return &this_ptr->prepend(c);
}

QByteArray* qt_core_c_QByteArray_prepend_count_c(QByteArray* this_ptr, int count, char c) {
  return &this_ptr->prepend(count, c);
}

QByteArray* qt_core_c_QByteArray_prepend_s(QByteArray* this_ptr, const char* s) {
  return &this_ptr->prepend(s);
}

QByteArray* qt_core_c_QByteArray_prepend_s_len(QByteArray* this_ptr, const char* s, int len) {
  return &this_ptr->prepend(s, len);
}

void qt_core_c_QByteArray_push_back_char(QByteArray* this_ptr, char c) {
  this_ptr->push_back(c);
}

void qt_core_c_QByteArray_push_back_const_QByteArray_ref(QByteArray* this_ptr, const QByteArray* a) {
  this_ptr->push_back(*a);
}

void qt_core_c_QByteArray_push_back_const_char_ptr(QByteArray* this_ptr, const char* c) {
  this_ptr->push_back(c);
}

void qt_core_c_QByteArray_push_front_char(QByteArray* this_ptr, char c) {
  this_ptr->push_front(c);
}

void qt_core_c_QByteArray_push_front_const_QByteArray_ref(QByteArray* this_ptr, const QByteArray* a) {
  this_ptr->push_front(*a);
}

void qt_core_c_QByteArray_push_front_const_char_ptr(QByteArray* this_ptr, const char* c) {
  this_ptr->push_front(c);
}

QByteArray* qt_core_c_QByteArray_remove(QByteArray* this_ptr, int index, int len) {
  return &this_ptr->remove(index, len);
}

void qt_core_c_QByteArray_repeated_to_output(const QByteArray* this_ptr, int times, QByteArray* output) {
  new(output) QByteArray(this_ptr->repeated(times));
}

QByteArray* qt_core_c_QByteArray_replace_char_char(QByteArray* this_ptr, char before, char after) {
  return &this_ptr->replace(before, after);
}

QByteArray* qt_core_c_QByteArray_replace_char_const_QByteArray_ref(QByteArray* this_ptr, char before, const QByteArray* after) {
  return &this_ptr->replace(before, *after);
}

QByteArray* qt_core_c_QByteArray_replace_char_const_QString_ref(QByteArray* this_ptr, char c, const QString* after) {
  return &this_ptr->replace(c, *after);
}

QByteArray* qt_core_c_QByteArray_replace_char_const_char_ptr(QByteArray* this_ptr, char before, const char* after) {
  return &this_ptr->replace(before, after);
}

QByteArray* qt_core_c_QByteArray_replace_const_QByteArray_ref_const_QByteArray_ref(QByteArray* this_ptr, const QByteArray* before, const QByteArray* after) {
  return &this_ptr->replace(*before, *after);
}

QByteArray* qt_core_c_QByteArray_replace_const_QByteArray_ref_const_char_ptr(QByteArray* this_ptr, const QByteArray* before, const char* after) {
  return &this_ptr->replace(*before, after);
}

QByteArray* qt_core_c_QByteArray_replace_const_QString_ref_const_QByteArray_ref(QByteArray* this_ptr, const QString* before, const QByteArray* after) {
  return &this_ptr->replace(*before, *after);
}

QByteArray* qt_core_c_QByteArray_replace_const_QString_ref_const_char_ptr(QByteArray* this_ptr, const QString* before, const char* after) {
  return &this_ptr->replace(*before, after);
}

QByteArray* qt_core_c_QByteArray_replace_const_char_ptr_const_QByteArray_ref(QByteArray* this_ptr, const char* before, const QByteArray* after) {
  return &this_ptr->replace(before, *after);
}

QByteArray* qt_core_c_QByteArray_replace_const_char_ptr_const_char_ptr(QByteArray* this_ptr, const char* before, const char* after) {
  return &this_ptr->replace(before, after);
}

QByteArray* qt_core_c_QByteArray_replace_const_char_ptr_int_const_char_ptr_int(QByteArray* this_ptr, const char* before, int bsize, const char* after, int asize) {
  return &this_ptr->replace(before, bsize, after, asize);
}

QByteArray* qt_core_c_QByteArray_replace_int_int_const_QByteArray_ref(QByteArray* this_ptr, int index, int len, const QByteArray* s) {
  return &this_ptr->replace(index, len, *s);
}

QByteArray* qt_core_c_QByteArray_replace_int_int_const_char_ptr(QByteArray* this_ptr, int index, int len, const char* s) {
  return &this_ptr->replace(index, len, s);
}

QByteArray* qt_core_c_QByteArray_replace_int_int_const_char_ptr_int(QByteArray* this_ptr, int index, int len, const char* s, int alen) {
  return &this_ptr->replace(index, len, s, alen);
}

void qt_core_c_QByteArray_reserve(QByteArray* this_ptr, int size) {
  this_ptr->reserve(size);
}

void qt_core_c_QByteArray_resize(QByteArray* this_ptr, int size) {
  this_ptr->resize(size);
}

void qt_core_c_QByteArray_rightJustified_to_output_width(const QByteArray* this_ptr, int width, QByteArray* output) {
  new(output) QByteArray(this_ptr->rightJustified(width));
}

void qt_core_c_QByteArray_rightJustified_to_output_width_fill(const QByteArray* this_ptr, int width, char fill, QByteArray* output) {
  new(output) QByteArray(this_ptr->rightJustified(width, fill));
}

void qt_core_c_QByteArray_rightJustified_to_output_width_fill_truncate(const QByteArray* this_ptr, int width, char fill, bool truncate, QByteArray* output) {
  new(output) QByteArray(this_ptr->rightJustified(width, fill, truncate));
}

void qt_core_c_QByteArray_right_to_output(const QByteArray* this_ptr, int len, QByteArray* output) {
  new(output) QByteArray(this_ptr->right(len));
}

QByteArray* qt_core_c_QByteArray_setNum_double(QByteArray* this_ptr, double arg1) {
  return &this_ptr->setNum(arg1);
}

QByteArray* qt_core_c_QByteArray_setNum_double_char(QByteArray* this_ptr, double arg1, char f) {
  return &this_ptr->setNum(arg1, f);
}

QByteArray* qt_core_c_QByteArray_setNum_double_char_int(QByteArray* this_ptr, double arg1, char f, int prec) {
  return &this_ptr->setNum(arg1, f, prec);
}

QByteArray* qt_core_c_QByteArray_setNum_float(QByteArray* this_ptr, float arg1) {
  return &this_ptr->setNum(arg1);
}

QByteArray* qt_core_c_QByteArray_setNum_float_char(QByteArray* this_ptr, float arg1, char f) {
  return &this_ptr->setNum(arg1, f);
}

QByteArray* qt_core_c_QByteArray_setNum_float_char_int(QByteArray* this_ptr, float arg1, char f, int prec) {
  return &this_ptr->setNum(arg1, f, prec);
}

QByteArray* qt_core_c_QByteArray_setNum_int(QByteArray* this_ptr, int arg1) {
  return &this_ptr->setNum(arg1);
}

QByteArray* qt_core_c_QByteArray_setNum_int_int(QByteArray* this_ptr, int arg1, int base) {
  return &this_ptr->setNum(arg1, base);
}

QByteArray* qt_core_c_QByteArray_setNum_qlonglong(QByteArray* this_ptr, qlonglong arg1) {
  return &this_ptr->setNum(arg1);
}

QByteArray* qt_core_c_QByteArray_setNum_qlonglong_int(QByteArray* this_ptr, qlonglong arg1, int base) {
  return &this_ptr->setNum(arg1, base);
}

QByteArray* qt_core_c_QByteArray_setNum_qulonglong(QByteArray* this_ptr, qulonglong arg1) {
  return &this_ptr->setNum(arg1);
}

QByteArray* qt_core_c_QByteArray_setNum_qulonglong_int(QByteArray* this_ptr, qulonglong arg1, int base) {
  return &this_ptr->setNum(arg1, base);
}

QByteArray* qt_core_c_QByteArray_setNum_short(QByteArray* this_ptr, short arg1) {
  return &this_ptr->setNum(arg1);
}

QByteArray* qt_core_c_QByteArray_setNum_short_int(QByteArray* this_ptr, short arg1, int base) {
  return &this_ptr->setNum(arg1, base);
}

QByteArray* qt_core_c_QByteArray_setNum_unsigned_int(QByteArray* this_ptr, unsigned int arg1) {
  return &this_ptr->setNum(arg1);
}

QByteArray* qt_core_c_QByteArray_setNum_unsigned_int_int(QByteArray* this_ptr, unsigned int arg1, int base) {
  return &this_ptr->setNum(arg1, base);
}

QByteArray* qt_core_c_QByteArray_setNum_unsigned_short(QByteArray* this_ptr, unsigned short arg1) {
  return &this_ptr->setNum(arg1);
}

QByteArray* qt_core_c_QByteArray_setNum_unsigned_short_int(QByteArray* this_ptr, unsigned short arg1, int base) {
  return &this_ptr->setNum(arg1, base);
}

QByteArray* qt_core_c_QByteArray_setRawData(QByteArray* this_ptr, const char* a, unsigned int n) {
  return &this_ptr->setRawData(a, n);
}

void qt_core_c_QByteArray_simplified_to_output(QByteArray* this_ptr, QByteArray* output) {
  new(output) QByteArray(this_ptr->simplified());
}

void qt_core_c_QByteArray_simplified_to_output_const(const QByteArray* this_ptr, QByteArray* output) {
  new(output) QByteArray(this_ptr->simplified());
}

int qt_core_c_QByteArray_size(const QByteArray* this_ptr) {
  return this_ptr->size();
}

void qt_core_c_QByteArray_split_to_output(const QByteArray* this_ptr, char sep, QList< QByteArray >* output) {
  new(output) QList< QByteArray >(this_ptr->split(sep));
}

void qt_core_c_QByteArray_squeeze(QByteArray* this_ptr) {
  this_ptr->squeeze();
}

bool qt_core_c_QByteArray_startsWith_char(const QByteArray* this_ptr, char c) {
  return this_ptr->startsWith(c);
}

bool qt_core_c_QByteArray_startsWith_const_QByteArray_ref(const QByteArray* this_ptr, const QByteArray* a) {
  return this_ptr->startsWith(*a);
}

bool qt_core_c_QByteArray_startsWith_const_char_ptr(const QByteArray* this_ptr, const char* c) {
  return this_ptr->startsWith(c);
}

void qt_core_c_QByteArray_swap(QByteArray* this_ptr, QByteArray* other) {
  this_ptr->swap(*other);
}

void qt_core_c_QByteArray_toBase64_to_output_no_args(const QByteArray* this_ptr, QByteArray* output) {
  new(output) QByteArray(this_ptr->toBase64());
}

void qt_core_c_QByteArray_toBase64_to_output_options(const QByteArray* this_ptr, unsigned int options, QByteArray* output) {
  new(output) QByteArray(this_ptr->toBase64(QFlags< QByteArray::Base64Option >(options)));
}

double qt_core_c_QByteArray_toDouble_no_args(const QByteArray* this_ptr) {
  return this_ptr->toDouble();
}

double qt_core_c_QByteArray_toDouble_ok(const QByteArray* this_ptr, bool* ok) {
  return this_ptr->toDouble(ok);
}

float qt_core_c_QByteArray_toFloat_no_args(const QByteArray* this_ptr) {
  return this_ptr->toFloat();
}

float qt_core_c_QByteArray_toFloat_ok(const QByteArray* this_ptr, bool* ok) {
  return this_ptr->toFloat(ok);
}

void qt_core_c_QByteArray_toHex_to_output_no_args(const QByteArray* this_ptr, QByteArray* output) {
  new(output) QByteArray(this_ptr->toHex());
}

void qt_core_c_QByteArray_toHex_to_output_separator(const QByteArray* this_ptr, char separator, QByteArray* output) {
  new(output) QByteArray(this_ptr->toHex(separator));
}

int qt_core_c_QByteArray_toInt_no_args(const QByteArray* this_ptr) {
  return this_ptr->toInt();
}

int qt_core_c_QByteArray_toInt_ok(const QByteArray* this_ptr, bool* ok) {
  return this_ptr->toInt(ok);
}

int qt_core_c_QByteArray_toInt_ok_base(const QByteArray* this_ptr, bool* ok, int base) {
  return this_ptr->toInt(ok, base);
}

qlonglong qt_core_c_QByteArray_toLongLong_no_args(const QByteArray* this_ptr) {
  return this_ptr->toLongLong();
}

qlonglong qt_core_c_QByteArray_toLongLong_ok(const QByteArray* this_ptr, bool* ok) {
  return this_ptr->toLongLong(ok);
}

qlonglong qt_core_c_QByteArray_toLongLong_ok_base(const QByteArray* this_ptr, bool* ok, int base) {
  return this_ptr->toLongLong(ok, base);
}

long qt_core_c_QByteArray_toLong_no_args(const QByteArray* this_ptr) {
  return this_ptr->toLong();
}

long qt_core_c_QByteArray_toLong_ok(const QByteArray* this_ptr, bool* ok) {
  return this_ptr->toLong(ok);
}

long qt_core_c_QByteArray_toLong_ok_base(const QByteArray* this_ptr, bool* ok, int base) {
  return this_ptr->toLong(ok, base);
}

void qt_core_c_QByteArray_toLower_to_output(QByteArray* this_ptr, QByteArray* output) {
  new(output) QByteArray(this_ptr->toLower());
}

void qt_core_c_QByteArray_toLower_to_output_const(const QByteArray* this_ptr, QByteArray* output) {
  new(output) QByteArray(this_ptr->toLower());
}

void qt_core_c_QByteArray_toPercentEncoding_to_output_exclude(const QByteArray* this_ptr, const QByteArray* exclude, QByteArray* output) {
  new(output) QByteArray(this_ptr->toPercentEncoding(*exclude));
}

void qt_core_c_QByteArray_toPercentEncoding_to_output_exclude_include(const QByteArray* this_ptr, const QByteArray* exclude, const QByteArray* include, QByteArray* output) {
  new(output) QByteArray(this_ptr->toPercentEncoding(*exclude, *include));
}

void qt_core_c_QByteArray_toPercentEncoding_to_output_exclude_include_percent(const QByteArray* this_ptr, const QByteArray* exclude, const QByteArray* include, char percent, QByteArray* output) {
  new(output) QByteArray(this_ptr->toPercentEncoding(*exclude, *include, percent));
}

void qt_core_c_QByteArray_toPercentEncoding_to_output_no_args(const QByteArray* this_ptr, QByteArray* output) {
  new(output) QByteArray(this_ptr->toPercentEncoding());
}

short qt_core_c_QByteArray_toShort_no_args(const QByteArray* this_ptr) {
  return this_ptr->toShort();
}

short qt_core_c_QByteArray_toShort_ok(const QByteArray* this_ptr, bool* ok) {
  return this_ptr->toShort(ok);
}

short qt_core_c_QByteArray_toShort_ok_base(const QByteArray* this_ptr, bool* ok, int base) {
  return this_ptr->toShort(ok, base);
}

unsigned int qt_core_c_QByteArray_toUInt_no_args(const QByteArray* this_ptr) {
  return this_ptr->toUInt();
}

unsigned int qt_core_c_QByteArray_toUInt_ok(const QByteArray* this_ptr, bool* ok) {
  return this_ptr->toUInt(ok);
}

unsigned int qt_core_c_QByteArray_toUInt_ok_base(const QByteArray* this_ptr, bool* ok, int base) {
  return this_ptr->toUInt(ok, base);
}

qulonglong qt_core_c_QByteArray_toULongLong_no_args(const QByteArray* this_ptr) {
  return this_ptr->toULongLong();
}

qulonglong qt_core_c_QByteArray_toULongLong_ok(const QByteArray* this_ptr, bool* ok) {
  return this_ptr->toULongLong(ok);
}

qulonglong qt_core_c_QByteArray_toULongLong_ok_base(const QByteArray* this_ptr, bool* ok, int base) {
  return this_ptr->toULongLong(ok, base);
}

unsigned long qt_core_c_QByteArray_toULong_no_args(const QByteArray* this_ptr) {
  return this_ptr->toULong();
}

unsigned long qt_core_c_QByteArray_toULong_ok(const QByteArray* this_ptr, bool* ok) {
  return this_ptr->toULong(ok);
}

unsigned long qt_core_c_QByteArray_toULong_ok_base(const QByteArray* this_ptr, bool* ok, int base) {
  return this_ptr->toULong(ok, base);
}

unsigned short qt_core_c_QByteArray_toUShort_no_args(const QByteArray* this_ptr) {
  return this_ptr->toUShort();
}

unsigned short qt_core_c_QByteArray_toUShort_ok(const QByteArray* this_ptr, bool* ok) {
  return this_ptr->toUShort(ok);
}

unsigned short qt_core_c_QByteArray_toUShort_ok_base(const QByteArray* this_ptr, bool* ok, int base) {
  return this_ptr->toUShort(ok, base);
}

void qt_core_c_QByteArray_toUpper_to_output(QByteArray* this_ptr, QByteArray* output) {
  new(output) QByteArray(this_ptr->toUpper());
}

void qt_core_c_QByteArray_toUpper_to_output_const(const QByteArray* this_ptr, QByteArray* output) {
  new(output) QByteArray(this_ptr->toUpper());
}

void qt_core_c_QByteArray_trimmed_to_output(QByteArray* this_ptr, QByteArray* output) {
  new(output) QByteArray(this_ptr->trimmed());
}

void qt_core_c_QByteArray_trimmed_to_output_const(const QByteArray* this_ptr, QByteArray* output) {
  new(output) QByteArray(this_ptr->trimmed());
}

void qt_core_c_QByteArray_truncate(QByteArray* this_ptr, int pos) {
  this_ptr->truncate(pos);
}

