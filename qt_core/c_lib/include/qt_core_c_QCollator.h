#ifndef QT_CORE_C_QCOLLATOR_H
#define QT_CORE_C_QCOLLATOR_H

#include "qt_core_c_global.h"

extern "C" {

QT_CORE_C_EXPORT bool qt_core_c_QCollator_G_operator_lt(const QCollatorSortKey* lhs, const QCollatorSortKey* rhs);
QT_CORE_C_EXPORT void qt_core_c_QCollator_G_swap_QCollatorSortKey_QCollatorSortKey(QCollatorSortKey* value1, QCollatorSortKey* value2);
QT_CORE_C_EXPORT void qt_core_c_QCollator_G_swap_QCollator_QCollator(QCollator* value1, QCollator* value2);
QT_CORE_C_EXPORT int qt_core_c_QCollator_compare_QChar_int_QChar_int(const QCollator* this_ptr, const QChar* s1, int len1, const QChar* s2, int len2);
QT_CORE_C_EXPORT int qt_core_c_QCollator_compare_QStringRef_QStringRef(const QCollator* this_ptr, const QStringRef* s1, const QStringRef* s2);
QT_CORE_C_EXPORT int qt_core_c_QCollator_compare_QString_QString(const QCollator* this_ptr, const QString* s1, const QString* s2);
QT_CORE_C_EXPORT void qt_core_c_QCollator_constructor_arg1(const QCollator* arg1, QCollator* output);
QT_CORE_C_EXPORT void qt_core_c_QCollator_constructor_locale(const QLocale* locale, QCollator* output);
QT_CORE_C_EXPORT void qt_core_c_QCollator_constructor_no_args(QCollator* output);
QT_CORE_C_EXPORT void qt_core_c_QCollator_destructor(QCollator* this_ptr);
QT_CORE_C_EXPORT bool qt_core_c_QCollator_ignorePunctuation(const QCollator* this_ptr);
QT_CORE_C_EXPORT void qt_core_c_QCollator_locale_to_output(const QCollator* this_ptr, QLocale* output);
QT_CORE_C_EXPORT bool qt_core_c_QCollator_numericMode(const QCollator* this_ptr);
QT_CORE_C_EXPORT QCollator* qt_core_c_QCollator_operator_assign(QCollator* this_ptr, const QCollator* arg1);
QT_CORE_C_EXPORT bool qt_core_c_QCollator_operator_call(const QCollator* this_ptr, const QString* s1, const QString* s2);
QT_CORE_C_EXPORT void qt_core_c_QCollator_setCaseSensitivity(QCollator* this_ptr, const Qt::CaseSensitivity* cs);
QT_CORE_C_EXPORT void qt_core_c_QCollator_setIgnorePunctuation(QCollator* this_ptr, bool on);
QT_CORE_C_EXPORT void qt_core_c_QCollator_setLocale(QCollator* this_ptr, const QLocale* locale);
QT_CORE_C_EXPORT void qt_core_c_QCollator_setNumericMode(QCollator* this_ptr, bool on);
QT_CORE_C_EXPORT void qt_core_c_QCollator_sortKey_to_output(const QCollator* this_ptr, const QString* string, QCollatorSortKey* output);
QT_CORE_C_EXPORT void qt_core_c_QCollator_swap(QCollator* this_ptr, QCollator* other);

} // extern "C"

#endif // QT_CORE_C_QCOLLATOR_H
