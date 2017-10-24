#ifndef QT_CORE_C_QSTRINGLIST_H
#define QT_CORE_C_QSTRINGLIST_H

#include "qt_core_c_global.h"

extern "C" {

QT_CORE_C_EXPORT void qt_core_c_QStringList_G_operator_add_to_output(const QList< QString >* one, const QStringList* other, QStringList* output);
QT_CORE_C_EXPORT QList< QString >* qt_core_c_QStringList_G_static_cast_QList_QString_ptr(QStringList* ptr);
QT_CORE_C_EXPORT QStringList* qt_core_c_QStringList_G_static_cast_QStringList_ptr(QList< QString >* ptr);
QT_CORE_C_EXPORT void qt_core_c_QStringList_constructor_i(const QString* i, QStringList* output);
QT_CORE_C_EXPORT void qt_core_c_QStringList_constructor_l(const QList< QString >* l, QStringList* output);
QT_CORE_C_EXPORT void qt_core_c_QStringList_constructor_no_args(QStringList* output);
QT_CORE_C_EXPORT bool qt_core_c_QStringList_contains_str(const QStringList* this_ptr, const QString* str);
QT_CORE_C_EXPORT bool qt_core_c_QStringList_contains_str_cs(const QStringList* this_ptr, const QString* str, const Qt::CaseSensitivity* cs);
QT_CORE_C_EXPORT void qt_core_c_QStringList_destructor(QStringList* this_ptr);
QT_CORE_C_EXPORT int qt_core_c_QStringList_indexOf_QRegExp_ref(const QStringList* this_ptr, QRegExp* rx);
QT_CORE_C_EXPORT int qt_core_c_QStringList_indexOf_QRegExp_ref_int(const QStringList* this_ptr, QRegExp* rx, int from);
QT_CORE_C_EXPORT int qt_core_c_QStringList_indexOf_const_QRegExp_ref(const QStringList* this_ptr, const QRegExp* rx);
QT_CORE_C_EXPORT int qt_core_c_QStringList_indexOf_const_QRegExp_ref_int(const QStringList* this_ptr, const QRegExp* rx, int from);
QT_CORE_C_EXPORT int qt_core_c_QStringList_indexOf_const_QRegularExpression_ref(const QStringList* this_ptr, const QRegularExpression* re);
QT_CORE_C_EXPORT int qt_core_c_QStringList_indexOf_const_QRegularExpression_ref_int(const QStringList* this_ptr, const QRegularExpression* re, int from);
QT_CORE_C_EXPORT int qt_core_c_QStringList_lastIndexOf_QRegExp_ref(const QStringList* this_ptr, QRegExp* rx);
QT_CORE_C_EXPORT int qt_core_c_QStringList_lastIndexOf_QRegExp_ref_int(const QStringList* this_ptr, QRegExp* rx, int from);
QT_CORE_C_EXPORT int qt_core_c_QStringList_lastIndexOf_const_QRegExp_ref(const QStringList* this_ptr, const QRegExp* rx);
QT_CORE_C_EXPORT int qt_core_c_QStringList_lastIndexOf_const_QRegExp_ref_int(const QStringList* this_ptr, const QRegExp* rx, int from);
QT_CORE_C_EXPORT int qt_core_c_QStringList_lastIndexOf_const_QRegularExpression_ref(const QStringList* this_ptr, const QRegularExpression* re);
QT_CORE_C_EXPORT int qt_core_c_QStringList_lastIndexOf_const_QRegularExpression_ref_int(const QStringList* this_ptr, const QRegularExpression* re, int from);
QT_CORE_C_EXPORT void qt_core_c_QStringList_operator_add_to_output(const QStringList* this_ptr, const QStringList* other, QStringList* output);
QT_CORE_C_EXPORT QStringList* qt_core_c_QStringList_operator_assign(QStringList* this_ptr, const QList< QString >* other);
QT_CORE_C_EXPORT QStringList* qt_core_c_QStringList_operator_shl_QList_QString(QStringList* this_ptr, const QList< QString >* l);
QT_CORE_C_EXPORT QStringList* qt_core_c_QStringList_operator_shl_QString(QStringList* this_ptr, const QString* str);
QT_CORE_C_EXPORT QStringList* qt_core_c_QStringList_operator_shl_QStringList(QStringList* this_ptr, const QStringList* l);

} // extern "C"

#endif // QT_CORE_C_QSTRINGLIST_H
