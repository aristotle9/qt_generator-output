#include "qt_core_c_QCommandLineParser.h"

void qt_core_c_QCommandLineParser_addHelpOption_to_output(QCommandLineParser* this_ptr, QCommandLineOption* output) {
  new(output) QCommandLineOption(this_ptr->addHelpOption());
}

bool qt_core_c_QCommandLineParser_addOption(QCommandLineParser* this_ptr, const QCommandLineOption* commandLineOption) {
  return this_ptr->addOption(*commandLineOption);
}

bool qt_core_c_QCommandLineParser_addOptions(QCommandLineParser* this_ptr, const QList< QCommandLineOption >* options) {
  return this_ptr->addOptions(*options);
}

void qt_core_c_QCommandLineParser_addPositionalArgument_name_description(QCommandLineParser* this_ptr, const QString* name, const QString* description) {
  this_ptr->addPositionalArgument(*name, *description);
}

void qt_core_c_QCommandLineParser_addPositionalArgument_name_description_syntax(QCommandLineParser* this_ptr, const QString* name, const QString* description, const QString* syntax) {
  this_ptr->addPositionalArgument(*name, *description, *syntax);
}

void qt_core_c_QCommandLineParser_addVersionOption_to_output(QCommandLineParser* this_ptr, QCommandLineOption* output) {
  new(output) QCommandLineOption(this_ptr->addVersionOption());
}

void qt_core_c_QCommandLineParser_applicationDescription_to_output(const QCommandLineParser* this_ptr, QString* output) {
  new(output) QString(this_ptr->applicationDescription());
}

void qt_core_c_QCommandLineParser_clearPositionalArguments(QCommandLineParser* this_ptr) {
  this_ptr->clearPositionalArguments();
}

void qt_core_c_QCommandLineParser_constructor(QCommandLineParser* output) {
  new(output) QCommandLineParser();
}

void qt_core_c_QCommandLineParser_destructor(QCommandLineParser* this_ptr) {
  qt_core_c_call_destructor(this_ptr);
}

void qt_core_c_QCommandLineParser_errorText_to_output(const QCommandLineParser* this_ptr, QString* output) {
  new(output) QString(this_ptr->errorText());
}

void qt_core_c_QCommandLineParser_helpText_to_output(const QCommandLineParser* this_ptr, QString* output) {
  new(output) QString(this_ptr->helpText());
}

bool qt_core_c_QCommandLineParser_isSet_name(const QCommandLineParser* this_ptr, const QString* name) {
  return this_ptr->isSet(*name);
}

bool qt_core_c_QCommandLineParser_isSet_option(const QCommandLineParser* this_ptr, const QCommandLineOption* option) {
  return this_ptr->isSet(*option);
}

void qt_core_c_QCommandLineParser_optionNames_to_output(const QCommandLineParser* this_ptr, QStringList* output) {
  new(output) QStringList(this_ptr->optionNames());
}

bool qt_core_c_QCommandLineParser_parse(QCommandLineParser* this_ptr, const QStringList* arguments) {
  return this_ptr->parse(*arguments);
}

void qt_core_c_QCommandLineParser_positionalArguments_to_output(const QCommandLineParser* this_ptr, QStringList* output) {
  new(output) QStringList(this_ptr->positionalArguments());
}

void qt_core_c_QCommandLineParser_process_app(QCommandLineParser* this_ptr, const QCoreApplication* app) {
  this_ptr->process(*app);
}

void qt_core_c_QCommandLineParser_process_arguments(QCommandLineParser* this_ptr, const QStringList* arguments) {
  this_ptr->process(*arguments);
}

void qt_core_c_QCommandLineParser_setApplicationDescription(QCommandLineParser* this_ptr, const QString* description) {
  this_ptr->setApplicationDescription(*description);
}

void qt_core_c_QCommandLineParser_setOptionsAfterPositionalArgumentsMode(QCommandLineParser* this_ptr, QCommandLineParser::OptionsAfterPositionalArgumentsMode mode) {
  this_ptr->setOptionsAfterPositionalArgumentsMode(mode);
}

void qt_core_c_QCommandLineParser_setSingleDashWordOptionMode(QCommandLineParser* this_ptr, QCommandLineParser::SingleDashWordOptionMode parsingMode) {
  this_ptr->setSingleDashWordOptionMode(parsingMode);
}

void qt_core_c_QCommandLineParser_showHelp_exitCode(QCommandLineParser* this_ptr, int exitCode) {
  this_ptr->showHelp(exitCode);
}

void qt_core_c_QCommandLineParser_showHelp_no_args(QCommandLineParser* this_ptr) {
  this_ptr->showHelp();
}

void qt_core_c_QCommandLineParser_showVersion(QCommandLineParser* this_ptr) {
  this_ptr->showVersion();
}

void qt_core_c_QCommandLineParser_trUtf8_to_output(const char* sourceText, const char* disambiguation, int n, QString* output) {
  new(output) QString(QCommandLineParser::trUtf8(sourceText, disambiguation, n));
}

void qt_core_c_QCommandLineParser_tr_to_output(const char* sourceText, const char* disambiguation, int n, QString* output) {
  new(output) QString(QCommandLineParser::tr(sourceText, disambiguation, n));
}

void qt_core_c_QCommandLineParser_unknownOptionNames_to_output(const QCommandLineParser* this_ptr, QStringList* output) {
  new(output) QStringList(this_ptr->unknownOptionNames());
}

void qt_core_c_QCommandLineParser_value_to_output_name(const QCommandLineParser* this_ptr, const QString* name, QString* output) {
  new(output) QString(this_ptr->value(*name));
}

void qt_core_c_QCommandLineParser_value_to_output_option(const QCommandLineParser* this_ptr, const QCommandLineOption* option, QString* output) {
  new(output) QString(this_ptr->value(*option));
}

void qt_core_c_QCommandLineParser_values_to_output_name(const QCommandLineParser* this_ptr, const QString* name, QStringList* output) {
  new(output) QStringList(this_ptr->values(*name));
}

void qt_core_c_QCommandLineParser_values_to_output_option(const QCommandLineParser* this_ptr, const QCommandLineOption* option, QStringList* output) {
  new(output) QStringList(this_ptr->values(*option));
}

