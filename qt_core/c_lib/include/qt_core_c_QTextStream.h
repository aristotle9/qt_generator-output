#ifndef QT_CORE_C_QTEXTSTREAM_H
#define QT_CORE_C_QTEXTSTREAM_H

#include "qt_core_c_global.h"

extern "C" {

QT_CORE_C_EXPORT QTextStream* qt_core_c_QTextStream_G_bin(QTextStream* s);
QT_CORE_C_EXPORT QTextStream* qt_core_c_QTextStream_G_bom(QTextStream* s);
QT_CORE_C_EXPORT QTextStream* qt_core_c_QTextStream_G_center(QTextStream* s);
QT_CORE_C_EXPORT QTextStream* qt_core_c_QTextStream_G_dec(QTextStream* s);
QT_CORE_C_EXPORT QTextStream* qt_core_c_QTextStream_G_endl(QTextStream* s);
QT_CORE_C_EXPORT QTextStream* qt_core_c_QTextStream_G_fixed(QTextStream* s);
QT_CORE_C_EXPORT QTextStream* qt_core_c_QTextStream_G_flush(QTextStream* s);
QT_CORE_C_EXPORT QTextStream* qt_core_c_QTextStream_G_forcepoint(QTextStream* s);
QT_CORE_C_EXPORT QTextStream* qt_core_c_QTextStream_G_forcesign(QTextStream* s);
QT_CORE_C_EXPORT QTextStream* qt_core_c_QTextStream_G_hex(QTextStream* s);
QT_CORE_C_EXPORT QTextStream* qt_core_c_QTextStream_G_left(QTextStream* s);
QT_CORE_C_EXPORT QTextStream* qt_core_c_QTextStream_G_lowercasebase(QTextStream* s);
QT_CORE_C_EXPORT QTextStream* qt_core_c_QTextStream_G_lowercasedigits(QTextStream* s);
QT_CORE_C_EXPORT QTextStream* qt_core_c_QTextStream_G_noforcepoint(QTextStream* s);
QT_CORE_C_EXPORT QTextStream* qt_core_c_QTextStream_G_noforcesign(QTextStream* s);
QT_CORE_C_EXPORT QTextStream* qt_core_c_QTextStream_G_noshowbase(QTextStream* s);
QT_CORE_C_EXPORT QTextStream* qt_core_c_QTextStream_G_oct(QTextStream* s);
QT_CORE_C_EXPORT QTextStream* qt_core_c_QTextStream_G_operator_shl(QTextStream* s, const QTextStreamManipulator* m);
QT_CORE_C_EXPORT void qt_core_c_QTextStream_G_qSetFieldWidth_to_output(int width, QTextStreamManipulator* output);
QT_CORE_C_EXPORT void qt_core_c_QTextStream_G_qSetPadChar_to_output(const QChar* ch, QTextStreamManipulator* output);
QT_CORE_C_EXPORT void qt_core_c_QTextStream_G_qSetRealNumberPrecision_to_output(int precision, QTextStreamManipulator* output);
QT_CORE_C_EXPORT QTextStream* qt_core_c_QTextStream_G_reset(QTextStream* s);
QT_CORE_C_EXPORT QTextStream* qt_core_c_QTextStream_G_right(QTextStream* s);
QT_CORE_C_EXPORT QTextStream* qt_core_c_QTextStream_G_scientific(QTextStream* s);
QT_CORE_C_EXPORT QTextStream* qt_core_c_QTextStream_G_showbase(QTextStream* s);
QT_CORE_C_EXPORT QTextStream* qt_core_c_QTextStream_G_uppercasebase(QTextStream* s);
QT_CORE_C_EXPORT QTextStream* qt_core_c_QTextStream_G_uppercasedigits(QTextStream* s);
QT_CORE_C_EXPORT QTextStream* qt_core_c_QTextStream_G_ws(QTextStream* s);
QT_CORE_C_EXPORT bool qt_core_c_QTextStream_atEnd(const QTextStream* this_ptr);
QT_CORE_C_EXPORT bool qt_core_c_QTextStream_autoDetectUnicode(const QTextStream* this_ptr);
QT_CORE_C_EXPORT QTextCodec* qt_core_c_QTextStream_codec(const QTextStream* this_ptr);
QT_CORE_C_EXPORT void qt_core_c_QTextStream_delete(QTextStream* this_ptr);
QT_CORE_C_EXPORT QIODevice* qt_core_c_QTextStream_device(const QTextStream* this_ptr);
QT_CORE_C_EXPORT QTextStream::FieldAlignment qt_core_c_QTextStream_fieldAlignment(const QTextStream* this_ptr);
QT_CORE_C_EXPORT int qt_core_c_QTextStream_fieldWidth(const QTextStream* this_ptr);
QT_CORE_C_EXPORT void qt_core_c_QTextStream_flush(QTextStream* this_ptr);
QT_CORE_C_EXPORT bool qt_core_c_QTextStream_generateByteOrderMark(const QTextStream* this_ptr);
QT_CORE_C_EXPORT int qt_core_c_QTextStream_integerBase(const QTextStream* this_ptr);
QT_CORE_C_EXPORT void qt_core_c_QTextStream_locale_to_output(const QTextStream* this_ptr, QLocale* output);
QT_CORE_C_EXPORT QTextStream* qt_core_c_QTextStream_new_device(QIODevice* device);
QT_CORE_C_EXPORT QTextStream* qt_core_c_QTextStream_new_no_args();
QT_CORE_C_EXPORT unsigned int qt_core_c_QTextStream_numberFlags(const QTextStream* this_ptr);
QT_CORE_C_EXPORT QTextStream* qt_core_c_QTextStream_operator_shl_QByteArray_array(QTextStream* this_ptr, const QByteArray* array);
QT_CORE_C_EXPORT QTextStream* qt_core_c_QTextStream_operator_shl_QChar_ch(QTextStream* this_ptr, const QChar* ch);
QT_CORE_C_EXPORT QTextStream* qt_core_c_QTextStream_operator_shl_QLatin1String_s(QTextStream* this_ptr, const QLatin1String* s);
QT_CORE_C_EXPORT QTextStream* qt_core_c_QTextStream_operator_shl_QStringRef_s(QTextStream* this_ptr, const QStringRef* s);
QT_CORE_C_EXPORT QTextStream* qt_core_c_QTextStream_operator_shl_QString_s(QTextStream* this_ptr, const QString* s);
QT_CORE_C_EXPORT QTextStream* qt_core_c_QTextStream_operator_shl_char_c(QTextStream* this_ptr, const char* c);
QT_CORE_C_EXPORT QTextStream* qt_core_c_QTextStream_operator_shl_char_ch(QTextStream* this_ptr, char ch);
QT_CORE_C_EXPORT QTextStream* qt_core_c_QTextStream_operator_shl_double_f(QTextStream* this_ptr, double f);
QT_CORE_C_EXPORT QTextStream* qt_core_c_QTextStream_operator_shl_float_f(QTextStream* this_ptr, float f);
QT_CORE_C_EXPORT QTextStream* qt_core_c_QTextStream_operator_shl_int_i(QTextStream* this_ptr, int i);
QT_CORE_C_EXPORT QTextStream* qt_core_c_QTextStream_operator_shl_long_i(QTextStream* this_ptr, long i);
QT_CORE_C_EXPORT QTextStream* qt_core_c_QTextStream_operator_shl_qlonglong_i(QTextStream* this_ptr, qlonglong i);
QT_CORE_C_EXPORT QTextStream* qt_core_c_QTextStream_operator_shl_qulonglong_i(QTextStream* this_ptr, qulonglong i);
QT_CORE_C_EXPORT QTextStream* qt_core_c_QTextStream_operator_shl_short_i(QTextStream* this_ptr, short i);
QT_CORE_C_EXPORT QTextStream* qt_core_c_QTextStream_operator_shl_unsigned_int_i(QTextStream* this_ptr, unsigned int i);
QT_CORE_C_EXPORT QTextStream* qt_core_c_QTextStream_operator_shl_unsigned_long_i(QTextStream* this_ptr, unsigned long i);
QT_CORE_C_EXPORT QTextStream* qt_core_c_QTextStream_operator_shl_unsigned_short_i(QTextStream* this_ptr, unsigned short i);
QT_CORE_C_EXPORT QTextStream* qt_core_c_QTextStream_operator_shl_void_ptr(QTextStream* this_ptr, const void* ptr);
QT_CORE_C_EXPORT QTextStream* qt_core_c_QTextStream_operator_shr_QByteArray_array(QTextStream* this_ptr, QByteArray* array);
QT_CORE_C_EXPORT QTextStream* qt_core_c_QTextStream_operator_shr_QChar_ch(QTextStream* this_ptr, QChar* ch);
QT_CORE_C_EXPORT QTextStream* qt_core_c_QTextStream_operator_shr_QString_s(QTextStream* this_ptr, QString* s);
QT_CORE_C_EXPORT QTextStream* qt_core_c_QTextStream_operator_shr_char_c(QTextStream* this_ptr, char* c);
QT_CORE_C_EXPORT QTextStream* qt_core_c_QTextStream_operator_shr_char_ch(QTextStream* this_ptr, char* ch);
QT_CORE_C_EXPORT QTextStream* qt_core_c_QTextStream_operator_shr_double_f(QTextStream* this_ptr, double* f);
QT_CORE_C_EXPORT QTextStream* qt_core_c_QTextStream_operator_shr_float_f(QTextStream* this_ptr, float* f);
QT_CORE_C_EXPORT QTextStream* qt_core_c_QTextStream_operator_shr_int_i(QTextStream* this_ptr, int* i);
QT_CORE_C_EXPORT QTextStream* qt_core_c_QTextStream_operator_shr_long_i(QTextStream* this_ptr, long* i);
QT_CORE_C_EXPORT QTextStream* qt_core_c_QTextStream_operator_shr_qlonglong_i(QTextStream* this_ptr, qlonglong* i);
QT_CORE_C_EXPORT QTextStream* qt_core_c_QTextStream_operator_shr_qulonglong_i(QTextStream* this_ptr, qulonglong* i);
QT_CORE_C_EXPORT QTextStream* qt_core_c_QTextStream_operator_shr_short_i(QTextStream* this_ptr, short* i);
QT_CORE_C_EXPORT QTextStream* qt_core_c_QTextStream_operator_shr_unsigned_int_i(QTextStream* this_ptr, unsigned int* i);
QT_CORE_C_EXPORT QTextStream* qt_core_c_QTextStream_operator_shr_unsigned_long_i(QTextStream* this_ptr, unsigned long* i);
QT_CORE_C_EXPORT QTextStream* qt_core_c_QTextStream_operator_shr_unsigned_short_i(QTextStream* this_ptr, unsigned short* i);
QT_CORE_C_EXPORT void qt_core_c_QTextStream_padChar_to_output(const QTextStream* this_ptr, QChar* output);
QT_CORE_C_EXPORT qint64 qt_core_c_QTextStream_pos(const QTextStream* this_ptr);
QT_CORE_C_EXPORT void qt_core_c_QTextStream_readAll_to_output(QTextStream* this_ptr, QString* output);
QT_CORE_C_EXPORT bool qt_core_c_QTextStream_readLineInto_line(QTextStream* this_ptr, QString* line);
QT_CORE_C_EXPORT bool qt_core_c_QTextStream_readLineInto_line_maxlen(QTextStream* this_ptr, QString* line, qint64 maxlen);
QT_CORE_C_EXPORT void qt_core_c_QTextStream_readLine_to_output_maxlen(QTextStream* this_ptr, qint64 maxlen, QString* output);
QT_CORE_C_EXPORT void qt_core_c_QTextStream_readLine_to_output_no_args(QTextStream* this_ptr, QString* output);
QT_CORE_C_EXPORT void qt_core_c_QTextStream_read_to_output(QTextStream* this_ptr, qint64 maxlen, QString* output);
QT_CORE_C_EXPORT QTextStream::RealNumberNotation qt_core_c_QTextStream_realNumberNotation(const QTextStream* this_ptr);
QT_CORE_C_EXPORT int qt_core_c_QTextStream_realNumberPrecision(const QTextStream* this_ptr);
QT_CORE_C_EXPORT void qt_core_c_QTextStream_reset(QTextStream* this_ptr);
QT_CORE_C_EXPORT void qt_core_c_QTextStream_resetStatus(QTextStream* this_ptr);
QT_CORE_C_EXPORT bool qt_core_c_QTextStream_seek(QTextStream* this_ptr, qint64 pos);
QT_CORE_C_EXPORT void qt_core_c_QTextStream_setAutoDetectUnicode(QTextStream* this_ptr, bool enabled);
QT_CORE_C_EXPORT void qt_core_c_QTextStream_setCodec_codec(QTextStream* this_ptr, QTextCodec* codec);
QT_CORE_C_EXPORT void qt_core_c_QTextStream_setCodec_codecName(QTextStream* this_ptr, const char* codecName);
QT_CORE_C_EXPORT void qt_core_c_QTextStream_setDevice(QTextStream* this_ptr, QIODevice* device);
QT_CORE_C_EXPORT void qt_core_c_QTextStream_setFieldAlignment(QTextStream* this_ptr, QTextStream::FieldAlignment alignment);
QT_CORE_C_EXPORT void qt_core_c_QTextStream_setFieldWidth(QTextStream* this_ptr, int width);
QT_CORE_C_EXPORT void qt_core_c_QTextStream_setGenerateByteOrderMark(QTextStream* this_ptr, bool generate);
QT_CORE_C_EXPORT void qt_core_c_QTextStream_setIntegerBase(QTextStream* this_ptr, int base);
QT_CORE_C_EXPORT void qt_core_c_QTextStream_setLocale(QTextStream* this_ptr, const QLocale* locale);
QT_CORE_C_EXPORT void qt_core_c_QTextStream_setNumberFlags(QTextStream* this_ptr, unsigned int flags);
QT_CORE_C_EXPORT void qt_core_c_QTextStream_setPadChar(QTextStream* this_ptr, const QChar* ch);
QT_CORE_C_EXPORT void qt_core_c_QTextStream_setRealNumberNotation(QTextStream* this_ptr, QTextStream::RealNumberNotation notation);
QT_CORE_C_EXPORT void qt_core_c_QTextStream_setRealNumberPrecision(QTextStream* this_ptr, int precision);
QT_CORE_C_EXPORT void qt_core_c_QTextStream_setStatus(QTextStream* this_ptr, QTextStream::Status status);
QT_CORE_C_EXPORT void qt_core_c_QTextStream_skipWhiteSpace(QTextStream* this_ptr);
QT_CORE_C_EXPORT QTextStream::Status qt_core_c_QTextStream_status(const QTextStream* this_ptr);
QT_CORE_C_EXPORT QString* qt_core_c_QTextStream_string(const QTextStream* this_ptr);

} // extern "C"

#endif // QT_CORE_C_QTEXTSTREAM_H
