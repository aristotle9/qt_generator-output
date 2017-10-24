#include "qt_core_c_QTranslator.h"

QTranslator* qt_core_c_QTranslator_G_dynamic_cast_QTranslator_ptr(QObject* ptr) {
  return dynamic_cast<QTranslator*>(ptr);
}

QObject* qt_core_c_QTranslator_G_static_cast_QObject_ptr(QTranslator* ptr) {
  return static_cast<QObject*>(ptr);
}

QTranslator* qt_core_c_QTranslator_G_static_cast_QTranslator_ptr(QObject* ptr) {
  return static_cast<QTranslator*>(ptr);
}

void qt_core_c_QTranslator_delete(QTranslator* this_ptr) {
  delete this_ptr;
}

bool qt_core_c_QTranslator_isEmpty(const QTranslator* this_ptr) {
  return this_ptr->isEmpty();
}

bool qt_core_c_QTranslator_load_data_len(QTranslator* this_ptr, const unsigned char* data, int len) {
  return this_ptr->load(data, len);
}

bool qt_core_c_QTranslator_load_data_len_directory(QTranslator* this_ptr, const unsigned char* data, int len, const QString* directory) {
  return this_ptr->load(data, len, *directory);
}

bool qt_core_c_QTranslator_load_filename(QTranslator* this_ptr, const QString* filename) {
  return this_ptr->load(*filename);
}

bool qt_core_c_QTranslator_load_filename_directory(QTranslator* this_ptr, const QString* filename, const QString* directory) {
  return this_ptr->load(*filename, *directory);
}

bool qt_core_c_QTranslator_load_filename_directory_search_delimiters(QTranslator* this_ptr, const QString* filename, const QString* directory, const QString* search_delimiters) {
  return this_ptr->load(*filename, *directory, *search_delimiters);
}

bool qt_core_c_QTranslator_load_filename_directory_search_delimiters_suffix(QTranslator* this_ptr, const QString* filename, const QString* directory, const QString* search_delimiters, const QString* suffix) {
  return this_ptr->load(*filename, *directory, *search_delimiters, *suffix);
}

bool qt_core_c_QTranslator_load_locale_filename(QTranslator* this_ptr, const QLocale* locale, const QString* filename) {
  return this_ptr->load(*locale, *filename);
}

bool qt_core_c_QTranslator_load_locale_filename_prefix(QTranslator* this_ptr, const QLocale* locale, const QString* filename, const QString* prefix) {
  return this_ptr->load(*locale, *filename, *prefix);
}

bool qt_core_c_QTranslator_load_locale_filename_prefix_directory(QTranslator* this_ptr, const QLocale* locale, const QString* filename, const QString* prefix, const QString* directory) {
  return this_ptr->load(*locale, *filename, *prefix, *directory);
}

bool qt_core_c_QTranslator_load_locale_filename_prefix_directory_suffix(QTranslator* this_ptr, const QLocale* locale, const QString* filename, const QString* prefix, const QString* directory, const QString* suffix) {
  return this_ptr->load(*locale, *filename, *prefix, *directory, *suffix);
}

const QMetaObject* qt_core_c_QTranslator_metaObject(const QTranslator* this_ptr) {
  return this_ptr->metaObject();
}

QTranslator* qt_core_c_QTranslator_new_no_args() {
  return new QTranslator();
}

QTranslator* qt_core_c_QTranslator_new_parent(QObject* parent) {
  return new QTranslator(parent);
}

void qt_core_c_QTranslator_trUtf8_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(QTranslator::trUtf8(s, c, n));
}

void qt_core_c_QTranslator_tr_to_output(const char* s, const char* c, int n, QString* output) {
  new(output) QString(QTranslator::tr(s, c, n));
}

void qt_core_c_QTranslator_translate_to_output_context_sourceText(const QTranslator* this_ptr, const char* context, const char* sourceText, QString* output) {
  new(output) QString(this_ptr->translate(context, sourceText));
}

void qt_core_c_QTranslator_translate_to_output_context_sourceText_disambiguation(const QTranslator* this_ptr, const char* context, const char* sourceText, const char* disambiguation, QString* output) {
  new(output) QString(this_ptr->translate(context, sourceText, disambiguation));
}

void qt_core_c_QTranslator_translate_to_output_context_sourceText_disambiguation_n(const QTranslator* this_ptr, const char* context, const char* sourceText, const char* disambiguation, int n, QString* output) {
  new(output) QString(this_ptr->translate(context, sourceText, disambiguation, n));
}

