#ifndef QT_CORE_C_QCOMMANDLINEPARSER_H
#define QT_CORE_C_QCOMMANDLINEPARSER_H

#include "qt_core_c_global.h"

extern "C" {

QT_CORE_C_EXPORT void qt_core_c_QCommandLineParser_addHelpOption_to_output(QCommandLineParser* this_ptr, QCommandLineOption* output);
QT_CORE_C_EXPORT bool qt_core_c_QCommandLineParser_addOption(QCommandLineParser* this_ptr, const QCommandLineOption* commandLineOption);
QT_CORE_C_EXPORT bool qt_core_c_QCommandLineParser_addOptions(QCommandLineParser* this_ptr, const QList< QCommandLineOption >* options);
QT_CORE_C_EXPORT void qt_core_c_QCommandLineParser_addPositionalArgument_name_description(QCommandLineParser* this_ptr, const QString* name, const QString* description);
QT_CORE_C_EXPORT void qt_core_c_QCommandLineParser_addPositionalArgument_name_description_syntax(QCommandLineParser* this_ptr, const QString* name, const QString* description, const QString* syntax);
QT_CORE_C_EXPORT void qt_core_c_QCommandLineParser_addVersionOption_to_output(QCommandLineParser* this_ptr, QCommandLineOption* output);
QT_CORE_C_EXPORT void qt_core_c_QCommandLineParser_applicationDescription_to_output(const QCommandLineParser* this_ptr, QString* output);
QT_CORE_C_EXPORT void qt_core_c_QCommandLineParser_clearPositionalArguments(QCommandLineParser* this_ptr);
QT_CORE_C_EXPORT void qt_core_c_QCommandLineParser_constructor(QCommandLineParser* output);
QT_CORE_C_EXPORT void qt_core_c_QCommandLineParser_destructor(QCommandLineParser* this_ptr);
QT_CORE_C_EXPORT void qt_core_c_QCommandLineParser_errorText_to_output(const QCommandLineParser* this_ptr, QString* output);
QT_CORE_C_EXPORT void qt_core_c_QCommandLineParser_helpText_to_output(const QCommandLineParser* this_ptr, QString* output);
QT_CORE_C_EXPORT bool qt_core_c_QCommandLineParser_isSet_name(const QCommandLineParser* this_ptr, const QString* name);
QT_CORE_C_EXPORT bool qt_core_c_QCommandLineParser_isSet_option(const QCommandLineParser* this_ptr, const QCommandLineOption* option);
QT_CORE_C_EXPORT void qt_core_c_QCommandLineParser_optionNames_to_output(const QCommandLineParser* this_ptr, QStringList* output);
QT_CORE_C_EXPORT bool qt_core_c_QCommandLineParser_parse(QCommandLineParser* this_ptr, const QStringList* arguments);
QT_CORE_C_EXPORT void qt_core_c_QCommandLineParser_positionalArguments_to_output(const QCommandLineParser* this_ptr, QStringList* output);
QT_CORE_C_EXPORT void qt_core_c_QCommandLineParser_process_app(QCommandLineParser* this_ptr, const QCoreApplication* app);
QT_CORE_C_EXPORT void qt_core_c_QCommandLineParser_process_arguments(QCommandLineParser* this_ptr, const QStringList* arguments);
QT_CORE_C_EXPORT void qt_core_c_QCommandLineParser_setApplicationDescription(QCommandLineParser* this_ptr, const QString* description);
QT_CORE_C_EXPORT void qt_core_c_QCommandLineParser_setOptionsAfterPositionalArgumentsMode(QCommandLineParser* this_ptr, QCommandLineParser::OptionsAfterPositionalArgumentsMode mode);
QT_CORE_C_EXPORT void qt_core_c_QCommandLineParser_setSingleDashWordOptionMode(QCommandLineParser* this_ptr, QCommandLineParser::SingleDashWordOptionMode parsingMode);
QT_CORE_C_EXPORT void qt_core_c_QCommandLineParser_showHelp_exitCode(QCommandLineParser* this_ptr, int exitCode);
QT_CORE_C_EXPORT void qt_core_c_QCommandLineParser_showHelp_no_args(QCommandLineParser* this_ptr);
QT_CORE_C_EXPORT void qt_core_c_QCommandLineParser_showVersion(QCommandLineParser* this_ptr);
QT_CORE_C_EXPORT void qt_core_c_QCommandLineParser_trUtf8_to_output(const char* sourceText, const char* disambiguation, int n, QString* output);
QT_CORE_C_EXPORT void qt_core_c_QCommandLineParser_tr_to_output(const char* sourceText, const char* disambiguation, int n, QString* output);
QT_CORE_C_EXPORT void qt_core_c_QCommandLineParser_unknownOptionNames_to_output(const QCommandLineParser* this_ptr, QStringList* output);
QT_CORE_C_EXPORT void qt_core_c_QCommandLineParser_value_to_output_name(const QCommandLineParser* this_ptr, const QString* name, QString* output);
QT_CORE_C_EXPORT void qt_core_c_QCommandLineParser_value_to_output_option(const QCommandLineParser* this_ptr, const QCommandLineOption* option, QString* output);
QT_CORE_C_EXPORT void qt_core_c_QCommandLineParser_values_to_output_name(const QCommandLineParser* this_ptr, const QString* name, QStringList* output);
QT_CORE_C_EXPORT void qt_core_c_QCommandLineParser_values_to_output_option(const QCommandLineParser* this_ptr, const QCommandLineOption* option, QStringList* output);

} // extern "C"

#endif // QT_CORE_C_QCOMMANDLINEPARSER_H
