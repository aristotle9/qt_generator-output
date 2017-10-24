#ifndef QT_CORE_C_QTEXTCODEC_H
#define QT_CORE_C_QTEXTCODEC_H

#include "qt_core_c_global.h"

extern "C" {

QT_CORE_C_EXPORT void* qt_core_c_QTextCodec_ConverterState_d(const QTextCodec::ConverterState* this_ptr);
QT_CORE_C_EXPORT void qt_core_c_QTextCodec_ConverterState_delete(QTextCodec::ConverterState* this_ptr);
QT_CORE_C_EXPORT unsigned int qt_core_c_QTextCodec_ConverterState_flags(const QTextCodec::ConverterState* this_ptr);
QT_CORE_C_EXPORT int qt_core_c_QTextCodec_ConverterState_invalidChars(const QTextCodec::ConverterState* this_ptr);
QT_CORE_C_EXPORT QTextCodec::ConverterState* qt_core_c_QTextCodec_ConverterState_new_f(unsigned int f);
QT_CORE_C_EXPORT QTextCodec::ConverterState* qt_core_c_QTextCodec_ConverterState_new_no_args();
QT_CORE_C_EXPORT int qt_core_c_QTextCodec_ConverterState_remainingChars(const QTextCodec::ConverterState* this_ptr);
QT_CORE_C_EXPORT void qt_core_c_QTextCodec_ConverterState_set_d(QTextCodec::ConverterState* this_ptr, void* value);
QT_CORE_C_EXPORT void qt_core_c_QTextCodec_ConverterState_set_flags(QTextCodec::ConverterState* this_ptr, unsigned int value);
QT_CORE_C_EXPORT void qt_core_c_QTextCodec_ConverterState_set_invalidChars(QTextCodec::ConverterState* this_ptr, int value);
QT_CORE_C_EXPORT void qt_core_c_QTextCodec_ConverterState_set_remainingChars(QTextCodec::ConverterState* this_ptr, int value);
QT_CORE_C_EXPORT void qt_core_c_QTextCodec_aliases_to_output(const QTextCodec* this_ptr, QList< QByteArray >* output);
QT_CORE_C_EXPORT void qt_core_c_QTextCodec_availableCodecs_to_output(QList< QByteArray >* output);
QT_CORE_C_EXPORT void qt_core_c_QTextCodec_availableMibs_to_output(QList< int >* output);
QT_CORE_C_EXPORT bool qt_core_c_QTextCodec_canEncode_QChar(const QTextCodec* this_ptr, const QChar* arg1);
QT_CORE_C_EXPORT bool qt_core_c_QTextCodec_canEncode_QString(const QTextCodec* this_ptr, const QString* arg1);
QT_CORE_C_EXPORT QTextCodec* qt_core_c_QTextCodec_codecForHtml_ba(const QByteArray* ba);
QT_CORE_C_EXPORT QTextCodec* qt_core_c_QTextCodec_codecForHtml_ba_defaultCodec(const QByteArray* ba, QTextCodec* defaultCodec);
QT_CORE_C_EXPORT QTextCodec* qt_core_c_QTextCodec_codecForLocale();
QT_CORE_C_EXPORT QTextCodec* qt_core_c_QTextCodec_codecForMib(int mib);
QT_CORE_C_EXPORT QTextCodec* qt_core_c_QTextCodec_codecForName_QByteArray(const QByteArray* name);
QT_CORE_C_EXPORT QTextCodec* qt_core_c_QTextCodec_codecForName_char(const char* name);
QT_CORE_C_EXPORT QTextCodec* qt_core_c_QTextCodec_codecForUtfText_ba(const QByteArray* ba);
QT_CORE_C_EXPORT QTextCodec* qt_core_c_QTextCodec_codecForUtfText_ba_defaultCodec(const QByteArray* ba, QTextCodec* defaultCodec);
QT_CORE_C_EXPORT void qt_core_c_QTextCodec_fromUnicode_to_output_in_length(const QTextCodec* this_ptr, const QChar* in, int length, QByteArray* output);
QT_CORE_C_EXPORT void qt_core_c_QTextCodec_fromUnicode_to_output_in_length_state(const QTextCodec* this_ptr, const QChar* in, int length, QTextCodec::ConverterState* state, QByteArray* output);
QT_CORE_C_EXPORT void qt_core_c_QTextCodec_fromUnicode_to_output_uc(const QTextCodec* this_ptr, const QString* uc, QByteArray* output);
QT_CORE_C_EXPORT QTextDecoder* qt_core_c_QTextCodec_makeDecoder_flags(const QTextCodec* this_ptr, unsigned int flags);
QT_CORE_C_EXPORT QTextDecoder* qt_core_c_QTextCodec_makeDecoder_no_args(const QTextCodec* this_ptr);
QT_CORE_C_EXPORT QTextEncoder* qt_core_c_QTextCodec_makeEncoder_flags(const QTextCodec* this_ptr, unsigned int flags);
QT_CORE_C_EXPORT QTextEncoder* qt_core_c_QTextCodec_makeEncoder_no_args(const QTextCodec* this_ptr);
QT_CORE_C_EXPORT int qt_core_c_QTextCodec_mibEnum(const QTextCodec* this_ptr);
QT_CORE_C_EXPORT void qt_core_c_QTextCodec_name_to_output(const QTextCodec* this_ptr, QByteArray* output);
QT_CORE_C_EXPORT void qt_core_c_QTextCodec_setCodecForLocale(QTextCodec* c);
QT_CORE_C_EXPORT void qt_core_c_QTextCodec_toUnicode_to_output_arg1(const QTextCodec* this_ptr, const QByteArray* arg1, QString* output);
QT_CORE_C_EXPORT void qt_core_c_QTextCodec_toUnicode_to_output_chars(const QTextCodec* this_ptr, const char* chars, QString* output);
QT_CORE_C_EXPORT void qt_core_c_QTextCodec_toUnicode_to_output_in_length(const QTextCodec* this_ptr, const char* in, int length, QString* output);
QT_CORE_C_EXPORT void qt_core_c_QTextCodec_toUnicode_to_output_in_length_state(const QTextCodec* this_ptr, const char* in, int length, QTextCodec::ConverterState* state, QString* output);

} // extern "C"

#endif // QT_CORE_C_QTEXTCODEC_H
