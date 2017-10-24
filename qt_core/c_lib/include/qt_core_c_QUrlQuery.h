#ifndef QT_CORE_C_QURLQUERY_H
#define QT_CORE_C_QURLQUERY_H

#include "qt_core_c_global.h"

extern "C" {

QT_CORE_C_EXPORT unsigned int qt_core_c_QUrlQuery_G_qHash_key(const QUrlQuery* key);
QT_CORE_C_EXPORT unsigned int qt_core_c_QUrlQuery_G_qHash_key_seed(const QUrlQuery* key, unsigned int seed);
QT_CORE_C_EXPORT void qt_core_c_QUrlQuery_G_swap(QUrlQuery* value1, QUrlQuery* value2);
QT_CORE_C_EXPORT void qt_core_c_QUrlQuery_addQueryItem(QUrlQuery* this_ptr, const QString* key, const QString* value);
QT_CORE_C_EXPORT void qt_core_c_QUrlQuery_clear(QUrlQuery* this_ptr);
QT_CORE_C_EXPORT void qt_core_c_QUrlQuery_constructor_no_args(QUrlQuery* output);
QT_CORE_C_EXPORT void qt_core_c_QUrlQuery_constructor_other(const QUrlQuery* other, QUrlQuery* output);
QT_CORE_C_EXPORT void qt_core_c_QUrlQuery_constructor_queryString(const QString* queryString, QUrlQuery* output);
QT_CORE_C_EXPORT void qt_core_c_QUrlQuery_constructor_url(const QUrl* url, QUrlQuery* output);
QT_CORE_C_EXPORT void qt_core_c_QUrlQuery_defaultQueryPairDelimiter_to_output(QChar* output);
QT_CORE_C_EXPORT void qt_core_c_QUrlQuery_defaultQueryValueDelimiter_to_output(QChar* output);
QT_CORE_C_EXPORT void qt_core_c_QUrlQuery_destructor(QUrlQuery* this_ptr);
QT_CORE_C_EXPORT bool qt_core_c_QUrlQuery_hasQueryItem(const QUrlQuery* this_ptr, const QString* key);
QT_CORE_C_EXPORT bool qt_core_c_QUrlQuery_isEmpty(const QUrlQuery* this_ptr);
QT_CORE_C_EXPORT QUrlQuery* qt_core_c_QUrlQuery_operator_assign(QUrlQuery* this_ptr, const QUrlQuery* other);
QT_CORE_C_EXPORT bool qt_core_c_QUrlQuery_operator_eq(const QUrlQuery* this_ptr, const QUrlQuery* other);
QT_CORE_C_EXPORT bool qt_core_c_QUrlQuery_operator_neq(const QUrlQuery* this_ptr, const QUrlQuery* other);
QT_CORE_C_EXPORT void qt_core_c_QUrlQuery_queryPairDelimiter_to_output(const QUrlQuery* this_ptr, QChar* output);
QT_CORE_C_EXPORT void qt_core_c_QUrlQuery_queryValueDelimiter_to_output(const QUrlQuery* this_ptr, QChar* output);
QT_CORE_C_EXPORT void qt_core_c_QUrlQuery_removeAllQueryItems(QUrlQuery* this_ptr, const QString* key);
QT_CORE_C_EXPORT void qt_core_c_QUrlQuery_removeQueryItem(QUrlQuery* this_ptr, const QString* key);
QT_CORE_C_EXPORT void qt_core_c_QUrlQuery_setQuery(QUrlQuery* this_ptr, const QString* queryString);
QT_CORE_C_EXPORT void qt_core_c_QUrlQuery_setQueryDelimiters(QUrlQuery* this_ptr, const QChar* valueDelimiter, const QChar* pairDelimiter);
QT_CORE_C_EXPORT void qt_core_c_QUrlQuery_setQueryItems(QUrlQuery* this_ptr, const QList< QPair< QString, QString > >* query);
QT_CORE_C_EXPORT void qt_core_c_QUrlQuery_swap(QUrlQuery* this_ptr, QUrlQuery* other);

} // extern "C"

#endif // QT_CORE_C_QURLQUERY_H
