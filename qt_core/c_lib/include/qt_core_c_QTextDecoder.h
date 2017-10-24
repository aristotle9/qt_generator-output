#ifndef QT_CORE_C_QTEXTDECODER_H
#define QT_CORE_C_QTEXTDECODER_H

#include "qt_core_c_global.h"

extern "C" {

QT_CORE_C_EXPORT void qt_core_c_QTextDecoder_delete(QTextDecoder* this_ptr);
QT_CORE_C_EXPORT bool qt_core_c_QTextDecoder_hasFailure(const QTextDecoder* this_ptr);
QT_CORE_C_EXPORT QTextDecoder* qt_core_c_QTextDecoder_new(const QTextCodec* codec);
QT_CORE_C_EXPORT void qt_core_c_QTextDecoder_toUnicode(QTextDecoder* this_ptr, QString* target, const char* chars, int len);
QT_CORE_C_EXPORT void qt_core_c_QTextDecoder_toUnicode_to_output_ba(QTextDecoder* this_ptr, const QByteArray* ba, QString* output);
QT_CORE_C_EXPORT void qt_core_c_QTextDecoder_toUnicode_to_output_chars_len(QTextDecoder* this_ptr, const char* chars, int len, QString* output);

} // extern "C"

#endif // QT_CORE_C_QTEXTDECODER_H
