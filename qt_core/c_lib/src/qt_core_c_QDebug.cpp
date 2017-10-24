#include "qt_core_c_QDebug.h"

void qt_core_c_QDebug_G_swap(QDebug* value1, QDebug* value2) {
  swap(*value1, *value2);
}

bool qt_core_c_QDebug_autoInsertSpaces(const QDebug* this_ptr) {
  return this_ptr->autoInsertSpaces();
}

void qt_core_c_QDebug_constructor_device(QIODevice* device, QDebug* output) {
  new(output) QDebug(device);
}

void qt_core_c_QDebug_constructor_o(const QDebug* o, QDebug* output) {
  new(output) QDebug(*o);
}

void qt_core_c_QDebug_constructor_string(QString* string, QDebug* output) {
  new(output) QDebug(string);
}

void qt_core_c_QDebug_constructor_t(QtMsgType t, QDebug* output) {
  new(output) QDebug(t);
}

void qt_core_c_QDebug_destructor(QDebug* this_ptr) {
  qt_core_c_call_destructor(this_ptr);
}

QDebug* qt_core_c_QDebug_maybeQuote_c(QDebug* this_ptr, char c) {
  return &this_ptr->maybeQuote(c);
}

QDebug* qt_core_c_QDebug_maybeQuote_no_args(QDebug* this_ptr) {
  return &this_ptr->maybeQuote();
}

QDebug* qt_core_c_QDebug_maybeSpace(QDebug* this_ptr) {
  return &this_ptr->maybeSpace();
}

QDebug* qt_core_c_QDebug_noquote(QDebug* this_ptr) {
  return &this_ptr->noquote();
}

QDebug* qt_core_c_QDebug_nospace(QDebug* this_ptr) {
  return &this_ptr->nospace();
}

QDebug* qt_core_c_QDebug_operator_assign(QDebug* this_ptr, const QDebug* other) {
  return &this_ptr->operator=(*other);
}

QDebug* qt_core_c_QDebug_operator_shl_QChar(QDebug* this_ptr, const QChar* t) {
  return &this_ptr->operator<<(*t);
}

QDebug* qt_core_c_QDebug_operator_shl_QLatin1String(QDebug* this_ptr, const QLatin1String* t) {
  return &this_ptr->operator<<(*t);
}

QDebug* qt_core_c_QDebug_operator_shl_QTextStreamManipulator(QDebug* this_ptr, const QTextStreamManipulator* m) {
  return &this_ptr->operator<<(*m);
}

QDebug* qt_core_c_QDebug_operator_shl_bool(QDebug* this_ptr, bool t) {
  return &this_ptr->operator<<(t);
}

QDebug* qt_core_c_QDebug_operator_shl_char(QDebug* this_ptr, char t) {
  return &this_ptr->operator<<(t);
}

QDebug* qt_core_c_QDebug_operator_shl_char16_t(QDebug* this_ptr, char16_t t) {
  return &this_ptr->operator<<(t);
}

QDebug* qt_core_c_QDebug_operator_shl_char32_t(QDebug* this_ptr, char32_t t) {
  return &this_ptr->operator<<(t);
}

QDebug* qt_core_c_QDebug_operator_shl_const_QByteArray_ref(QDebug* this_ptr, const QByteArray* t) {
  return &this_ptr->operator<<(*t);
}

QDebug* qt_core_c_QDebug_operator_shl_const_QStringRef_ref(QDebug* this_ptr, const QStringRef* t) {
  return &this_ptr->operator<<(*t);
}

QDebug* qt_core_c_QDebug_operator_shl_const_QString_ref(QDebug* this_ptr, const QString* t) {
  return &this_ptr->operator<<(*t);
}

QDebug* qt_core_c_QDebug_operator_shl_const_char_ptr(QDebug* this_ptr, const char* t) {
  return &this_ptr->operator<<(t);
}

QDebug* qt_core_c_QDebug_operator_shl_const_void_ptr(QDebug* this_ptr, const void* t) {
  return &this_ptr->operator<<(t);
}

QDebug* qt_core_c_QDebug_operator_shl_double(QDebug* this_ptr, double t) {
  return &this_ptr->operator<<(t);
}

QDebug* qt_core_c_QDebug_operator_shl_float(QDebug* this_ptr, float t) {
  return &this_ptr->operator<<(t);
}

QDebug* qt_core_c_QDebug_operator_shl_int(QDebug* this_ptr, int t) {
  return &this_ptr->operator<<(t);
}

QDebug* qt_core_c_QDebug_operator_shl_long(QDebug* this_ptr, long t) {
  return &this_ptr->operator<<(t);
}

QDebug* qt_core_c_QDebug_operator_shl_qint64(QDebug* this_ptr, qint64 t) {
  return &this_ptr->operator<<(t);
}

QDebug* qt_core_c_QDebug_operator_shl_quint64(QDebug* this_ptr, quint64 t) {
  return &this_ptr->operator<<(t);
}

QDebug* qt_core_c_QDebug_operator_shl_short(QDebug* this_ptr, short t) {
  return &this_ptr->operator<<(t);
}

QDebug* qt_core_c_QDebug_operator_shl_unsigned_int(QDebug* this_ptr, unsigned int t) {
  return &this_ptr->operator<<(t);
}

QDebug* qt_core_c_QDebug_operator_shl_unsigned_long(QDebug* this_ptr, unsigned long t) {
  return &this_ptr->operator<<(t);
}

QDebug* qt_core_c_QDebug_operator_shl_unsigned_short(QDebug* this_ptr, unsigned short t) {
  return &this_ptr->operator<<(t);
}

QDebug* qt_core_c_QDebug_quote(QDebug* this_ptr) {
  return &this_ptr->quote();
}

QDebug* qt_core_c_QDebug_resetFormat(QDebug* this_ptr) {
  return &this_ptr->resetFormat();
}

void qt_core_c_QDebug_setAutoInsertSpaces(QDebug* this_ptr, bool b) {
  this_ptr->setAutoInsertSpaces(b);
}

void qt_core_c_QDebug_setVerbosity(QDebug* this_ptr, int verbosityLevel) {
  this_ptr->setVerbosity(verbosityLevel);
}

QDebug* qt_core_c_QDebug_space(QDebug* this_ptr) {
  return &this_ptr->space();
}

void qt_core_c_QDebug_swap(QDebug* this_ptr, QDebug* other) {
  this_ptr->swap(*other);
}

int qt_core_c_QDebug_verbosity(const QDebug* this_ptr) {
  return this_ptr->verbosity();
}

