#include "qt_core_c_QTextCodec.h"

void* qt_core_c_QTextCodec_ConverterState_d(const QTextCodec::ConverterState* this_ptr) {
  return this_ptr->d;
}

void qt_core_c_QTextCodec_ConverterState_delete(QTextCodec::ConverterState* this_ptr) {
  delete this_ptr;
}

unsigned int qt_core_c_QTextCodec_ConverterState_flags(const QTextCodec::ConverterState* this_ptr) {
  return uint(this_ptr->flags);
}

int qt_core_c_QTextCodec_ConverterState_invalidChars(const QTextCodec::ConverterState* this_ptr) {
  return this_ptr->invalidChars;
}

QTextCodec::ConverterState* qt_core_c_QTextCodec_ConverterState_new_f(unsigned int f) {
  return new QTextCodec::ConverterState(QFlags< QTextCodec::ConversionFlag >(f));
}

QTextCodec::ConverterState* qt_core_c_QTextCodec_ConverterState_new_no_args() {
  return new QTextCodec::ConverterState();
}

int qt_core_c_QTextCodec_ConverterState_remainingChars(const QTextCodec::ConverterState* this_ptr) {
  return this_ptr->remainingChars;
}

void qt_core_c_QTextCodec_ConverterState_set_d(QTextCodec::ConverterState* this_ptr, void* value) {
  this_ptr->d = value;
}

void qt_core_c_QTextCodec_ConverterState_set_flags(QTextCodec::ConverterState* this_ptr, unsigned int value) {
  this_ptr->flags = QFlags< QTextCodec::ConversionFlag >(value);
}

void qt_core_c_QTextCodec_ConverterState_set_invalidChars(QTextCodec::ConverterState* this_ptr, int value) {
  this_ptr->invalidChars = value;
}

void qt_core_c_QTextCodec_ConverterState_set_remainingChars(QTextCodec::ConverterState* this_ptr, int value) {
  this_ptr->remainingChars = value;
}

void qt_core_c_QTextCodec_aliases_to_output(const QTextCodec* this_ptr, QList< QByteArray >* output) {
  new(output) QList< QByteArray >(this_ptr->aliases());
}

void qt_core_c_QTextCodec_availableCodecs_to_output(QList< QByteArray >* output) {
  new(output) QList< QByteArray >(QTextCodec::availableCodecs());
}

void qt_core_c_QTextCodec_availableMibs_to_output(QList< int >* output) {
  new(output) QList< int >(QTextCodec::availableMibs());
}

bool qt_core_c_QTextCodec_canEncode_QChar(const QTextCodec* this_ptr, const QChar* arg1) {
  return this_ptr->canEncode(*arg1);
}

bool qt_core_c_QTextCodec_canEncode_QString(const QTextCodec* this_ptr, const QString* arg1) {
  return this_ptr->canEncode(*arg1);
}

QTextCodec* qt_core_c_QTextCodec_codecForHtml_ba(const QByteArray* ba) {
  return QTextCodec::codecForHtml(*ba);
}

QTextCodec* qt_core_c_QTextCodec_codecForHtml_ba_defaultCodec(const QByteArray* ba, QTextCodec* defaultCodec) {
  return QTextCodec::codecForHtml(*ba, defaultCodec);
}

QTextCodec* qt_core_c_QTextCodec_codecForLocale() {
  return QTextCodec::codecForLocale();
}

QTextCodec* qt_core_c_QTextCodec_codecForMib(int mib) {
  return QTextCodec::codecForMib(mib);
}

QTextCodec* qt_core_c_QTextCodec_codecForName_QByteArray(const QByteArray* name) {
  return QTextCodec::codecForName(*name);
}

QTextCodec* qt_core_c_QTextCodec_codecForName_char(const char* name) {
  return QTextCodec::codecForName(name);
}

QTextCodec* qt_core_c_QTextCodec_codecForUtfText_ba(const QByteArray* ba) {
  return QTextCodec::codecForUtfText(*ba);
}

QTextCodec* qt_core_c_QTextCodec_codecForUtfText_ba_defaultCodec(const QByteArray* ba, QTextCodec* defaultCodec) {
  return QTextCodec::codecForUtfText(*ba, defaultCodec);
}

void qt_core_c_QTextCodec_fromUnicode_to_output_in_length(const QTextCodec* this_ptr, const QChar* in, int length, QByteArray* output) {
  new(output) QByteArray(this_ptr->fromUnicode(in, length));
}

void qt_core_c_QTextCodec_fromUnicode_to_output_in_length_state(const QTextCodec* this_ptr, const QChar* in, int length, QTextCodec::ConverterState* state, QByteArray* output) {
  new(output) QByteArray(this_ptr->fromUnicode(in, length, state));
}

void qt_core_c_QTextCodec_fromUnicode_to_output_uc(const QTextCodec* this_ptr, const QString* uc, QByteArray* output) {
  new(output) QByteArray(this_ptr->fromUnicode(*uc));
}

QTextDecoder* qt_core_c_QTextCodec_makeDecoder_flags(const QTextCodec* this_ptr, unsigned int flags) {
  return this_ptr->makeDecoder(QFlags< QTextCodec::ConversionFlag >(flags));
}

QTextDecoder* qt_core_c_QTextCodec_makeDecoder_no_args(const QTextCodec* this_ptr) {
  return this_ptr->makeDecoder();
}

QTextEncoder* qt_core_c_QTextCodec_makeEncoder_flags(const QTextCodec* this_ptr, unsigned int flags) {
  return this_ptr->makeEncoder(QFlags< QTextCodec::ConversionFlag >(flags));
}

QTextEncoder* qt_core_c_QTextCodec_makeEncoder_no_args(const QTextCodec* this_ptr) {
  return this_ptr->makeEncoder();
}

int qt_core_c_QTextCodec_mibEnum(const QTextCodec* this_ptr) {
  return this_ptr->mibEnum();
}

void qt_core_c_QTextCodec_name_to_output(const QTextCodec* this_ptr, QByteArray* output) {
  new(output) QByteArray(this_ptr->name());
}

void qt_core_c_QTextCodec_setCodecForLocale(QTextCodec* c) {
  QTextCodec::setCodecForLocale(c);
}

void qt_core_c_QTextCodec_toUnicode_to_output_arg1(const QTextCodec* this_ptr, const QByteArray* arg1, QString* output) {
  new(output) QString(this_ptr->toUnicode(*arg1));
}

void qt_core_c_QTextCodec_toUnicode_to_output_chars(const QTextCodec* this_ptr, const char* chars, QString* output) {
  new(output) QString(this_ptr->toUnicode(chars));
}

void qt_core_c_QTextCodec_toUnicode_to_output_in_length(const QTextCodec* this_ptr, const char* in, int length, QString* output) {
  new(output) QString(this_ptr->toUnicode(in, length));
}

void qt_core_c_QTextCodec_toUnicode_to_output_in_length_state(const QTextCodec* this_ptr, const char* in, int length, QTextCodec::ConverterState* state, QString* output) {
  new(output) QString(this_ptr->toUnicode(in, length, state));
}

