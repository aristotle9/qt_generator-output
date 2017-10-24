#ifndef QT_CORE_C_QURL_H
#define QT_CORE_C_QURL_H

#include "qt_core_c_global.h"

extern "C" {

QT_CORE_C_EXPORT QDataStream* qt_core_c_QUrl_G_operator_shl(QDataStream* arg1, const QUrl* arg2);
QT_CORE_C_EXPORT void qt_core_c_QUrl_G_operator_shl_to_output(const QDebug* arg1, const QUrl* arg2, QDebug* output);
QT_CORE_C_EXPORT QDataStream* qt_core_c_QUrl_G_operator_shr(QDataStream* arg1, QUrl* arg2);
QT_CORE_C_EXPORT unsigned int qt_core_c_QUrl_G_qHash_url(const QUrl* url);
QT_CORE_C_EXPORT unsigned int qt_core_c_QUrl_G_qHash_url_seed(const QUrl* url, unsigned int seed);
QT_CORE_C_EXPORT void qt_core_c_QUrl_G_swap(QUrl* value1, QUrl* value2);
QT_CORE_C_EXPORT void qt_core_c_QUrl_adjusted_to_output(const QUrl* this_ptr, const QUrlTwoFlags< QUrl::UrlFormattingOption, QUrl::ComponentFormattingOption >* options, QUrl* output);
QT_CORE_C_EXPORT void qt_core_c_QUrl_authority_to_output_no_args(const QUrl* this_ptr, QString* output);
QT_CORE_C_EXPORT void qt_core_c_QUrl_authority_to_output_options(const QUrl* this_ptr, unsigned int options, QString* output);
QT_CORE_C_EXPORT void qt_core_c_QUrl_clear(QUrl* this_ptr);
QT_CORE_C_EXPORT void qt_core_c_QUrl_constructor_copy(const QUrl* copy, QUrl* output);
QT_CORE_C_EXPORT void qt_core_c_QUrl_constructor_no_args(QUrl* output);
QT_CORE_C_EXPORT void qt_core_c_QUrl_constructor_url(const QString* url, QUrl* output);
QT_CORE_C_EXPORT void qt_core_c_QUrl_constructor_url_mode(const QString* url, QUrl::ParsingMode mode, QUrl* output);
QT_CORE_C_EXPORT void qt_core_c_QUrl_destructor(QUrl* this_ptr);
QT_CORE_C_EXPORT void qt_core_c_QUrl_errorString_to_output(const QUrl* this_ptr, QString* output);
QT_CORE_C_EXPORT void qt_core_c_QUrl_fileName_to_output_no_args(const QUrl* this_ptr, QString* output);
QT_CORE_C_EXPORT void qt_core_c_QUrl_fileName_to_output_options(const QUrl* this_ptr, unsigned int options, QString* output);
QT_CORE_C_EXPORT void qt_core_c_QUrl_fragment_to_output_no_args(const QUrl* this_ptr, QString* output);
QT_CORE_C_EXPORT void qt_core_c_QUrl_fragment_to_output_options(const QUrl* this_ptr, unsigned int options, QString* output);
QT_CORE_C_EXPORT void qt_core_c_QUrl_fromAce_to_output(const QByteArray* arg1, QString* output);
QT_CORE_C_EXPORT void qt_core_c_QUrl_fromEncoded_to_output_url(const QByteArray* url, QUrl* output);
QT_CORE_C_EXPORT void qt_core_c_QUrl_fromEncoded_to_output_url_mode(const QByteArray* url, QUrl::ParsingMode mode, QUrl* output);
QT_CORE_C_EXPORT void qt_core_c_QUrl_fromLocalFile_to_output(const QString* localfile, QUrl* output);
QT_CORE_C_EXPORT void qt_core_c_QUrl_fromPercentEncoding_to_output(const QByteArray* arg1, QString* output);
QT_CORE_C_EXPORT void qt_core_c_QUrl_fromStringList_to_output_uris(const QStringList* uris, QList< QUrl >* output);
QT_CORE_C_EXPORT void qt_core_c_QUrl_fromStringList_to_output_uris_mode(const QStringList* uris, QUrl::ParsingMode mode, QList< QUrl >* output);
QT_CORE_C_EXPORT void qt_core_c_QUrl_fromUserInput_to_output_userInput(const QString* userInput, QUrl* output);
QT_CORE_C_EXPORT void qt_core_c_QUrl_fromUserInput_to_output_userInput_workingDirectory(const QString* userInput, const QString* workingDirectory, QUrl* output);
QT_CORE_C_EXPORT void qt_core_c_QUrl_fromUserInput_to_output_userInput_workingDirectory_options(const QString* userInput, const QString* workingDirectory, unsigned int options, QUrl* output);
QT_CORE_C_EXPORT bool qt_core_c_QUrl_hasFragment(const QUrl* this_ptr);
QT_CORE_C_EXPORT bool qt_core_c_QUrl_hasQuery(const QUrl* this_ptr);
QT_CORE_C_EXPORT void qt_core_c_QUrl_host_to_output_arg1(const QUrl* this_ptr, unsigned int arg1, QString* output);
QT_CORE_C_EXPORT void qt_core_c_QUrl_host_to_output_no_args(const QUrl* this_ptr, QString* output);
QT_CORE_C_EXPORT void qt_core_c_QUrl_idnWhitelist_to_output(QStringList* output);
QT_CORE_C_EXPORT bool qt_core_c_QUrl_isEmpty(const QUrl* this_ptr);
QT_CORE_C_EXPORT bool qt_core_c_QUrl_isLocalFile(const QUrl* this_ptr);
QT_CORE_C_EXPORT bool qt_core_c_QUrl_isParentOf(const QUrl* this_ptr, const QUrl* url);
QT_CORE_C_EXPORT bool qt_core_c_QUrl_isRelative(const QUrl* this_ptr);
QT_CORE_C_EXPORT bool qt_core_c_QUrl_isValid(const QUrl* this_ptr);
QT_CORE_C_EXPORT bool qt_core_c_QUrl_matches(const QUrl* this_ptr, const QUrl* url, const QUrlTwoFlags< QUrl::UrlFormattingOption, QUrl::ComponentFormattingOption >* options);
QT_CORE_C_EXPORT QUrl* qt_core_c_QUrl_operator_assign_copy(QUrl* this_ptr, const QUrl* copy);
QT_CORE_C_EXPORT QUrl* qt_core_c_QUrl_operator_assign_url(QUrl* this_ptr, const QString* url);
QT_CORE_C_EXPORT bool qt_core_c_QUrl_operator_eq(const QUrl* this_ptr, const QUrl* url);
QT_CORE_C_EXPORT bool qt_core_c_QUrl_operator_lt(const QUrl* this_ptr, const QUrl* url);
QT_CORE_C_EXPORT bool qt_core_c_QUrl_operator_neq(const QUrl* this_ptr, const QUrl* url);
QT_CORE_C_EXPORT void qt_core_c_QUrl_password_to_output_arg1(const QUrl* this_ptr, unsigned int arg1, QString* output);
QT_CORE_C_EXPORT void qt_core_c_QUrl_password_to_output_no_args(const QUrl* this_ptr, QString* output);
QT_CORE_C_EXPORT void qt_core_c_QUrl_path_to_output_no_args(const QUrl* this_ptr, QString* output);
QT_CORE_C_EXPORT void qt_core_c_QUrl_path_to_output_options(const QUrl* this_ptr, unsigned int options, QString* output);
QT_CORE_C_EXPORT int qt_core_c_QUrl_port_defaultPort(const QUrl* this_ptr, int defaultPort);
QT_CORE_C_EXPORT int qt_core_c_QUrl_port_no_args(const QUrl* this_ptr);
QT_CORE_C_EXPORT void qt_core_c_QUrl_query_to_output_arg1(const QUrl* this_ptr, unsigned int arg1, QString* output);
QT_CORE_C_EXPORT void qt_core_c_QUrl_query_to_output_no_args(const QUrl* this_ptr, QString* output);
QT_CORE_C_EXPORT void qt_core_c_QUrl_resolved_to_output(const QUrl* this_ptr, const QUrl* relative, QUrl* output);
QT_CORE_C_EXPORT void qt_core_c_QUrl_scheme_to_output(const QUrl* this_ptr, QString* output);
QT_CORE_C_EXPORT void qt_core_c_QUrl_setAuthority_authority(QUrl* this_ptr, const QString* authority);
QT_CORE_C_EXPORT void qt_core_c_QUrl_setAuthority_authority_mode(QUrl* this_ptr, const QString* authority, QUrl::ParsingMode mode);
QT_CORE_C_EXPORT void qt_core_c_QUrl_setFragment_fragment(QUrl* this_ptr, const QString* fragment);
QT_CORE_C_EXPORT void qt_core_c_QUrl_setFragment_fragment_mode(QUrl* this_ptr, const QString* fragment, QUrl::ParsingMode mode);
QT_CORE_C_EXPORT void qt_core_c_QUrl_setHost_host(QUrl* this_ptr, const QString* host);
QT_CORE_C_EXPORT void qt_core_c_QUrl_setHost_host_mode(QUrl* this_ptr, const QString* host, QUrl::ParsingMode mode);
QT_CORE_C_EXPORT void qt_core_c_QUrl_setIdnWhitelist(const QStringList* arg1);
QT_CORE_C_EXPORT void qt_core_c_QUrl_setPassword_password(QUrl* this_ptr, const QString* password);
QT_CORE_C_EXPORT void qt_core_c_QUrl_setPassword_password_mode(QUrl* this_ptr, const QString* password, QUrl::ParsingMode mode);
QT_CORE_C_EXPORT void qt_core_c_QUrl_setPath_path(QUrl* this_ptr, const QString* path);
QT_CORE_C_EXPORT void qt_core_c_QUrl_setPath_path_mode(QUrl* this_ptr, const QString* path, QUrl::ParsingMode mode);
QT_CORE_C_EXPORT void qt_core_c_QUrl_setPort(QUrl* this_ptr, int port);
QT_CORE_C_EXPORT void qt_core_c_QUrl_setQuery_QString(QUrl* this_ptr, const QString* query);
QT_CORE_C_EXPORT void qt_core_c_QUrl_setQuery_QString_QUrl_ParsingMode(QUrl* this_ptr, const QString* query, QUrl::ParsingMode mode);
QT_CORE_C_EXPORT void qt_core_c_QUrl_setQuery_QUrlQuery(QUrl* this_ptr, const QUrlQuery* query);
QT_CORE_C_EXPORT void qt_core_c_QUrl_setScheme(QUrl* this_ptr, const QString* scheme);
QT_CORE_C_EXPORT void qt_core_c_QUrl_setUrl_url(QUrl* this_ptr, const QString* url);
QT_CORE_C_EXPORT void qt_core_c_QUrl_setUrl_url_mode(QUrl* this_ptr, const QString* url, QUrl::ParsingMode mode);
QT_CORE_C_EXPORT void qt_core_c_QUrl_setUserInfo_userInfo(QUrl* this_ptr, const QString* userInfo);
QT_CORE_C_EXPORT void qt_core_c_QUrl_setUserInfo_userInfo_mode(QUrl* this_ptr, const QString* userInfo, QUrl::ParsingMode mode);
QT_CORE_C_EXPORT void qt_core_c_QUrl_setUserName_userName(QUrl* this_ptr, const QString* userName);
QT_CORE_C_EXPORT void qt_core_c_QUrl_setUserName_userName_mode(QUrl* this_ptr, const QString* userName, QUrl::ParsingMode mode);
QT_CORE_C_EXPORT void qt_core_c_QUrl_swap(QUrl* this_ptr, QUrl* other);
QT_CORE_C_EXPORT void qt_core_c_QUrl_toAce_to_output(const QString* arg1, QByteArray* output);
QT_CORE_C_EXPORT void qt_core_c_QUrl_toDisplayString_to_output_no_args(const QUrl* this_ptr, QString* output);
QT_CORE_C_EXPORT void qt_core_c_QUrl_toDisplayString_to_output_options(const QUrl* this_ptr, const QUrlTwoFlags< QUrl::UrlFormattingOption, QUrl::ComponentFormattingOption >* options, QString* output);
QT_CORE_C_EXPORT void qt_core_c_QUrl_toEncoded_to_output_no_args(const QUrl* this_ptr, QByteArray* output);
QT_CORE_C_EXPORT void qt_core_c_QUrl_toEncoded_to_output_options(const QUrl* this_ptr, const QUrlTwoFlags< QUrl::UrlFormattingOption, QUrl::ComponentFormattingOption >* options, QByteArray* output);
QT_CORE_C_EXPORT void qt_core_c_QUrl_toLocalFile_to_output(const QUrl* this_ptr, QString* output);
QT_CORE_C_EXPORT void qt_core_c_QUrl_toPercentEncoding_to_output_arg1(const QString* arg1, QByteArray* output);
QT_CORE_C_EXPORT void qt_core_c_QUrl_toPercentEncoding_to_output_arg1_exclude(const QString* arg1, const QByteArray* exclude, QByteArray* output);
QT_CORE_C_EXPORT void qt_core_c_QUrl_toPercentEncoding_to_output_arg1_exclude_include(const QString* arg1, const QByteArray* exclude, const QByteArray* include, QByteArray* output);
QT_CORE_C_EXPORT void qt_core_c_QUrl_toStringList_to_output_uris(const QList< QUrl >* uris, QStringList* output);
QT_CORE_C_EXPORT void qt_core_c_QUrl_toStringList_to_output_uris_options(const QList< QUrl >* uris, const QUrlTwoFlags< QUrl::UrlFormattingOption, QUrl::ComponentFormattingOption >* options, QStringList* output);
QT_CORE_C_EXPORT void qt_core_c_QUrl_toString_to_output_no_args(const QUrl* this_ptr, QString* output);
QT_CORE_C_EXPORT void qt_core_c_QUrl_toString_to_output_options(const QUrl* this_ptr, const QUrlTwoFlags< QUrl::UrlFormattingOption, QUrl::ComponentFormattingOption >* options, QString* output);
QT_CORE_C_EXPORT void qt_core_c_QUrl_topLevelDomain_to_output_no_args(const QUrl* this_ptr, QString* output);
QT_CORE_C_EXPORT void qt_core_c_QUrl_topLevelDomain_to_output_options(const QUrl* this_ptr, unsigned int options, QString* output);
QT_CORE_C_EXPORT void qt_core_c_QUrl_url_to_output_no_args(const QUrl* this_ptr, QString* output);
QT_CORE_C_EXPORT void qt_core_c_QUrl_url_to_output_options(const QUrl* this_ptr, const QUrlTwoFlags< QUrl::UrlFormattingOption, QUrl::ComponentFormattingOption >* options, QString* output);
QT_CORE_C_EXPORT void qt_core_c_QUrl_userInfo_to_output_no_args(const QUrl* this_ptr, QString* output);
QT_CORE_C_EXPORT void qt_core_c_QUrl_userInfo_to_output_options(const QUrl* this_ptr, unsigned int options, QString* output);
QT_CORE_C_EXPORT void qt_core_c_QUrl_userName_to_output_no_args(const QUrl* this_ptr, QString* output);
QT_CORE_C_EXPORT void qt_core_c_QUrl_userName_to_output_options(const QUrl* this_ptr, unsigned int options, QString* output);

} // extern "C"

#endif // QT_CORE_C_QURL_H
