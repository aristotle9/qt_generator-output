#ifndef QT_CORE_C_QTEXTENCODER_H
#define QT_CORE_C_QTEXTENCODER_H

#include "qt_core_c_global.h"

extern "C" {

QT_CORE_C_EXPORT void qt_core_c_QTextEncoder_delete(QTextEncoder* this_ptr);
QT_CORE_C_EXPORT void qt_core_c_QTextEncoder_fromUnicode_to_output_str(QTextEncoder* this_ptr, const QString* str, QByteArray* output);
QT_CORE_C_EXPORT void qt_core_c_QTextEncoder_fromUnicode_to_output_uc_len(QTextEncoder* this_ptr, const QChar* uc, int len, QByteArray* output);
QT_CORE_C_EXPORT bool qt_core_c_QTextEncoder_hasFailure(const QTextEncoder* this_ptr);
QT_CORE_C_EXPORT QTextEncoder* qt_core_c_QTextEncoder_new(const QTextCodec* codec);

} // extern "C"

#endif // QT_CORE_C_QTEXTENCODER_H
