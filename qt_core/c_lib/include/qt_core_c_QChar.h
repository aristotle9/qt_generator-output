#ifndef QT_CORE_C_QCHAR_H
#define QT_CORE_C_QCHAR_H

#include "qt_core_c_global.h"

extern "C" {

QT_CORE_C_EXPORT bool qt_core_c_QChar_G_operator_ge(const QChar* c1, const QChar* c2);
QT_CORE_C_EXPORT bool qt_core_c_QChar_G_operator_gt(const QChar* c1, const QChar* c2);
QT_CORE_C_EXPORT bool qt_core_c_QChar_G_operator_le(const QChar* c1, const QChar* c2);
QT_CORE_C_EXPORT bool qt_core_c_QChar_G_operator_neq(const QChar* c1, const QChar* c2);
QT_CORE_C_EXPORT QDataStream* qt_core_c_QChar_G_operator_shl(QDataStream* arg1, const QChar* arg2);
QT_CORE_C_EXPORT QDataStream* qt_core_c_QChar_G_operator_shr(QDataStream* arg1, QChar* arg2);
QT_CORE_C_EXPORT QChar::Category qt_core_c_QChar_category_no_args(const QChar* this_ptr);
QT_CORE_C_EXPORT QChar::Category qt_core_c_QChar_category_ucs4(unsigned int ucs4);
QT_CORE_C_EXPORT unsigned char qt_core_c_QChar_cell(const QChar* this_ptr);
QT_CORE_C_EXPORT unsigned char qt_core_c_QChar_combiningClass_no_args(const QChar* this_ptr);
QT_CORE_C_EXPORT unsigned char qt_core_c_QChar_combiningClass_ucs4(unsigned int ucs4);
QT_CORE_C_EXPORT void qt_core_c_QChar_constructor_QChar_SpecialCharacter(QChar::SpecialCharacter s, QChar* output);
QT_CORE_C_EXPORT void qt_core_c_QChar_constructor_QLatin1Char(const QLatin1Char* ch, QChar* output);
QT_CORE_C_EXPORT void qt_core_c_QChar_constructor_char(char c, QChar* output);
QT_CORE_C_EXPORT void qt_core_c_QChar_constructor_int(int rc, QChar* output);
QT_CORE_C_EXPORT void qt_core_c_QChar_constructor_no_args(QChar* output);
QT_CORE_C_EXPORT void qt_core_c_QChar_constructor_short(short rc, QChar* output);
QT_CORE_C_EXPORT void qt_core_c_QChar_constructor_unsigned_char(unsigned char c, QChar* output);
QT_CORE_C_EXPORT void qt_core_c_QChar_constructor_unsigned_char_unsigned_char(unsigned char c, unsigned char r, QChar* output);
QT_CORE_C_EXPORT void qt_core_c_QChar_constructor_unsigned_int(unsigned int rc, QChar* output);
QT_CORE_C_EXPORT void qt_core_c_QChar_constructor_unsigned_short(unsigned short rc, QChar* output);
QT_CORE_C_EXPORT QChar::UnicodeVersion qt_core_c_QChar_currentUnicodeVersion();
QT_CORE_C_EXPORT QChar::Decomposition qt_core_c_QChar_decompositionTag_no_args(const QChar* this_ptr);
QT_CORE_C_EXPORT QChar::Decomposition qt_core_c_QChar_decompositionTag_ucs4(unsigned int ucs4);
QT_CORE_C_EXPORT void qt_core_c_QChar_decomposition_to_output_no_args(const QChar* this_ptr, QString* output);
QT_CORE_C_EXPORT void qt_core_c_QChar_decomposition_to_output_ucs4(unsigned int ucs4, QString* output);
QT_CORE_C_EXPORT void qt_core_c_QChar_destructor(QChar* this_ptr);
QT_CORE_C_EXPORT int qt_core_c_QChar_digitValue_no_args(const QChar* this_ptr);
QT_CORE_C_EXPORT int qt_core_c_QChar_digitValue_ucs4(unsigned int ucs4);
QT_CORE_C_EXPORT QChar::Direction qt_core_c_QChar_direction_no_args(const QChar* this_ptr);
QT_CORE_C_EXPORT QChar::Direction qt_core_c_QChar_direction_ucs4(unsigned int ucs4);
QT_CORE_C_EXPORT void qt_core_c_QChar_fromLatin1_to_output(char c, QChar* output);
QT_CORE_C_EXPORT bool qt_core_c_QChar_hasMirrored_no_args(const QChar* this_ptr);
QT_CORE_C_EXPORT bool qt_core_c_QChar_hasMirrored_ucs4(unsigned int ucs4);
QT_CORE_C_EXPORT unsigned short qt_core_c_QChar_highSurrogate(unsigned int ucs4);
QT_CORE_C_EXPORT bool qt_core_c_QChar_isDigit_no_args(const QChar* this_ptr);
QT_CORE_C_EXPORT bool qt_core_c_QChar_isDigit_ucs4(unsigned int ucs4);
QT_CORE_C_EXPORT bool qt_core_c_QChar_isHighSurrogate_no_args(const QChar* this_ptr);
QT_CORE_C_EXPORT bool qt_core_c_QChar_isHighSurrogate_ucs4(unsigned int ucs4);
QT_CORE_C_EXPORT bool qt_core_c_QChar_isLetterOrNumber_no_args(const QChar* this_ptr);
QT_CORE_C_EXPORT bool qt_core_c_QChar_isLetterOrNumber_ucs4(unsigned int ucs4);
QT_CORE_C_EXPORT bool qt_core_c_QChar_isLetter_no_args(const QChar* this_ptr);
QT_CORE_C_EXPORT bool qt_core_c_QChar_isLetter_ucs4(unsigned int ucs4);
QT_CORE_C_EXPORT bool qt_core_c_QChar_isLowSurrogate_no_args(const QChar* this_ptr);
QT_CORE_C_EXPORT bool qt_core_c_QChar_isLowSurrogate_ucs4(unsigned int ucs4);
QT_CORE_C_EXPORT bool qt_core_c_QChar_isLower_no_args(const QChar* this_ptr);
QT_CORE_C_EXPORT bool qt_core_c_QChar_isLower_ucs4(unsigned int ucs4);
QT_CORE_C_EXPORT bool qt_core_c_QChar_isMark_no_args(const QChar* this_ptr);
QT_CORE_C_EXPORT bool qt_core_c_QChar_isMark_ucs4(unsigned int ucs4);
QT_CORE_C_EXPORT bool qt_core_c_QChar_isNonCharacter_no_args(const QChar* this_ptr);
QT_CORE_C_EXPORT bool qt_core_c_QChar_isNonCharacter_ucs4(unsigned int ucs4);
QT_CORE_C_EXPORT bool qt_core_c_QChar_isNull(const QChar* this_ptr);
QT_CORE_C_EXPORT bool qt_core_c_QChar_isNumber_no_args(const QChar* this_ptr);
QT_CORE_C_EXPORT bool qt_core_c_QChar_isNumber_ucs4(unsigned int ucs4);
QT_CORE_C_EXPORT bool qt_core_c_QChar_isPrint_no_args(const QChar* this_ptr);
QT_CORE_C_EXPORT bool qt_core_c_QChar_isPrint_ucs4(unsigned int ucs4);
QT_CORE_C_EXPORT bool qt_core_c_QChar_isPunct_no_args(const QChar* this_ptr);
QT_CORE_C_EXPORT bool qt_core_c_QChar_isPunct_ucs4(unsigned int ucs4);
QT_CORE_C_EXPORT bool qt_core_c_QChar_isSpace_no_args(const QChar* this_ptr);
QT_CORE_C_EXPORT bool qt_core_c_QChar_isSpace_ucs4(unsigned int ucs4);
QT_CORE_C_EXPORT bool qt_core_c_QChar_isSurrogate_no_args(const QChar* this_ptr);
QT_CORE_C_EXPORT bool qt_core_c_QChar_isSurrogate_ucs4(unsigned int ucs4);
QT_CORE_C_EXPORT bool qt_core_c_QChar_isSymbol_no_args(const QChar* this_ptr);
QT_CORE_C_EXPORT bool qt_core_c_QChar_isSymbol_ucs4(unsigned int ucs4);
QT_CORE_C_EXPORT bool qt_core_c_QChar_isTitleCase_no_args(const QChar* this_ptr);
QT_CORE_C_EXPORT bool qt_core_c_QChar_isTitleCase_ucs4(unsigned int ucs4);
QT_CORE_C_EXPORT bool qt_core_c_QChar_isUpper_no_args(const QChar* this_ptr);
QT_CORE_C_EXPORT bool qt_core_c_QChar_isUpper_ucs4(unsigned int ucs4);
QT_CORE_C_EXPORT QChar::JoiningType qt_core_c_QChar_joiningType_no_args(const QChar* this_ptr);
QT_CORE_C_EXPORT QChar::JoiningType qt_core_c_QChar_joiningType_ucs4(unsigned int ucs4);
QT_CORE_C_EXPORT QChar::Joining qt_core_c_QChar_joining_no_args(const QChar* this_ptr);
QT_CORE_C_EXPORT QChar::Joining qt_core_c_QChar_joining_ucs4(unsigned int ucs4);
QT_CORE_C_EXPORT unsigned short qt_core_c_QChar_lowSurrogate(unsigned int ucs4);
QT_CORE_C_EXPORT unsigned int qt_core_c_QChar_mirroredChar(unsigned int ucs4);
QT_CORE_C_EXPORT void qt_core_c_QChar_mirroredChar_to_output(const QChar* this_ptr, QChar* output);
QT_CORE_C_EXPORT bool qt_core_c_QChar_requiresSurrogates(unsigned int ucs4);
QT_CORE_C_EXPORT unsigned char qt_core_c_QChar_row(const QChar* this_ptr);
QT_CORE_C_EXPORT QChar::Script qt_core_c_QChar_script_no_args(const QChar* this_ptr);
QT_CORE_C_EXPORT QChar::Script qt_core_c_QChar_script_ucs4(unsigned int ucs4);
QT_CORE_C_EXPORT void qt_core_c_QChar_setCell(QChar* this_ptr, unsigned char acell);
QT_CORE_C_EXPORT void qt_core_c_QChar_setRow(QChar* this_ptr, unsigned char arow);
QT_CORE_C_EXPORT unsigned int qt_core_c_QChar_surrogateToUcs4_QChar_QChar(const QChar* high, const QChar* low);
QT_CORE_C_EXPORT unsigned int qt_core_c_QChar_surrogateToUcs4_unsigned_short_unsigned_short(unsigned short high, unsigned short low);
QT_CORE_C_EXPORT unsigned int qt_core_c_QChar_toCaseFolded(unsigned int ucs4);
QT_CORE_C_EXPORT void qt_core_c_QChar_toCaseFolded_to_output(const QChar* this_ptr, QChar* output);
QT_CORE_C_EXPORT char qt_core_c_QChar_toLatin1(const QChar* this_ptr);
QT_CORE_C_EXPORT unsigned int qt_core_c_QChar_toLower(unsigned int ucs4);
QT_CORE_C_EXPORT void qt_core_c_QChar_toLower_to_output(const QChar* this_ptr, QChar* output);
QT_CORE_C_EXPORT unsigned int qt_core_c_QChar_toTitleCase(unsigned int ucs4);
QT_CORE_C_EXPORT void qt_core_c_QChar_toTitleCase_to_output(const QChar* this_ptr, QChar* output);
QT_CORE_C_EXPORT unsigned int qt_core_c_QChar_toUpper(unsigned int ucs4);
QT_CORE_C_EXPORT void qt_core_c_QChar_toUpper_to_output(const QChar* this_ptr, QChar* output);
QT_CORE_C_EXPORT unsigned short* qt_core_c_QChar_unicode(QChar* this_ptr);
QT_CORE_C_EXPORT QChar::UnicodeVersion qt_core_c_QChar_unicodeVersion_no_args(const QChar* this_ptr);
QT_CORE_C_EXPORT QChar::UnicodeVersion qt_core_c_QChar_unicodeVersion_ucs4(unsigned int ucs4);
QT_CORE_C_EXPORT unsigned short qt_core_c_QChar_unicode_const(const QChar* this_ptr);

} // extern "C"

#endif // QT_CORE_C_QCHAR_H
