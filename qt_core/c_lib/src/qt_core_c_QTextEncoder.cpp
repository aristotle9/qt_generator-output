#include "qt_core_c_QTextEncoder.h"

void qt_core_c_QTextEncoder_delete(QTextEncoder* this_ptr) {
  delete this_ptr;
}

void qt_core_c_QTextEncoder_fromUnicode_to_output_str(QTextEncoder* this_ptr, const QString* str, QByteArray* output) {
  new(output) QByteArray(this_ptr->fromUnicode(*str));
}

void qt_core_c_QTextEncoder_fromUnicode_to_output_uc_len(QTextEncoder* this_ptr, const QChar* uc, int len, QByteArray* output) {
  new(output) QByteArray(this_ptr->fromUnicode(uc, len));
}

bool qt_core_c_QTextEncoder_hasFailure(const QTextEncoder* this_ptr) {
  return this_ptr->hasFailure();
}

QTextEncoder* qt_core_c_QTextEncoder_new(const QTextCodec* codec) {
  return new QTextEncoder(codec);
}

