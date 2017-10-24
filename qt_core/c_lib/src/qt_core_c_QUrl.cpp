#include "qt_core_c_QUrl.h"

QDataStream* qt_core_c_QUrl_G_operator_shl(QDataStream* arg1, const QUrl* arg2) {
  return &operator<<(*arg1, *arg2);
}

void qt_core_c_QUrl_G_operator_shl_to_output(const QDebug* arg1, const QUrl* arg2, QDebug* output) {
  new(output) QDebug(operator<<(*arg1, *arg2));
}

QDataStream* qt_core_c_QUrl_G_operator_shr(QDataStream* arg1, QUrl* arg2) {
  return &operator>>(*arg1, *arg2);
}

unsigned int qt_core_c_QUrl_G_qHash_url(const QUrl* url) {
  return qHash(*url);
}

unsigned int qt_core_c_QUrl_G_qHash_url_seed(const QUrl* url, unsigned int seed) {
  return qHash(*url, seed);
}

void qt_core_c_QUrl_G_swap(QUrl* value1, QUrl* value2) {
  swap(*value1, *value2);
}

void qt_core_c_QUrl_adjusted_to_output(const QUrl* this_ptr, const QUrlTwoFlags< QUrl::UrlFormattingOption, QUrl::ComponentFormattingOption >* options, QUrl* output) {
  new(output) QUrl(this_ptr->adjusted(*options));
}

void qt_core_c_QUrl_authority_to_output_no_args(const QUrl* this_ptr, QString* output) {
  new(output) QString(this_ptr->authority());
}

void qt_core_c_QUrl_authority_to_output_options(const QUrl* this_ptr, unsigned int options, QString* output) {
  new(output) QString(this_ptr->authority(QFlags< QUrl::ComponentFormattingOption >(options)));
}

void qt_core_c_QUrl_clear(QUrl* this_ptr) {
  this_ptr->clear();
}

void qt_core_c_QUrl_constructor_copy(const QUrl* copy, QUrl* output) {
  new(output) QUrl(*copy);
}

void qt_core_c_QUrl_constructor_no_args(QUrl* output) {
  new(output) QUrl();
}

void qt_core_c_QUrl_constructor_url(const QString* url, QUrl* output) {
  new(output) QUrl(*url);
}

void qt_core_c_QUrl_constructor_url_mode(const QString* url, QUrl::ParsingMode mode, QUrl* output) {
  new(output) QUrl(*url, mode);
}

void qt_core_c_QUrl_destructor(QUrl* this_ptr) {
  qt_core_c_call_destructor(this_ptr);
}

void qt_core_c_QUrl_errorString_to_output(const QUrl* this_ptr, QString* output) {
  new(output) QString(this_ptr->errorString());
}

void qt_core_c_QUrl_fileName_to_output_no_args(const QUrl* this_ptr, QString* output) {
  new(output) QString(this_ptr->fileName());
}

void qt_core_c_QUrl_fileName_to_output_options(const QUrl* this_ptr, unsigned int options, QString* output) {
  new(output) QString(this_ptr->fileName(QFlags< QUrl::ComponentFormattingOption >(options)));
}

void qt_core_c_QUrl_fragment_to_output_no_args(const QUrl* this_ptr, QString* output) {
  new(output) QString(this_ptr->fragment());
}

void qt_core_c_QUrl_fragment_to_output_options(const QUrl* this_ptr, unsigned int options, QString* output) {
  new(output) QString(this_ptr->fragment(QFlags< QUrl::ComponentFormattingOption >(options)));
}

void qt_core_c_QUrl_fromAce_to_output(const QByteArray* arg1, QString* output) {
  new(output) QString(QUrl::fromAce(*arg1));
}

void qt_core_c_QUrl_fromEncoded_to_output_url(const QByteArray* url, QUrl* output) {
  new(output) QUrl(QUrl::fromEncoded(*url));
}

void qt_core_c_QUrl_fromEncoded_to_output_url_mode(const QByteArray* url, QUrl::ParsingMode mode, QUrl* output) {
  new(output) QUrl(QUrl::fromEncoded(*url, mode));
}

void qt_core_c_QUrl_fromLocalFile_to_output(const QString* localfile, QUrl* output) {
  new(output) QUrl(QUrl::fromLocalFile(*localfile));
}

void qt_core_c_QUrl_fromPercentEncoding_to_output(const QByteArray* arg1, QString* output) {
  new(output) QString(QUrl::fromPercentEncoding(*arg1));
}

void qt_core_c_QUrl_fromStringList_to_output_uris(const QStringList* uris, QList< QUrl >* output) {
  new(output) QList< QUrl >(QUrl::fromStringList(*uris));
}

void qt_core_c_QUrl_fromStringList_to_output_uris_mode(const QStringList* uris, QUrl::ParsingMode mode, QList< QUrl >* output) {
  new(output) QList< QUrl >(QUrl::fromStringList(*uris, mode));
}

void qt_core_c_QUrl_fromUserInput_to_output_userInput(const QString* userInput, QUrl* output) {
  new(output) QUrl(QUrl::fromUserInput(*userInput));
}

void qt_core_c_QUrl_fromUserInput_to_output_userInput_workingDirectory(const QString* userInput, const QString* workingDirectory, QUrl* output) {
  new(output) QUrl(QUrl::fromUserInput(*userInput, *workingDirectory));
}

void qt_core_c_QUrl_fromUserInput_to_output_userInput_workingDirectory_options(const QString* userInput, const QString* workingDirectory, unsigned int options, QUrl* output) {
  new(output) QUrl(QUrl::fromUserInput(*userInput, *workingDirectory, QFlags< QUrl::UserInputResolutionOption >(options)));
}

bool qt_core_c_QUrl_hasFragment(const QUrl* this_ptr) {
  return this_ptr->hasFragment();
}

bool qt_core_c_QUrl_hasQuery(const QUrl* this_ptr) {
  return this_ptr->hasQuery();
}

void qt_core_c_QUrl_host_to_output_arg1(const QUrl* this_ptr, unsigned int arg1, QString* output) {
  new(output) QString(this_ptr->host(QFlags< QUrl::ComponentFormattingOption >(arg1)));
}

void qt_core_c_QUrl_host_to_output_no_args(const QUrl* this_ptr, QString* output) {
  new(output) QString(this_ptr->host());
}

void qt_core_c_QUrl_idnWhitelist_to_output(QStringList* output) {
  new(output) QStringList(QUrl::idnWhitelist());
}

bool qt_core_c_QUrl_isEmpty(const QUrl* this_ptr) {
  return this_ptr->isEmpty();
}

bool qt_core_c_QUrl_isLocalFile(const QUrl* this_ptr) {
  return this_ptr->isLocalFile();
}

bool qt_core_c_QUrl_isParentOf(const QUrl* this_ptr, const QUrl* url) {
  return this_ptr->isParentOf(*url);
}

bool qt_core_c_QUrl_isRelative(const QUrl* this_ptr) {
  return this_ptr->isRelative();
}

bool qt_core_c_QUrl_isValid(const QUrl* this_ptr) {
  return this_ptr->isValid();
}

bool qt_core_c_QUrl_matches(const QUrl* this_ptr, const QUrl* url, const QUrlTwoFlags< QUrl::UrlFormattingOption, QUrl::ComponentFormattingOption >* options) {
  return this_ptr->matches(*url, *options);
}

QUrl* qt_core_c_QUrl_operator_assign_copy(QUrl* this_ptr, const QUrl* copy) {
  return &this_ptr->operator=(*copy);
}

QUrl* qt_core_c_QUrl_operator_assign_url(QUrl* this_ptr, const QString* url) {
  return &this_ptr->operator=(*url);
}

bool qt_core_c_QUrl_operator_eq(const QUrl* this_ptr, const QUrl* url) {
  return this_ptr->operator==(*url);
}

bool qt_core_c_QUrl_operator_lt(const QUrl* this_ptr, const QUrl* url) {
  return this_ptr->operator<(*url);
}

bool qt_core_c_QUrl_operator_neq(const QUrl* this_ptr, const QUrl* url) {
  return this_ptr->operator!=(*url);
}

void qt_core_c_QUrl_password_to_output_arg1(const QUrl* this_ptr, unsigned int arg1, QString* output) {
  new(output) QString(this_ptr->password(QFlags< QUrl::ComponentFormattingOption >(arg1)));
}

void qt_core_c_QUrl_password_to_output_no_args(const QUrl* this_ptr, QString* output) {
  new(output) QString(this_ptr->password());
}

void qt_core_c_QUrl_path_to_output_no_args(const QUrl* this_ptr, QString* output) {
  new(output) QString(this_ptr->path());
}

void qt_core_c_QUrl_path_to_output_options(const QUrl* this_ptr, unsigned int options, QString* output) {
  new(output) QString(this_ptr->path(QFlags< QUrl::ComponentFormattingOption >(options)));
}

int qt_core_c_QUrl_port_defaultPort(const QUrl* this_ptr, int defaultPort) {
  return this_ptr->port(defaultPort);
}

int qt_core_c_QUrl_port_no_args(const QUrl* this_ptr) {
  return this_ptr->port();
}

void qt_core_c_QUrl_query_to_output_arg1(const QUrl* this_ptr, unsigned int arg1, QString* output) {
  new(output) QString(this_ptr->query(QFlags< QUrl::ComponentFormattingOption >(arg1)));
}

void qt_core_c_QUrl_query_to_output_no_args(const QUrl* this_ptr, QString* output) {
  new(output) QString(this_ptr->query());
}

void qt_core_c_QUrl_resolved_to_output(const QUrl* this_ptr, const QUrl* relative, QUrl* output) {
  new(output) QUrl(this_ptr->resolved(*relative));
}

void qt_core_c_QUrl_scheme_to_output(const QUrl* this_ptr, QString* output) {
  new(output) QString(this_ptr->scheme());
}

void qt_core_c_QUrl_setAuthority_authority(QUrl* this_ptr, const QString* authority) {
  this_ptr->setAuthority(*authority);
}

void qt_core_c_QUrl_setAuthority_authority_mode(QUrl* this_ptr, const QString* authority, QUrl::ParsingMode mode) {
  this_ptr->setAuthority(*authority, mode);
}

void qt_core_c_QUrl_setFragment_fragment(QUrl* this_ptr, const QString* fragment) {
  this_ptr->setFragment(*fragment);
}

void qt_core_c_QUrl_setFragment_fragment_mode(QUrl* this_ptr, const QString* fragment, QUrl::ParsingMode mode) {
  this_ptr->setFragment(*fragment, mode);
}

void qt_core_c_QUrl_setHost_host(QUrl* this_ptr, const QString* host) {
  this_ptr->setHost(*host);
}

void qt_core_c_QUrl_setHost_host_mode(QUrl* this_ptr, const QString* host, QUrl::ParsingMode mode) {
  this_ptr->setHost(*host, mode);
}

void qt_core_c_QUrl_setIdnWhitelist(const QStringList* arg1) {
  QUrl::setIdnWhitelist(*arg1);
}

void qt_core_c_QUrl_setPassword_password(QUrl* this_ptr, const QString* password) {
  this_ptr->setPassword(*password);
}

void qt_core_c_QUrl_setPassword_password_mode(QUrl* this_ptr, const QString* password, QUrl::ParsingMode mode) {
  this_ptr->setPassword(*password, mode);
}

void qt_core_c_QUrl_setPath_path(QUrl* this_ptr, const QString* path) {
  this_ptr->setPath(*path);
}

void qt_core_c_QUrl_setPath_path_mode(QUrl* this_ptr, const QString* path, QUrl::ParsingMode mode) {
  this_ptr->setPath(*path, mode);
}

void qt_core_c_QUrl_setPort(QUrl* this_ptr, int port) {
  this_ptr->setPort(port);
}

void qt_core_c_QUrl_setQuery_QString(QUrl* this_ptr, const QString* query) {
  this_ptr->setQuery(*query);
}

void qt_core_c_QUrl_setQuery_QString_QUrl_ParsingMode(QUrl* this_ptr, const QString* query, QUrl::ParsingMode mode) {
  this_ptr->setQuery(*query, mode);
}

void qt_core_c_QUrl_setQuery_QUrlQuery(QUrl* this_ptr, const QUrlQuery* query) {
  this_ptr->setQuery(*query);
}

void qt_core_c_QUrl_setScheme(QUrl* this_ptr, const QString* scheme) {
  this_ptr->setScheme(*scheme);
}

void qt_core_c_QUrl_setUrl_url(QUrl* this_ptr, const QString* url) {
  this_ptr->setUrl(*url);
}

void qt_core_c_QUrl_setUrl_url_mode(QUrl* this_ptr, const QString* url, QUrl::ParsingMode mode) {
  this_ptr->setUrl(*url, mode);
}

void qt_core_c_QUrl_setUserInfo_userInfo(QUrl* this_ptr, const QString* userInfo) {
  this_ptr->setUserInfo(*userInfo);
}

void qt_core_c_QUrl_setUserInfo_userInfo_mode(QUrl* this_ptr, const QString* userInfo, QUrl::ParsingMode mode) {
  this_ptr->setUserInfo(*userInfo, mode);
}

void qt_core_c_QUrl_setUserName_userName(QUrl* this_ptr, const QString* userName) {
  this_ptr->setUserName(*userName);
}

void qt_core_c_QUrl_setUserName_userName_mode(QUrl* this_ptr, const QString* userName, QUrl::ParsingMode mode) {
  this_ptr->setUserName(*userName, mode);
}

void qt_core_c_QUrl_swap(QUrl* this_ptr, QUrl* other) {
  this_ptr->swap(*other);
}

void qt_core_c_QUrl_toAce_to_output(const QString* arg1, QByteArray* output) {
  new(output) QByteArray(QUrl::toAce(*arg1));
}

void qt_core_c_QUrl_toDisplayString_to_output_no_args(const QUrl* this_ptr, QString* output) {
  new(output) QString(this_ptr->toDisplayString());
}

void qt_core_c_QUrl_toDisplayString_to_output_options(const QUrl* this_ptr, const QUrlTwoFlags< QUrl::UrlFormattingOption, QUrl::ComponentFormattingOption >* options, QString* output) {
  new(output) QString(this_ptr->toDisplayString(*options));
}

void qt_core_c_QUrl_toEncoded_to_output_no_args(const QUrl* this_ptr, QByteArray* output) {
  new(output) QByteArray(this_ptr->toEncoded());
}

void qt_core_c_QUrl_toEncoded_to_output_options(const QUrl* this_ptr, const QUrlTwoFlags< QUrl::UrlFormattingOption, QUrl::ComponentFormattingOption >* options, QByteArray* output) {
  new(output) QByteArray(this_ptr->toEncoded(*options));
}

void qt_core_c_QUrl_toLocalFile_to_output(const QUrl* this_ptr, QString* output) {
  new(output) QString(this_ptr->toLocalFile());
}

void qt_core_c_QUrl_toPercentEncoding_to_output_arg1(const QString* arg1, QByteArray* output) {
  new(output) QByteArray(QUrl::toPercentEncoding(*arg1));
}

void qt_core_c_QUrl_toPercentEncoding_to_output_arg1_exclude(const QString* arg1, const QByteArray* exclude, QByteArray* output) {
  new(output) QByteArray(QUrl::toPercentEncoding(*arg1, *exclude));
}

void qt_core_c_QUrl_toPercentEncoding_to_output_arg1_exclude_include(const QString* arg1, const QByteArray* exclude, const QByteArray* include, QByteArray* output) {
  new(output) QByteArray(QUrl::toPercentEncoding(*arg1, *exclude, *include));
}

void qt_core_c_QUrl_toStringList_to_output_uris(const QList< QUrl >* uris, QStringList* output) {
  new(output) QStringList(QUrl::toStringList(*uris));
}

void qt_core_c_QUrl_toStringList_to_output_uris_options(const QList< QUrl >* uris, const QUrlTwoFlags< QUrl::UrlFormattingOption, QUrl::ComponentFormattingOption >* options, QStringList* output) {
  new(output) QStringList(QUrl::toStringList(*uris, *options));
}

void qt_core_c_QUrl_toString_to_output_no_args(const QUrl* this_ptr, QString* output) {
  new(output) QString(this_ptr->toString());
}

void qt_core_c_QUrl_toString_to_output_options(const QUrl* this_ptr, const QUrlTwoFlags< QUrl::UrlFormattingOption, QUrl::ComponentFormattingOption >* options, QString* output) {
  new(output) QString(this_ptr->toString(*options));
}

void qt_core_c_QUrl_topLevelDomain_to_output_no_args(const QUrl* this_ptr, QString* output) {
  new(output) QString(this_ptr->topLevelDomain());
}

void qt_core_c_QUrl_topLevelDomain_to_output_options(const QUrl* this_ptr, unsigned int options, QString* output) {
  new(output) QString(this_ptr->topLevelDomain(QFlags< QUrl::ComponentFormattingOption >(options)));
}

void qt_core_c_QUrl_url_to_output_no_args(const QUrl* this_ptr, QString* output) {
  new(output) QString(this_ptr->url());
}

void qt_core_c_QUrl_url_to_output_options(const QUrl* this_ptr, const QUrlTwoFlags< QUrl::UrlFormattingOption, QUrl::ComponentFormattingOption >* options, QString* output) {
  new(output) QString(this_ptr->url(*options));
}

void qt_core_c_QUrl_userInfo_to_output_no_args(const QUrl* this_ptr, QString* output) {
  new(output) QString(this_ptr->userInfo());
}

void qt_core_c_QUrl_userInfo_to_output_options(const QUrl* this_ptr, unsigned int options, QString* output) {
  new(output) QString(this_ptr->userInfo(QFlags< QUrl::ComponentFormattingOption >(options)));
}

void qt_core_c_QUrl_userName_to_output_no_args(const QUrl* this_ptr, QString* output) {
  new(output) QString(this_ptr->userName());
}

void qt_core_c_QUrl_userName_to_output_options(const QUrl* this_ptr, unsigned int options, QString* output) {
  new(output) QString(this_ptr->userName(QFlags< QUrl::ComponentFormattingOption >(options)));
}

