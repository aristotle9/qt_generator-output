#include "qt_core_c_QTextDecoder.h"

void qt_core_c_QTextDecoder_delete(QTextDecoder* this_ptr) {
  delete this_ptr;
}

bool qt_core_c_QTextDecoder_hasFailure(const QTextDecoder* this_ptr) {
  return this_ptr->hasFailure();
}

QTextDecoder* qt_core_c_QTextDecoder_new(const QTextCodec* codec) {
  return new QTextDecoder(codec);
}

void qt_core_c_QTextDecoder_toUnicode(QTextDecoder* this_ptr, QString* target, const char* chars, int len) {
  this_ptr->toUnicode(target, chars, len);
}

void qt_core_c_QTextDecoder_toUnicode_to_output_ba(QTextDecoder* this_ptr, const QByteArray* ba, QString* output) {
  new(output) QString(this_ptr->toUnicode(*ba));
}

void qt_core_c_QTextDecoder_toUnicode_to_output_chars_len(QTextDecoder* this_ptr, const char* chars, int len, QString* output) {
  new(output) QString(this_ptr->toUnicode(chars, len));
}

