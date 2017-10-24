#ifndef QT_CORE_C_QTRANSLATOR_H
#define QT_CORE_C_QTRANSLATOR_H

#include "qt_core_c_global.h"

extern "C" {

QT_CORE_C_EXPORT QTranslator* qt_core_c_QTranslator_G_dynamic_cast_QTranslator_ptr(QObject* ptr);
QT_CORE_C_EXPORT QObject* qt_core_c_QTranslator_G_static_cast_QObject_ptr(QTranslator* ptr);
QT_CORE_C_EXPORT QTranslator* qt_core_c_QTranslator_G_static_cast_QTranslator_ptr(QObject* ptr);
QT_CORE_C_EXPORT void qt_core_c_QTranslator_delete(QTranslator* this_ptr);
QT_CORE_C_EXPORT bool qt_core_c_QTranslator_isEmpty(const QTranslator* this_ptr);
QT_CORE_C_EXPORT bool qt_core_c_QTranslator_load_data_len(QTranslator* this_ptr, const unsigned char* data, int len);
QT_CORE_C_EXPORT bool qt_core_c_QTranslator_load_data_len_directory(QTranslator* this_ptr, const unsigned char* data, int len, const QString* directory);
QT_CORE_C_EXPORT bool qt_core_c_QTranslator_load_filename(QTranslator* this_ptr, const QString* filename);
QT_CORE_C_EXPORT bool qt_core_c_QTranslator_load_filename_directory(QTranslator* this_ptr, const QString* filename, const QString* directory);
QT_CORE_C_EXPORT bool qt_core_c_QTranslator_load_filename_directory_search_delimiters(QTranslator* this_ptr, const QString* filename, const QString* directory, const QString* search_delimiters);
QT_CORE_C_EXPORT bool qt_core_c_QTranslator_load_filename_directory_search_delimiters_suffix(QTranslator* this_ptr, const QString* filename, const QString* directory, const QString* search_delimiters, const QString* suffix);
QT_CORE_C_EXPORT bool qt_core_c_QTranslator_load_locale_filename(QTranslator* this_ptr, const QLocale* locale, const QString* filename);
QT_CORE_C_EXPORT bool qt_core_c_QTranslator_load_locale_filename_prefix(QTranslator* this_ptr, const QLocale* locale, const QString* filename, const QString* prefix);
QT_CORE_C_EXPORT bool qt_core_c_QTranslator_load_locale_filename_prefix_directory(QTranslator* this_ptr, const QLocale* locale, const QString* filename, const QString* prefix, const QString* directory);
QT_CORE_C_EXPORT bool qt_core_c_QTranslator_load_locale_filename_prefix_directory_suffix(QTranslator* this_ptr, const QLocale* locale, const QString* filename, const QString* prefix, const QString* directory, const QString* suffix);
QT_CORE_C_EXPORT const QMetaObject* qt_core_c_QTranslator_metaObject(const QTranslator* this_ptr);
QT_CORE_C_EXPORT QTranslator* qt_core_c_QTranslator_new_no_args();
QT_CORE_C_EXPORT QTranslator* qt_core_c_QTranslator_new_parent(QObject* parent);
QT_CORE_C_EXPORT void qt_core_c_QTranslator_trUtf8_to_output(const char* s, const char* c, int n, QString* output);
QT_CORE_C_EXPORT void qt_core_c_QTranslator_tr_to_output(const char* s, const char* c, int n, QString* output);
QT_CORE_C_EXPORT void qt_core_c_QTranslator_translate_to_output_context_sourceText(const QTranslator* this_ptr, const char* context, const char* sourceText, QString* output);
QT_CORE_C_EXPORT void qt_core_c_QTranslator_translate_to_output_context_sourceText_disambiguation(const QTranslator* this_ptr, const char* context, const char* sourceText, const char* disambiguation, QString* output);
QT_CORE_C_EXPORT void qt_core_c_QTranslator_translate_to_output_context_sourceText_disambiguation_n(const QTranslator* this_ptr, const char* context, const char* sourceText, const char* disambiguation, int n, QString* output);

} // extern "C"

#endif // QT_CORE_C_QTRANSLATOR_H
