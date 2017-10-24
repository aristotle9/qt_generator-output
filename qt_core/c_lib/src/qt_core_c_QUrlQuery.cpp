#include "qt_core_c_QUrlQuery.h"

unsigned int qt_core_c_QUrlQuery_G_qHash_key(const QUrlQuery* key) {
  return qHash(*key);
}

unsigned int qt_core_c_QUrlQuery_G_qHash_key_seed(const QUrlQuery* key, unsigned int seed) {
  return qHash(*key, seed);
}

void qt_core_c_QUrlQuery_G_swap(QUrlQuery* value1, QUrlQuery* value2) {
  swap(*value1, *value2);
}

void qt_core_c_QUrlQuery_addQueryItem(QUrlQuery* this_ptr, const QString* key, const QString* value) {
  this_ptr->addQueryItem(*key, *value);
}

void qt_core_c_QUrlQuery_clear(QUrlQuery* this_ptr) {
  this_ptr->clear();
}

void qt_core_c_QUrlQuery_constructor_no_args(QUrlQuery* output) {
  new(output) QUrlQuery();
}

void qt_core_c_QUrlQuery_constructor_other(const QUrlQuery* other, QUrlQuery* output) {
  new(output) QUrlQuery(*other);
}

void qt_core_c_QUrlQuery_constructor_queryString(const QString* queryString, QUrlQuery* output) {
  new(output) QUrlQuery(*queryString);
}

void qt_core_c_QUrlQuery_constructor_url(const QUrl* url, QUrlQuery* output) {
  new(output) QUrlQuery(*url);
}

void qt_core_c_QUrlQuery_defaultQueryPairDelimiter_to_output(QChar* output) {
  new(output) QChar(QUrlQuery::defaultQueryPairDelimiter());
}

void qt_core_c_QUrlQuery_defaultQueryValueDelimiter_to_output(QChar* output) {
  new(output) QChar(QUrlQuery::defaultQueryValueDelimiter());
}

void qt_core_c_QUrlQuery_destructor(QUrlQuery* this_ptr) {
  qt_core_c_call_destructor(this_ptr);
}

bool qt_core_c_QUrlQuery_hasQueryItem(const QUrlQuery* this_ptr, const QString* key) {
  return this_ptr->hasQueryItem(*key);
}

bool qt_core_c_QUrlQuery_isEmpty(const QUrlQuery* this_ptr) {
  return this_ptr->isEmpty();
}

QUrlQuery* qt_core_c_QUrlQuery_operator_assign(QUrlQuery* this_ptr, const QUrlQuery* other) {
  return &this_ptr->operator=(*other);
}

bool qt_core_c_QUrlQuery_operator_eq(const QUrlQuery* this_ptr, const QUrlQuery* other) {
  return this_ptr->operator==(*other);
}

bool qt_core_c_QUrlQuery_operator_neq(const QUrlQuery* this_ptr, const QUrlQuery* other) {
  return this_ptr->operator!=(*other);
}

void qt_core_c_QUrlQuery_queryPairDelimiter_to_output(const QUrlQuery* this_ptr, QChar* output) {
  new(output) QChar(this_ptr->queryPairDelimiter());
}

void qt_core_c_QUrlQuery_queryValueDelimiter_to_output(const QUrlQuery* this_ptr, QChar* output) {
  new(output) QChar(this_ptr->queryValueDelimiter());
}

void qt_core_c_QUrlQuery_removeAllQueryItems(QUrlQuery* this_ptr, const QString* key) {
  this_ptr->removeAllQueryItems(*key);
}

void qt_core_c_QUrlQuery_removeQueryItem(QUrlQuery* this_ptr, const QString* key) {
  this_ptr->removeQueryItem(*key);
}

void qt_core_c_QUrlQuery_setQuery(QUrlQuery* this_ptr, const QString* queryString) {
  this_ptr->setQuery(*queryString);
}

void qt_core_c_QUrlQuery_setQueryDelimiters(QUrlQuery* this_ptr, const QChar* valueDelimiter, const QChar* pairDelimiter) {
  this_ptr->setQueryDelimiters(*valueDelimiter, *pairDelimiter);
}

void qt_core_c_QUrlQuery_setQueryItems(QUrlQuery* this_ptr, const QList< QPair< QString, QString > >* query) {
  this_ptr->setQueryItems(*query);
}

void qt_core_c_QUrlQuery_swap(QUrlQuery* this_ptr, QUrlQuery* other) {
  this_ptr->swap(*other);
}

