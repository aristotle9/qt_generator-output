#include "qt_core_c_QStringRef.h"

void qt_core_c_QStringRef_appendTo_to_output(const QStringRef* this_ptr, QString* string, QStringRef* output) {
  new(output) QStringRef(this_ptr->appendTo(string));
}

void qt_core_c_QStringRef_at_to_output(const QStringRef* this_ptr, int i, QChar* output) {
  new(output) QChar(this_ptr->at(i));
}

const QChar* qt_core_c_QStringRef_begin(const QStringRef* this_ptr) {
  return this_ptr->begin();
}

const QChar* qt_core_c_QStringRef_cbegin(const QStringRef* this_ptr) {
  return this_ptr->cbegin();
}

const QChar* qt_core_c_QStringRef_cend(const QStringRef* this_ptr) {
  return this_ptr->cend();
}

void qt_core_c_QStringRef_chop(QStringRef* this_ptr, int n) {
  this_ptr->chop(n);
}

void qt_core_c_QStringRef_clear(QStringRef* this_ptr) {
  this_ptr->clear();
}

int qt_core_c_QStringRef_compare_QByteArray(const QStringRef* this_ptr, const QByteArray* s) {
  return this_ptr->compare(*s);
}

int qt_core_c_QStringRef_compare_QByteArray_Qt_CaseSensitivity(const QStringRef* this_ptr, const QByteArray* s, const Qt::CaseSensitivity* cs) {
  return this_ptr->compare(*s, *cs);
}

int qt_core_c_QStringRef_compare_QLatin1String(const QStringRef* this_ptr, const QLatin1String* s) {
  return this_ptr->compare(*s);
}

int qt_core_c_QStringRef_compare_QLatin1String_Qt_CaseSensitivity(const QStringRef* this_ptr, const QLatin1String* s, const Qt::CaseSensitivity* cs) {
  return this_ptr->compare(*s, *cs);
}

int qt_core_c_QStringRef_compare_QString(const QStringRef* this_ptr, const QString* s) {
  return this_ptr->compare(*s);
}

int qt_core_c_QStringRef_compare_QStringRef(const QStringRef* this_ptr, const QStringRef* s) {
  return this_ptr->compare(*s);
}

int qt_core_c_QStringRef_compare_QStringRef_QLatin1String(const QStringRef* s1, const QLatin1String* s2) {
  return QStringRef::compare(*s1, *s2);
}

int qt_core_c_QStringRef_compare_QStringRef_QLatin1String_Qt_CaseSensitivity(const QStringRef* s1, const QLatin1String* s2, const Qt::CaseSensitivity* cs) {
  return QStringRef::compare(*s1, *s2, *cs);
}

int qt_core_c_QStringRef_compare_QStringRef_QString(const QStringRef* s1, const QString* s2) {
  return QStringRef::compare(*s1, *s2);
}

int qt_core_c_QStringRef_compare_QStringRef_QStringRef(const QStringRef* s1, const QStringRef* s2) {
  return QStringRef::compare(*s1, *s2);
}

int qt_core_c_QStringRef_compare_QStringRef_QStringRef_Qt_CaseSensitivity(const QStringRef* s1, const QStringRef* s2, const Qt::CaseSensitivity* arg3) {
  return QStringRef::compare(*s1, *s2, *arg3);
}

int qt_core_c_QStringRef_compare_QStringRef_QString_Qt_CaseSensitivity(const QStringRef* s1, const QString* s2, const Qt::CaseSensitivity* arg3) {
  return QStringRef::compare(*s1, *s2, *arg3);
}

int qt_core_c_QStringRef_compare_QStringRef_Qt_CaseSensitivity(const QStringRef* this_ptr, const QStringRef* s, const Qt::CaseSensitivity* cs) {
  return this_ptr->compare(*s, *cs);
}

int qt_core_c_QStringRef_compare_QString_Qt_CaseSensitivity(const QStringRef* this_ptr, const QString* s, const Qt::CaseSensitivity* cs) {
  return this_ptr->compare(*s, *cs);
}

const QChar* qt_core_c_QStringRef_constBegin(const QStringRef* this_ptr) {
  return this_ptr->constBegin();
}

const QChar* qt_core_c_QStringRef_constData(const QStringRef* this_ptr) {
  return this_ptr->constData();
}

const QChar* qt_core_c_QStringRef_constEnd(const QStringRef* this_ptr) {
  return this_ptr->constEnd();
}

void qt_core_c_QStringRef_constructor_no_args(QStringRef* output) {
  new(output) QStringRef();
}

void qt_core_c_QStringRef_constructor_other(const QStringRef* other, QStringRef* output) {
  new(output) QStringRef(*other);
}

void qt_core_c_QStringRef_constructor_string(const QString* string, QStringRef* output) {
  new(output) QStringRef(string);
}

void qt_core_c_QStringRef_constructor_string_position_size(const QString* string, int position, int size, QStringRef* output) {
  new(output) QStringRef(string, position, size);
}

bool qt_core_c_QStringRef_contains_QChar(const QStringRef* this_ptr, const QChar* ch) {
  return this_ptr->contains(*ch);
}

bool qt_core_c_QStringRef_contains_QChar_Qt_CaseSensitivity(const QStringRef* this_ptr, const QChar* ch, const Qt::CaseSensitivity* cs) {
  return this_ptr->contains(*ch, *cs);
}

bool qt_core_c_QStringRef_contains_QLatin1String(const QStringRef* this_ptr, const QLatin1String* str) {
  return this_ptr->contains(*str);
}

bool qt_core_c_QStringRef_contains_QLatin1String_Qt_CaseSensitivity(const QStringRef* this_ptr, const QLatin1String* str, const Qt::CaseSensitivity* cs) {
  return this_ptr->contains(*str, *cs);
}

bool qt_core_c_QStringRef_contains_QString(const QStringRef* this_ptr, const QString* str) {
  return this_ptr->contains(*str);
}

bool qt_core_c_QStringRef_contains_QStringRef(const QStringRef* this_ptr, const QStringRef* str) {
  return this_ptr->contains(*str);
}

bool qt_core_c_QStringRef_contains_QStringRef_Qt_CaseSensitivity(const QStringRef* this_ptr, const QStringRef* str, const Qt::CaseSensitivity* cs) {
  return this_ptr->contains(*str, *cs);
}

bool qt_core_c_QStringRef_contains_QString_Qt_CaseSensitivity(const QStringRef* this_ptr, const QString* str, const Qt::CaseSensitivity* cs) {
  return this_ptr->contains(*str, *cs);
}

int qt_core_c_QStringRef_count_QChar(const QStringRef* this_ptr, const QChar* c) {
  return this_ptr->count(*c);
}

int qt_core_c_QStringRef_count_QChar_Qt_CaseSensitivity(const QStringRef* this_ptr, const QChar* c, const Qt::CaseSensitivity* cs) {
  return this_ptr->count(*c, *cs);
}

int qt_core_c_QStringRef_count_QString(const QStringRef* this_ptr, const QString* s) {
  return this_ptr->count(*s);
}

int qt_core_c_QStringRef_count_QStringRef(const QStringRef* this_ptr, const QStringRef* s) {
  return this_ptr->count(*s);
}

int qt_core_c_QStringRef_count_QStringRef_Qt_CaseSensitivity(const QStringRef* this_ptr, const QStringRef* s, const Qt::CaseSensitivity* cs) {
  return this_ptr->count(*s, *cs);
}

int qt_core_c_QStringRef_count_QString_Qt_CaseSensitivity(const QStringRef* this_ptr, const QString* s, const Qt::CaseSensitivity* cs) {
  return this_ptr->count(*s, *cs);
}

int qt_core_c_QStringRef_count_no_args(const QStringRef* this_ptr) {
  return this_ptr->count();
}

const QChar* qt_core_c_QStringRef_data(const QStringRef* this_ptr) {
  return this_ptr->data();
}

void qt_core_c_QStringRef_destructor(QStringRef* this_ptr) {
  qt_core_c_call_destructor(this_ptr);
}

const QChar* qt_core_c_QStringRef_end(const QStringRef* this_ptr) {
  return this_ptr->end();
}

bool qt_core_c_QStringRef_endsWith_QChar(const QStringRef* this_ptr, const QChar* c) {
  return this_ptr->endsWith(*c);
}

bool qt_core_c_QStringRef_endsWith_QChar_Qt_CaseSensitivity(const QStringRef* this_ptr, const QChar* c, const Qt::CaseSensitivity* cs) {
  return this_ptr->endsWith(*c, *cs);
}

bool qt_core_c_QStringRef_endsWith_QLatin1String(const QStringRef* this_ptr, const QLatin1String* s) {
  return this_ptr->endsWith(*s);
}

bool qt_core_c_QStringRef_endsWith_QLatin1String_Qt_CaseSensitivity(const QStringRef* this_ptr, const QLatin1String* s, const Qt::CaseSensitivity* cs) {
  return this_ptr->endsWith(*s, *cs);
}

bool qt_core_c_QStringRef_endsWith_QString(const QStringRef* this_ptr, const QString* s) {
  return this_ptr->endsWith(*s);
}

bool qt_core_c_QStringRef_endsWith_QStringRef(const QStringRef* this_ptr, const QStringRef* c) {
  return this_ptr->endsWith(*c);
}

bool qt_core_c_QStringRef_endsWith_QStringRef_Qt_CaseSensitivity(const QStringRef* this_ptr, const QStringRef* c, const Qt::CaseSensitivity* cs) {
  return this_ptr->endsWith(*c, *cs);
}

bool qt_core_c_QStringRef_endsWith_QString_Qt_CaseSensitivity(const QStringRef* this_ptr, const QString* s, const Qt::CaseSensitivity* cs) {
  return this_ptr->endsWith(*s, *cs);
}

int qt_core_c_QStringRef_indexOf_QChar(const QStringRef* this_ptr, const QChar* ch) {
  return this_ptr->indexOf(*ch);
}

int qt_core_c_QStringRef_indexOf_QChar_int(const QStringRef* this_ptr, const QChar* ch, int from) {
  return this_ptr->indexOf(*ch, from);
}

int qt_core_c_QStringRef_indexOf_QChar_int_Qt_CaseSensitivity(const QStringRef* this_ptr, const QChar* ch, int from, const Qt::CaseSensitivity* cs) {
  return this_ptr->indexOf(*ch, from, *cs);
}

int qt_core_c_QStringRef_indexOf_QLatin1String(const QStringRef* this_ptr, const QLatin1String* str) {
  return this_ptr->indexOf(*str);
}

int qt_core_c_QStringRef_indexOf_QLatin1String_int(const QStringRef* this_ptr, const QLatin1String* str, int from) {
  return this_ptr->indexOf(*str, from);
}

int qt_core_c_QStringRef_indexOf_QLatin1String_int_Qt_CaseSensitivity(const QStringRef* this_ptr, const QLatin1String* str, int from, const Qt::CaseSensitivity* cs) {
  return this_ptr->indexOf(*str, from, *cs);
}

int qt_core_c_QStringRef_indexOf_QString(const QStringRef* this_ptr, const QString* str) {
  return this_ptr->indexOf(*str);
}

int qt_core_c_QStringRef_indexOf_QStringRef(const QStringRef* this_ptr, const QStringRef* str) {
  return this_ptr->indexOf(*str);
}

int qt_core_c_QStringRef_indexOf_QStringRef_int(const QStringRef* this_ptr, const QStringRef* str, int from) {
  return this_ptr->indexOf(*str, from);
}

int qt_core_c_QStringRef_indexOf_QStringRef_int_Qt_CaseSensitivity(const QStringRef* this_ptr, const QStringRef* str, int from, const Qt::CaseSensitivity* cs) {
  return this_ptr->indexOf(*str, from, *cs);
}

int qt_core_c_QStringRef_indexOf_QString_int(const QStringRef* this_ptr, const QString* str, int from) {
  return this_ptr->indexOf(*str, from);
}

int qt_core_c_QStringRef_indexOf_QString_int_Qt_CaseSensitivity(const QStringRef* this_ptr, const QString* str, int from, const Qt::CaseSensitivity* cs) {
  return this_ptr->indexOf(*str, from, *cs);
}

bool qt_core_c_QStringRef_isEmpty(const QStringRef* this_ptr) {
  return this_ptr->isEmpty();
}

bool qt_core_c_QStringRef_isNull(const QStringRef* this_ptr) {
  return this_ptr->isNull();
}

bool qt_core_c_QStringRef_isRightToLeft(const QStringRef* this_ptr) {
  return this_ptr->isRightToLeft();
}

int qt_core_c_QStringRef_lastIndexOf_QChar(const QStringRef* this_ptr, const QChar* ch) {
  return this_ptr->lastIndexOf(*ch);
}

int qt_core_c_QStringRef_lastIndexOf_QChar_int(const QStringRef* this_ptr, const QChar* ch, int from) {
  return this_ptr->lastIndexOf(*ch, from);
}

int qt_core_c_QStringRef_lastIndexOf_QChar_int_Qt_CaseSensitivity(const QStringRef* this_ptr, const QChar* ch, int from, const Qt::CaseSensitivity* cs) {
  return this_ptr->lastIndexOf(*ch, from, *cs);
}

int qt_core_c_QStringRef_lastIndexOf_QLatin1String(const QStringRef* this_ptr, const QLatin1String* str) {
  return this_ptr->lastIndexOf(*str);
}

int qt_core_c_QStringRef_lastIndexOf_QLatin1String_int(const QStringRef* this_ptr, const QLatin1String* str, int from) {
  return this_ptr->lastIndexOf(*str, from);
}

int qt_core_c_QStringRef_lastIndexOf_QLatin1String_int_Qt_CaseSensitivity(const QStringRef* this_ptr, const QLatin1String* str, int from, const Qt::CaseSensitivity* cs) {
  return this_ptr->lastIndexOf(*str, from, *cs);
}

int qt_core_c_QStringRef_lastIndexOf_QString(const QStringRef* this_ptr, const QString* str) {
  return this_ptr->lastIndexOf(*str);
}

int qt_core_c_QStringRef_lastIndexOf_QStringRef(const QStringRef* this_ptr, const QStringRef* str) {
  return this_ptr->lastIndexOf(*str);
}

int qt_core_c_QStringRef_lastIndexOf_QStringRef_int(const QStringRef* this_ptr, const QStringRef* str, int from) {
  return this_ptr->lastIndexOf(*str, from);
}

int qt_core_c_QStringRef_lastIndexOf_QStringRef_int_Qt_CaseSensitivity(const QStringRef* this_ptr, const QStringRef* str, int from, const Qt::CaseSensitivity* cs) {
  return this_ptr->lastIndexOf(*str, from, *cs);
}

int qt_core_c_QStringRef_lastIndexOf_QString_int(const QStringRef* this_ptr, const QString* str, int from) {
  return this_ptr->lastIndexOf(*str, from);
}

int qt_core_c_QStringRef_lastIndexOf_QString_int_Qt_CaseSensitivity(const QStringRef* this_ptr, const QString* str, int from, const Qt::CaseSensitivity* cs) {
  return this_ptr->lastIndexOf(*str, from, *cs);
}

void qt_core_c_QStringRef_left_to_output(const QStringRef* this_ptr, int n, QStringRef* output) {
  new(output) QStringRef(this_ptr->left(n));
}

int qt_core_c_QStringRef_length(const QStringRef* this_ptr) {
  return this_ptr->length();
}

int qt_core_c_QStringRef_localeAwareCompare_QString(const QStringRef* this_ptr, const QString* s) {
  return this_ptr->localeAwareCompare(*s);
}

int qt_core_c_QStringRef_localeAwareCompare_QStringRef(const QStringRef* this_ptr, const QStringRef* s) {
  return this_ptr->localeAwareCompare(*s);
}

int qt_core_c_QStringRef_localeAwareCompare_QStringRef_QString(const QStringRef* s1, const QString* s2) {
  return QStringRef::localeAwareCompare(*s1, *s2);
}

int qt_core_c_QStringRef_localeAwareCompare_QStringRef_QStringRef(const QStringRef* s1, const QStringRef* s2) {
  return QStringRef::localeAwareCompare(*s1, *s2);
}

void qt_core_c_QStringRef_mid_to_output_pos(const QStringRef* this_ptr, int pos, QStringRef* output) {
  new(output) QStringRef(this_ptr->mid(pos));
}

void qt_core_c_QStringRef_mid_to_output_pos_n(const QStringRef* this_ptr, int pos, int n, QStringRef* output) {
  new(output) QStringRef(this_ptr->mid(pos, n));
}

QStringRef* qt_core_c_QStringRef_operator_assign_other(QStringRef* this_ptr, const QStringRef* other) {
  return &this_ptr->operator=(*other);
}

QStringRef* qt_core_c_QStringRef_operator_assign_string(QStringRef* this_ptr, const QString* string) {
  return &this_ptr->operator=(string);
}

bool qt_core_c_QStringRef_operator_eq(const QStringRef* this_ptr, const char* s) {
  return this_ptr->operator==(s);
}

bool qt_core_c_QStringRef_operator_ge(const QStringRef* this_ptr, const char* s) {
  return this_ptr->operator>=(s);
}

bool qt_core_c_QStringRef_operator_gt(const QStringRef* this_ptr, const char* s) {
  return this_ptr->operator>(s);
}

void qt_core_c_QStringRef_operator_index_to_output(const QStringRef* this_ptr, int i, QChar* output) {
  new(output) QChar(this_ptr->operator[](i));
}

bool qt_core_c_QStringRef_operator_le(const QStringRef* this_ptr, const char* s) {
  return this_ptr->operator<=(s);
}

bool qt_core_c_QStringRef_operator_lt(const QStringRef* this_ptr, const char* s) {
  return this_ptr->operator<(s);
}

bool qt_core_c_QStringRef_operator_neq(const QStringRef* this_ptr, const char* s) {
  return this_ptr->operator!=(s);
}

int qt_core_c_QStringRef_position(const QStringRef* this_ptr) {
  return this_ptr->position();
}

void qt_core_c_QStringRef_right_to_output(const QStringRef* this_ptr, int n, QStringRef* output) {
  new(output) QStringRef(this_ptr->right(n));
}

int qt_core_c_QStringRef_size(const QStringRef* this_ptr) {
  return this_ptr->size();
}

void qt_core_c_QStringRef_split_to_output_QChar(const QStringRef* this_ptr, const QChar* sep, QVector< QStringRef >* output) {
  new(output) QVector< QStringRef >(this_ptr->split(*sep));
}

void qt_core_c_QStringRef_split_to_output_QChar_QString_SplitBehavior(const QStringRef* this_ptr, const QChar* sep, const QString::SplitBehavior* behavior, QVector< QStringRef >* output) {
  new(output) QVector< QStringRef >(this_ptr->split(*sep, *behavior));
}

void qt_core_c_QStringRef_split_to_output_QChar_QString_SplitBehavior_Qt_CaseSensitivity(const QStringRef* this_ptr, const QChar* sep, const QString::SplitBehavior* behavior, const Qt::CaseSensitivity* cs, QVector< QStringRef >* output) {
  new(output) QVector< QStringRef >(this_ptr->split(*sep, *behavior, *cs));
}

void qt_core_c_QStringRef_split_to_output_QString(const QStringRef* this_ptr, const QString* sep, QVector< QStringRef >* output) {
  new(output) QVector< QStringRef >(this_ptr->split(*sep));
}

void qt_core_c_QStringRef_split_to_output_QString_QString_SplitBehavior(const QStringRef* this_ptr, const QString* sep, const QString::SplitBehavior* behavior, QVector< QStringRef >* output) {
  new(output) QVector< QStringRef >(this_ptr->split(*sep, *behavior));
}

void qt_core_c_QStringRef_split_to_output_QString_QString_SplitBehavior_Qt_CaseSensitivity(const QStringRef* this_ptr, const QString* sep, const QString::SplitBehavior* behavior, const Qt::CaseSensitivity* cs, QVector< QStringRef >* output) {
  new(output) QVector< QStringRef >(this_ptr->split(*sep, *behavior, *cs));
}

bool qt_core_c_QStringRef_startsWith_QChar(const QStringRef* this_ptr, const QChar* c) {
  return this_ptr->startsWith(*c);
}

bool qt_core_c_QStringRef_startsWith_QChar_Qt_CaseSensitivity(const QStringRef* this_ptr, const QChar* c, const Qt::CaseSensitivity* cs) {
  return this_ptr->startsWith(*c, *cs);
}

bool qt_core_c_QStringRef_startsWith_QLatin1String(const QStringRef* this_ptr, const QLatin1String* s) {
  return this_ptr->startsWith(*s);
}

bool qt_core_c_QStringRef_startsWith_QLatin1String_Qt_CaseSensitivity(const QStringRef* this_ptr, const QLatin1String* s, const Qt::CaseSensitivity* cs) {
  return this_ptr->startsWith(*s, *cs);
}

bool qt_core_c_QStringRef_startsWith_QString(const QStringRef* this_ptr, const QString* s) {
  return this_ptr->startsWith(*s);
}

bool qt_core_c_QStringRef_startsWith_QStringRef(const QStringRef* this_ptr, const QStringRef* c) {
  return this_ptr->startsWith(*c);
}

bool qt_core_c_QStringRef_startsWith_QStringRef_Qt_CaseSensitivity(const QStringRef* this_ptr, const QStringRef* c, const Qt::CaseSensitivity* cs) {
  return this_ptr->startsWith(*c, *cs);
}

bool qt_core_c_QStringRef_startsWith_QString_Qt_CaseSensitivity(const QStringRef* this_ptr, const QString* s, const Qt::CaseSensitivity* cs) {
  return this_ptr->startsWith(*s, *cs);
}

const QString* qt_core_c_QStringRef_string(const QStringRef* this_ptr) {
  return this_ptr->string();
}

double qt_core_c_QStringRef_toDouble_no_args(const QStringRef* this_ptr) {
  return this_ptr->toDouble();
}

double qt_core_c_QStringRef_toDouble_ok(const QStringRef* this_ptr, bool* ok) {
  return this_ptr->toDouble(ok);
}

float qt_core_c_QStringRef_toFloat_no_args(const QStringRef* this_ptr) {
  return this_ptr->toFloat();
}

float qt_core_c_QStringRef_toFloat_ok(const QStringRef* this_ptr, bool* ok) {
  return this_ptr->toFloat(ok);
}

int qt_core_c_QStringRef_toInt_no_args(const QStringRef* this_ptr) {
  return this_ptr->toInt();
}

int qt_core_c_QStringRef_toInt_ok(const QStringRef* this_ptr, bool* ok) {
  return this_ptr->toInt(ok);
}

int qt_core_c_QStringRef_toInt_ok_base(const QStringRef* this_ptr, bool* ok, int base) {
  return this_ptr->toInt(ok, base);
}

void qt_core_c_QStringRef_toLatin1_to_output(const QStringRef* this_ptr, QByteArray* output) {
  new(output) QByteArray(this_ptr->toLatin1());
}

void qt_core_c_QStringRef_toLocal8Bit_to_output(const QStringRef* this_ptr, QByteArray* output) {
  new(output) QByteArray(this_ptr->toLocal8Bit());
}

qlonglong qt_core_c_QStringRef_toLongLong_no_args(const QStringRef* this_ptr) {
  return this_ptr->toLongLong();
}

qlonglong qt_core_c_QStringRef_toLongLong_ok(const QStringRef* this_ptr, bool* ok) {
  return this_ptr->toLongLong(ok);
}

qlonglong qt_core_c_QStringRef_toLongLong_ok_base(const QStringRef* this_ptr, bool* ok, int base) {
  return this_ptr->toLongLong(ok, base);
}

long qt_core_c_QStringRef_toLong_no_args(const QStringRef* this_ptr) {
  return this_ptr->toLong();
}

long qt_core_c_QStringRef_toLong_ok(const QStringRef* this_ptr, bool* ok) {
  return this_ptr->toLong(ok);
}

long qt_core_c_QStringRef_toLong_ok_base(const QStringRef* this_ptr, bool* ok, int base) {
  return this_ptr->toLong(ok, base);
}

short qt_core_c_QStringRef_toShort_no_args(const QStringRef* this_ptr) {
  return this_ptr->toShort();
}

short qt_core_c_QStringRef_toShort_ok(const QStringRef* this_ptr, bool* ok) {
  return this_ptr->toShort(ok);
}

short qt_core_c_QStringRef_toShort_ok_base(const QStringRef* this_ptr, bool* ok, int base) {
  return this_ptr->toShort(ok, base);
}

void qt_core_c_QStringRef_toString_to_output(const QStringRef* this_ptr, QString* output) {
  new(output) QString(this_ptr->toString());
}

unsigned int qt_core_c_QStringRef_toUInt_no_args(const QStringRef* this_ptr) {
  return this_ptr->toUInt();
}

unsigned int qt_core_c_QStringRef_toUInt_ok(const QStringRef* this_ptr, bool* ok) {
  return this_ptr->toUInt(ok);
}

unsigned int qt_core_c_QStringRef_toUInt_ok_base(const QStringRef* this_ptr, bool* ok, int base) {
  return this_ptr->toUInt(ok, base);
}

qulonglong qt_core_c_QStringRef_toULongLong_no_args(const QStringRef* this_ptr) {
  return this_ptr->toULongLong();
}

qulonglong qt_core_c_QStringRef_toULongLong_ok(const QStringRef* this_ptr, bool* ok) {
  return this_ptr->toULongLong(ok);
}

qulonglong qt_core_c_QStringRef_toULongLong_ok_base(const QStringRef* this_ptr, bool* ok, int base) {
  return this_ptr->toULongLong(ok, base);
}

unsigned long qt_core_c_QStringRef_toULong_no_args(const QStringRef* this_ptr) {
  return this_ptr->toULong();
}

unsigned long qt_core_c_QStringRef_toULong_ok(const QStringRef* this_ptr, bool* ok) {
  return this_ptr->toULong(ok);
}

unsigned long qt_core_c_QStringRef_toULong_ok_base(const QStringRef* this_ptr, bool* ok, int base) {
  return this_ptr->toULong(ok, base);
}

unsigned short qt_core_c_QStringRef_toUShort_no_args(const QStringRef* this_ptr) {
  return this_ptr->toUShort();
}

unsigned short qt_core_c_QStringRef_toUShort_ok(const QStringRef* this_ptr, bool* ok) {
  return this_ptr->toUShort(ok);
}

unsigned short qt_core_c_QStringRef_toUShort_ok_base(const QStringRef* this_ptr, bool* ok, int base) {
  return this_ptr->toUShort(ok, base);
}

void qt_core_c_QStringRef_toUcs4_to_output(const QStringRef* this_ptr, QVector< unsigned int >* output) {
  new(output) QVector< unsigned int >(this_ptr->toUcs4());
}

void qt_core_c_QStringRef_toUtf8_to_output(const QStringRef* this_ptr, QByteArray* output) {
  new(output) QByteArray(this_ptr->toUtf8());
}

void qt_core_c_QStringRef_trimmed_to_output(const QStringRef* this_ptr, QStringRef* output) {
  new(output) QStringRef(this_ptr->trimmed());
}

void qt_core_c_QStringRef_truncate(QStringRef* this_ptr, int pos) {
  this_ptr->truncate(pos);
}

const QChar* qt_core_c_QStringRef_unicode(const QStringRef* this_ptr) {
  return this_ptr->unicode();
}

