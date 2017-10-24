#ifndef QT_CORE_C_QCHARREF_H
#define QT_CORE_C_QCHARREF_H

#include "qt_core_c_global.h"

extern "C" {

QT_CORE_C_EXPORT unsigned char qt_core_c_QCharRef_cell(const QCharRef* this_ptr);
QT_CORE_C_EXPORT unsigned char qt_core_c_QCharRef_combiningClass(const QCharRef* this_ptr);
QT_CORE_C_EXPORT void qt_core_c_QCharRef_convert_to_QChar_to_output(const QCharRef* this_ptr, QChar* output);
QT_CORE_C_EXPORT void qt_core_c_QCharRef_decomposition_to_output(const QCharRef* this_ptr, QString* output);
QT_CORE_C_EXPORT void qt_core_c_QCharRef_destructor(QCharRef* this_ptr);
QT_CORE_C_EXPORT int qt_core_c_QCharRef_digitValue(const QCharRef* this_ptr);
QT_CORE_C_EXPORT bool qt_core_c_QCharRef_hasMirrored(const QCharRef* this_ptr);
QT_CORE_C_EXPORT bool qt_core_c_QCharRef_isDigit(const QCharRef* this_ptr);
QT_CORE_C_EXPORT bool qt_core_c_QCharRef_isLetter(const QCharRef* this_ptr);
QT_CORE_C_EXPORT bool qt_core_c_QCharRef_isLetterOrNumber(QCharRef* this_ptr);
QT_CORE_C_EXPORT bool qt_core_c_QCharRef_isLower(const QCharRef* this_ptr);
QT_CORE_C_EXPORT bool qt_core_c_QCharRef_isMark(const QCharRef* this_ptr);
QT_CORE_C_EXPORT bool qt_core_c_QCharRef_isNull(const QCharRef* this_ptr);
QT_CORE_C_EXPORT bool qt_core_c_QCharRef_isNumber(const QCharRef* this_ptr);
QT_CORE_C_EXPORT bool qt_core_c_QCharRef_isPrint(const QCharRef* this_ptr);
QT_CORE_C_EXPORT bool qt_core_c_QCharRef_isPunct(const QCharRef* this_ptr);
QT_CORE_C_EXPORT bool qt_core_c_QCharRef_isSpace(const QCharRef* this_ptr);
QT_CORE_C_EXPORT bool qt_core_c_QCharRef_isTitleCase(const QCharRef* this_ptr);
QT_CORE_C_EXPORT bool qt_core_c_QCharRef_isUpper(const QCharRef* this_ptr);
QT_CORE_C_EXPORT void qt_core_c_QCharRef_mirroredChar_to_output(const QCharRef* this_ptr, QChar* output);
QT_CORE_C_EXPORT QCharRef* qt_core_c_QCharRef_operator_assign_QChar(QCharRef* this_ptr, const QChar* c);
QT_CORE_C_EXPORT QCharRef* qt_core_c_QCharRef_operator_assign_QCharRef(QCharRef* this_ptr, const QCharRef* c);
QT_CORE_C_EXPORT QCharRef* qt_core_c_QCharRef_operator_assign_char(QCharRef* this_ptr, char c);
QT_CORE_C_EXPORT QCharRef* qt_core_c_QCharRef_operator_assign_int(QCharRef* this_ptr, int rc);
QT_CORE_C_EXPORT QCharRef* qt_core_c_QCharRef_operator_assign_short(QCharRef* this_ptr, short rc);
QT_CORE_C_EXPORT QCharRef* qt_core_c_QCharRef_operator_assign_unsigned_char(QCharRef* this_ptr, unsigned char c);
QT_CORE_C_EXPORT QCharRef* qt_core_c_QCharRef_operator_assign_unsigned_int(QCharRef* this_ptr, unsigned int rc);
QT_CORE_C_EXPORT QCharRef* qt_core_c_QCharRef_operator_assign_unsigned_short(QCharRef* this_ptr, unsigned short rc);
QT_CORE_C_EXPORT unsigned char qt_core_c_QCharRef_row(const QCharRef* this_ptr);
QT_CORE_C_EXPORT void qt_core_c_QCharRef_setCell(QCharRef* this_ptr, unsigned char cell);
QT_CORE_C_EXPORT void qt_core_c_QCharRef_setRow(QCharRef* this_ptr, unsigned char row);
QT_CORE_C_EXPORT char qt_core_c_QCharRef_toLatin1(const QCharRef* this_ptr);
QT_CORE_C_EXPORT void qt_core_c_QCharRef_toLower_to_output(const QCharRef* this_ptr, QChar* output);
QT_CORE_C_EXPORT void qt_core_c_QCharRef_toTitleCase_to_output(const QCharRef* this_ptr, QChar* output);
QT_CORE_C_EXPORT void qt_core_c_QCharRef_toUpper_to_output(const QCharRef* this_ptr, QChar* output);
QT_CORE_C_EXPORT unsigned short* qt_core_c_QCharRef_unicode(QCharRef* this_ptr);
QT_CORE_C_EXPORT unsigned short qt_core_c_QCharRef_unicode_const(const QCharRef* this_ptr);

} // extern "C"

#endif // QT_CORE_C_QCHARREF_H
