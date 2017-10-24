#ifndef QT_CORE_C_QTEXTBOUNDARYFINDER_H
#define QT_CORE_C_QTEXTBOUNDARYFINDER_H

#include "qt_core_c_global.h"

extern "C" {

QT_CORE_C_EXPORT unsigned int qt_core_c_QTextBoundaryFinder_boundaryReasons(const QTextBoundaryFinder* this_ptr);
QT_CORE_C_EXPORT void qt_core_c_QTextBoundaryFinder_constructor_no_args(QTextBoundaryFinder* output);
QT_CORE_C_EXPORT void qt_core_c_QTextBoundaryFinder_constructor_other(const QTextBoundaryFinder* other, QTextBoundaryFinder* output);
QT_CORE_C_EXPORT void qt_core_c_QTextBoundaryFinder_constructor_type_chars_length(QTextBoundaryFinder::BoundaryType type, const QChar* chars, int length, QTextBoundaryFinder* output);
QT_CORE_C_EXPORT void qt_core_c_QTextBoundaryFinder_constructor_type_chars_length_buffer(QTextBoundaryFinder::BoundaryType type, const QChar* chars, int length, unsigned char* buffer, QTextBoundaryFinder* output);
QT_CORE_C_EXPORT void qt_core_c_QTextBoundaryFinder_constructor_type_chars_length_buffer_bufferSize(QTextBoundaryFinder::BoundaryType type, const QChar* chars, int length, unsigned char* buffer, int bufferSize, QTextBoundaryFinder* output);
QT_CORE_C_EXPORT void qt_core_c_QTextBoundaryFinder_constructor_type_string(QTextBoundaryFinder::BoundaryType type, const QString* string, QTextBoundaryFinder* output);
QT_CORE_C_EXPORT void qt_core_c_QTextBoundaryFinder_destructor(QTextBoundaryFinder* this_ptr);
QT_CORE_C_EXPORT bool qt_core_c_QTextBoundaryFinder_isAtBoundary(const QTextBoundaryFinder* this_ptr);
QT_CORE_C_EXPORT bool qt_core_c_QTextBoundaryFinder_isValid(const QTextBoundaryFinder* this_ptr);
QT_CORE_C_EXPORT QTextBoundaryFinder* qt_core_c_QTextBoundaryFinder_operator_assign(QTextBoundaryFinder* this_ptr, const QTextBoundaryFinder* other);
QT_CORE_C_EXPORT int qt_core_c_QTextBoundaryFinder_position(const QTextBoundaryFinder* this_ptr);
QT_CORE_C_EXPORT void qt_core_c_QTextBoundaryFinder_setPosition(QTextBoundaryFinder* this_ptr, int position);
QT_CORE_C_EXPORT void qt_core_c_QTextBoundaryFinder_string_to_output(const QTextBoundaryFinder* this_ptr, QString* output);
QT_CORE_C_EXPORT void qt_core_c_QTextBoundaryFinder_toEnd(QTextBoundaryFinder* this_ptr);
QT_CORE_C_EXPORT int qt_core_c_QTextBoundaryFinder_toNextBoundary(QTextBoundaryFinder* this_ptr);
QT_CORE_C_EXPORT int qt_core_c_QTextBoundaryFinder_toPreviousBoundary(QTextBoundaryFinder* this_ptr);
QT_CORE_C_EXPORT void qt_core_c_QTextBoundaryFinder_toStart(QTextBoundaryFinder* this_ptr);
QT_CORE_C_EXPORT QTextBoundaryFinder::BoundaryType qt_core_c_QTextBoundaryFinder_type(const QTextBoundaryFinder* this_ptr);

} // extern "C"

#endif // QT_CORE_C_QTEXTBOUNDARYFINDER_H
