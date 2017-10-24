#include "qt_core_c_QLocale.h"

QDataStream* qt_core_c_QLocale_G_operator_shl(QDataStream* arg1, const QLocale* arg2) {
  return &operator<<(*arg1, *arg2);
}

void qt_core_c_QLocale_G_operator_shl_to_output(const QDebug* arg1, const QLocale* arg2, QDebug* output) {
  new(output) QDebug(operator<<(*arg1, *arg2));
}

QDataStream* qt_core_c_QLocale_G_operator_shr(QDataStream* arg1, QLocale* arg2) {
  return &operator>>(*arg1, *arg2);
}

unsigned int qt_core_c_QLocale_G_qHash_key(const QLocale* key) {
  return qHash(*key);
}

unsigned int qt_core_c_QLocale_G_qHash_key_seed(const QLocale* key, unsigned int seed) {
  return qHash(*key, seed);
}

void qt_core_c_QLocale_G_swap(QLocale* value1, QLocale* value2) {
  swap(*value1, *value2);
}

void qt_core_c_QLocale_amText_to_output(const QLocale* this_ptr, QString* output) {
  new(output) QString(this_ptr->amText());
}

void qt_core_c_QLocale_bcp47Name_to_output(const QLocale* this_ptr, QString* output) {
  new(output) QString(this_ptr->bcp47Name());
}

void qt_core_c_QLocale_c_to_output(QLocale* output) {
  new(output) QLocale(QLocale::c());
}

void qt_core_c_QLocale_constructor_language(QLocale::Language language, QLocale* output) {
  new(output) QLocale(language);
}

void qt_core_c_QLocale_constructor_language_country(QLocale::Language language, QLocale::Country country, QLocale* output) {
  new(output) QLocale(language, country);
}

void qt_core_c_QLocale_constructor_language_script_country(QLocale::Language language, QLocale::Script script, QLocale::Country country, QLocale* output) {
  new(output) QLocale(language, script, country);
}

void qt_core_c_QLocale_constructor_name(const QString* name, QLocale* output) {
  new(output) QLocale(*name);
}

void qt_core_c_QLocale_constructor_no_args(QLocale* output) {
  new(output) QLocale();
}

void qt_core_c_QLocale_constructor_other(const QLocale* other, QLocale* output) {
  new(output) QLocale(*other);
}

void qt_core_c_QLocale_countriesForLanguage_to_output(QLocale::Language lang, QList< QLocale::Country >* output) {
  new(output) QList< QLocale::Country >(QLocale::countriesForLanguage(lang));
}

QLocale::Country qt_core_c_QLocale_country(const QLocale* this_ptr) {
  return this_ptr->country();
}

void qt_core_c_QLocale_countryToString_to_output(QLocale::Country country, QString* output) {
  new(output) QString(QLocale::countryToString(country));
}

void qt_core_c_QLocale_createSeparatedList_to_output(const QLocale* this_ptr, const QStringList* strl, QString* output) {
  new(output) QString(this_ptr->createSeparatedList(*strl));
}

void qt_core_c_QLocale_currencySymbol_to_output_arg1(const QLocale* this_ptr, QLocale::CurrencySymbolFormat arg1, QString* output) {
  new(output) QString(this_ptr->currencySymbol(arg1));
}

void qt_core_c_QLocale_currencySymbol_to_output_no_args(const QLocale* this_ptr, QString* output) {
  new(output) QString(this_ptr->currencySymbol());
}

void qt_core_c_QLocale_dateFormat_to_output_format(const QLocale* this_ptr, QLocale::FormatType format, QString* output) {
  new(output) QString(this_ptr->dateFormat(format));
}

void qt_core_c_QLocale_dateFormat_to_output_no_args(const QLocale* this_ptr, QString* output) {
  new(output) QString(this_ptr->dateFormat());
}

void qt_core_c_QLocale_dateTimeFormat_to_output_format(const QLocale* this_ptr, QLocale::FormatType format, QString* output) {
  new(output) QString(this_ptr->dateTimeFormat(format));
}

void qt_core_c_QLocale_dateTimeFormat_to_output_no_args(const QLocale* this_ptr, QString* output) {
  new(output) QString(this_ptr->dateTimeFormat());
}

void qt_core_c_QLocale_dayName_to_output_arg1(const QLocale* this_ptr, int arg1, QString* output) {
  new(output) QString(this_ptr->dayName(arg1));
}

void qt_core_c_QLocale_dayName_to_output_arg1_format(const QLocale* this_ptr, int arg1, QLocale::FormatType format, QString* output) {
  new(output) QString(this_ptr->dayName(arg1, format));
}

void qt_core_c_QLocale_decimalPoint_to_output(const QLocale* this_ptr, QChar* output) {
  new(output) QChar(this_ptr->decimalPoint());
}

void qt_core_c_QLocale_destructor(QLocale* this_ptr) {
  qt_core_c_call_destructor(this_ptr);
}

void qt_core_c_QLocale_exponential_to_output(const QLocale* this_ptr, QChar* output) {
  new(output) QChar(this_ptr->exponential());
}

void qt_core_c_QLocale_groupSeparator_to_output(const QLocale* this_ptr, QChar* output) {
  new(output) QChar(this_ptr->groupSeparator());
}

QLocale::Language qt_core_c_QLocale_language(const QLocale* this_ptr) {
  return this_ptr->language();
}

void qt_core_c_QLocale_languageToString_to_output(QLocale::Language language, QString* output) {
  new(output) QString(QLocale::languageToString(language));
}

void qt_core_c_QLocale_matchingLocales_to_output(const QLocale::Language* language, const QLocale::Script* script, const QLocale::Country* country, QList< QLocale >* output) {
  new(output) QList< QLocale >(QLocale::matchingLocales(*language, *script, *country));
}

QLocale::MeasurementSystem qt_core_c_QLocale_measurementSystem(const QLocale* this_ptr) {
  return this_ptr->measurementSystem();
}

void qt_core_c_QLocale_monthName_to_output_arg1(const QLocale* this_ptr, int arg1, QString* output) {
  new(output) QString(this_ptr->monthName(arg1));
}

void qt_core_c_QLocale_monthName_to_output_arg1_format(const QLocale* this_ptr, int arg1, QLocale::FormatType format, QString* output) {
  new(output) QString(this_ptr->monthName(arg1, format));
}

void qt_core_c_QLocale_name_to_output(const QLocale* this_ptr, QString* output) {
  new(output) QString(this_ptr->name());
}

void qt_core_c_QLocale_nativeCountryName_to_output(const QLocale* this_ptr, QString* output) {
  new(output) QString(this_ptr->nativeCountryName());
}

void qt_core_c_QLocale_nativeLanguageName_to_output(const QLocale* this_ptr, QString* output) {
  new(output) QString(this_ptr->nativeLanguageName());
}

void qt_core_c_QLocale_negativeSign_to_output(const QLocale* this_ptr, QChar* output) {
  new(output) QChar(this_ptr->negativeSign());
}

unsigned int qt_core_c_QLocale_numberOptions(const QLocale* this_ptr) {
  return uint(this_ptr->numberOptions());
}

QLocale* qt_core_c_QLocale_operator_assign(QLocale* this_ptr, const QLocale* other) {
  return &this_ptr->operator=(*other);
}

bool qt_core_c_QLocale_operator_eq(const QLocale* this_ptr, const QLocale* other) {
  return this_ptr->operator==(*other);
}

bool qt_core_c_QLocale_operator_neq(const QLocale* this_ptr, const QLocale* other) {
  return this_ptr->operator!=(*other);
}

void qt_core_c_QLocale_percent_to_output(const QLocale* this_ptr, QChar* output) {
  new(output) QChar(this_ptr->percent());
}

void qt_core_c_QLocale_pmText_to_output(const QLocale* this_ptr, QString* output) {
  new(output) QString(this_ptr->pmText());
}

void qt_core_c_QLocale_positiveSign_to_output(const QLocale* this_ptr, QChar* output) {
  new(output) QChar(this_ptr->positiveSign());
}

void qt_core_c_QLocale_quoteString_to_output_QString(const QLocale* this_ptr, const QString* str, QString* output) {
  new(output) QString(this_ptr->quoteString(*str));
}

void qt_core_c_QLocale_quoteString_to_output_QStringRef(const QLocale* this_ptr, const QStringRef* str, QString* output) {
  new(output) QString(this_ptr->quoteString(*str));
}

void qt_core_c_QLocale_quoteString_to_output_QStringRef_QLocale_QuotationStyle(const QLocale* this_ptr, const QStringRef* str, QLocale::QuotationStyle style, QString* output) {
  new(output) QString(this_ptr->quoteString(*str, style));
}

void qt_core_c_QLocale_quoteString_to_output_QString_QLocale_QuotationStyle(const QLocale* this_ptr, const QString* str, QLocale::QuotationStyle style, QString* output) {
  new(output) QString(this_ptr->quoteString(*str, style));
}

QLocale::Script qt_core_c_QLocale_script(const QLocale* this_ptr) {
  return this_ptr->script();
}

void qt_core_c_QLocale_scriptToString_to_output(QLocale::Script script, QString* output) {
  new(output) QString(QLocale::scriptToString(script));
}

void qt_core_c_QLocale_setDefault(const QLocale* locale) {
  QLocale::setDefault(*locale);
}

void qt_core_c_QLocale_setNumberOptions(QLocale* this_ptr, unsigned int options) {
  this_ptr->setNumberOptions(QFlags< QLocale::NumberOption >(options));
}

void qt_core_c_QLocale_standaloneDayName_to_output_arg1(const QLocale* this_ptr, int arg1, QString* output) {
  new(output) QString(this_ptr->standaloneDayName(arg1));
}

void qt_core_c_QLocale_standaloneDayName_to_output_arg1_format(const QLocale* this_ptr, int arg1, QLocale::FormatType format, QString* output) {
  new(output) QString(this_ptr->standaloneDayName(arg1, format));
}

void qt_core_c_QLocale_standaloneMonthName_to_output_arg1(const QLocale* this_ptr, int arg1, QString* output) {
  new(output) QString(this_ptr->standaloneMonthName(arg1));
}

void qt_core_c_QLocale_standaloneMonthName_to_output_arg1_format(const QLocale* this_ptr, int arg1, QLocale::FormatType format, QString* output) {
  new(output) QString(this_ptr->standaloneMonthName(arg1, format));
}

void qt_core_c_QLocale_swap(QLocale* this_ptr, QLocale* other) {
  this_ptr->swap(*other);
}

void qt_core_c_QLocale_system_to_output(QLocale* output) {
  new(output) QLocale(QLocale::system());
}

void qt_core_c_QLocale_timeFormat_to_output_format(const QLocale* this_ptr, QLocale::FormatType format, QString* output) {
  new(output) QString(this_ptr->timeFormat(format));
}

void qt_core_c_QLocale_timeFormat_to_output_no_args(const QLocale* this_ptr, QString* output) {
  new(output) QString(this_ptr->timeFormat());
}

void qt_core_c_QLocale_toCurrencyString_to_output_double(const QLocale* this_ptr, double arg1, QString* output) {
  new(output) QString(this_ptr->toCurrencyString(arg1));
}

void qt_core_c_QLocale_toCurrencyString_to_output_double_QString(const QLocale* this_ptr, double arg1, const QString* symbol, QString* output) {
  new(output) QString(this_ptr->toCurrencyString(arg1, *symbol));
}

void qt_core_c_QLocale_toCurrencyString_to_output_double_QString_int(const QLocale* this_ptr, double arg1, const QString* symbol, int precision, QString* output) {
  new(output) QString(this_ptr->toCurrencyString(arg1, *symbol, precision));
}

void qt_core_c_QLocale_toCurrencyString_to_output_float(const QLocale* this_ptr, float i, QString* output) {
  new(output) QString(this_ptr->toCurrencyString(i));
}

void qt_core_c_QLocale_toCurrencyString_to_output_float_QString(const QLocale* this_ptr, float i, const QString* symbol, QString* output) {
  new(output) QString(this_ptr->toCurrencyString(i, *symbol));
}

void qt_core_c_QLocale_toCurrencyString_to_output_float_QString_int(const QLocale* this_ptr, float i, const QString* symbol, int precision, QString* output) {
  new(output) QString(this_ptr->toCurrencyString(i, *symbol, precision));
}

void qt_core_c_QLocale_toCurrencyString_to_output_int(const QLocale* this_ptr, int arg1, QString* output) {
  new(output) QString(this_ptr->toCurrencyString(arg1));
}

void qt_core_c_QLocale_toCurrencyString_to_output_int_QString(const QLocale* this_ptr, int arg1, const QString* symbol, QString* output) {
  new(output) QString(this_ptr->toCurrencyString(arg1, *symbol));
}

void qt_core_c_QLocale_toCurrencyString_to_output_qlonglong(const QLocale* this_ptr, qlonglong arg1, QString* output) {
  new(output) QString(this_ptr->toCurrencyString(arg1));
}

void qt_core_c_QLocale_toCurrencyString_to_output_qlonglong_QString(const QLocale* this_ptr, qlonglong arg1, const QString* symbol, QString* output) {
  new(output) QString(this_ptr->toCurrencyString(arg1, *symbol));
}

void qt_core_c_QLocale_toCurrencyString_to_output_qulonglong(const QLocale* this_ptr, qulonglong arg1, QString* output) {
  new(output) QString(this_ptr->toCurrencyString(arg1));
}

void qt_core_c_QLocale_toCurrencyString_to_output_qulonglong_QString(const QLocale* this_ptr, qulonglong arg1, const QString* symbol, QString* output) {
  new(output) QString(this_ptr->toCurrencyString(arg1, *symbol));
}

void qt_core_c_QLocale_toCurrencyString_to_output_short(const QLocale* this_ptr, short arg1, QString* output) {
  new(output) QString(this_ptr->toCurrencyString(arg1));
}

void qt_core_c_QLocale_toCurrencyString_to_output_short_QString(const QLocale* this_ptr, short arg1, const QString* symbol, QString* output) {
  new(output) QString(this_ptr->toCurrencyString(arg1, *symbol));
}

void qt_core_c_QLocale_toCurrencyString_to_output_unsigned_int(const QLocale* this_ptr, unsigned int arg1, QString* output) {
  new(output) QString(this_ptr->toCurrencyString(arg1));
}

void qt_core_c_QLocale_toCurrencyString_to_output_unsigned_int_QString(const QLocale* this_ptr, unsigned int arg1, const QString* symbol, QString* output) {
  new(output) QString(this_ptr->toCurrencyString(arg1, *symbol));
}

void qt_core_c_QLocale_toCurrencyString_to_output_unsigned_short(const QLocale* this_ptr, unsigned short arg1, QString* output) {
  new(output) QString(this_ptr->toCurrencyString(arg1));
}

void qt_core_c_QLocale_toCurrencyString_to_output_unsigned_short_QString(const QLocale* this_ptr, unsigned short arg1, const QString* symbol, QString* output) {
  new(output) QString(this_ptr->toCurrencyString(arg1, *symbol));
}

void qt_core_c_QLocale_toDateTime_to_output_QString(const QLocale* this_ptr, const QString* string, QDateTime* output) {
  new(output) QDateTime(this_ptr->toDateTime(*string));
}

void qt_core_c_QLocale_toDateTime_to_output_QString_QLocale_FormatType(const QLocale* this_ptr, const QString* string, QLocale::FormatType format, QDateTime* output) {
  new(output) QDateTime(this_ptr->toDateTime(*string, format));
}

void qt_core_c_QLocale_toDateTime_to_output_QString_QString(const QLocale* this_ptr, const QString* string, const QString* format, QDateTime* output) {
  new(output) QDateTime(this_ptr->toDateTime(*string, *format));
}

void qt_core_c_QLocale_toDate_to_output_string(const QLocale* this_ptr, const QString* string, QDate* output) {
  new(output) QDate(this_ptr->toDate(*string));
}

void qt_core_c_QLocale_toDate_to_output_string_arg2(const QLocale* this_ptr, const QString* string, QLocale::FormatType arg2, QDate* output) {
  new(output) QDate(this_ptr->toDate(*string, arg2));
}

void qt_core_c_QLocale_toDate_to_output_string_format(const QLocale* this_ptr, const QString* string, const QString* format, QDate* output) {
  new(output) QDate(this_ptr->toDate(*string, *format));
}

double qt_core_c_QLocale_toDouble_QString(const QLocale* this_ptr, const QString* s) {
  return this_ptr->toDouble(*s);
}

double qt_core_c_QLocale_toDouble_QStringRef(const QLocale* this_ptr, const QStringRef* s) {
  return this_ptr->toDouble(*s);
}

double qt_core_c_QLocale_toDouble_QStringRef_bool(const QLocale* this_ptr, const QStringRef* s, bool* ok) {
  return this_ptr->toDouble(*s, ok);
}

double qt_core_c_QLocale_toDouble_QString_bool(const QLocale* this_ptr, const QString* s, bool* ok) {
  return this_ptr->toDouble(*s, ok);
}

float qt_core_c_QLocale_toFloat_QString(const QLocale* this_ptr, const QString* s) {
  return this_ptr->toFloat(*s);
}

float qt_core_c_QLocale_toFloat_QStringRef(const QLocale* this_ptr, const QStringRef* s) {
  return this_ptr->toFloat(*s);
}

float qt_core_c_QLocale_toFloat_QStringRef_bool(const QLocale* this_ptr, const QStringRef* s, bool* ok) {
  return this_ptr->toFloat(*s, ok);
}

float qt_core_c_QLocale_toFloat_QString_bool(const QLocale* this_ptr, const QString* s, bool* ok) {
  return this_ptr->toFloat(*s, ok);
}

int qt_core_c_QLocale_toInt_QString(const QLocale* this_ptr, const QString* s) {
  return this_ptr->toInt(*s);
}

int qt_core_c_QLocale_toInt_QStringRef(const QLocale* this_ptr, const QStringRef* s) {
  return this_ptr->toInt(*s);
}

int qt_core_c_QLocale_toInt_QStringRef_bool(const QLocale* this_ptr, const QStringRef* s, bool* ok) {
  return this_ptr->toInt(*s, ok);
}

int qt_core_c_QLocale_toInt_QString_bool(const QLocale* this_ptr, const QString* s, bool* ok) {
  return this_ptr->toInt(*s, ok);
}

qlonglong qt_core_c_QLocale_toLongLong_QString(const QLocale* this_ptr, const QString* s) {
  return this_ptr->toLongLong(*s);
}

qlonglong qt_core_c_QLocale_toLongLong_QStringRef(const QLocale* this_ptr, const QStringRef* s) {
  return this_ptr->toLongLong(*s);
}

qlonglong qt_core_c_QLocale_toLongLong_QStringRef_bool(const QLocale* this_ptr, const QStringRef* s, bool* ok) {
  return this_ptr->toLongLong(*s, ok);
}

qlonglong qt_core_c_QLocale_toLongLong_QString_bool(const QLocale* this_ptr, const QString* s, bool* ok) {
  return this_ptr->toLongLong(*s, ok);
}

void qt_core_c_QLocale_toLower_to_output(const QLocale* this_ptr, const QString* str, QString* output) {
  new(output) QString(this_ptr->toLower(*str));
}

short qt_core_c_QLocale_toShort_QString(const QLocale* this_ptr, const QString* s) {
  return this_ptr->toShort(*s);
}

short qt_core_c_QLocale_toShort_QStringRef(const QLocale* this_ptr, const QStringRef* s) {
  return this_ptr->toShort(*s);
}

short qt_core_c_QLocale_toShort_QStringRef_bool(const QLocale* this_ptr, const QStringRef* s, bool* ok) {
  return this_ptr->toShort(*s, ok);
}

short qt_core_c_QLocale_toShort_QString_bool(const QLocale* this_ptr, const QString* s, bool* ok) {
  return this_ptr->toShort(*s, ok);
}

void qt_core_c_QLocale_toString_to_output_QDate(const QLocale* this_ptr, const QDate* date, QString* output) {
  new(output) QString(this_ptr->toString(*date));
}

void qt_core_c_QLocale_toString_to_output_QDateTime(const QLocale* this_ptr, const QDateTime* dateTime, QString* output) {
  new(output) QString(this_ptr->toString(*dateTime));
}

void qt_core_c_QLocale_toString_to_output_QDateTime_QLocale_FormatType(const QLocale* this_ptr, const QDateTime* dateTime, QLocale::FormatType format, QString* output) {
  new(output) QString(this_ptr->toString(*dateTime, format));
}

void qt_core_c_QLocale_toString_to_output_QDateTime_QString(const QLocale* this_ptr, const QDateTime* dateTime, const QString* format, QString* output) {
  new(output) QString(this_ptr->toString(*dateTime, *format));
}

void qt_core_c_QLocale_toString_to_output_QDate_QLocale_FormatType(const QLocale* this_ptr, const QDate* date, QLocale::FormatType format, QString* output) {
  new(output) QString(this_ptr->toString(*date, format));
}

void qt_core_c_QLocale_toString_to_output_QDate_QString(const QLocale* this_ptr, const QDate* date, const QString* formatStr, QString* output) {
  new(output) QString(this_ptr->toString(*date, *formatStr));
}

void qt_core_c_QLocale_toString_to_output_QTime(const QLocale* this_ptr, const QTime* time, QString* output) {
  new(output) QString(this_ptr->toString(*time));
}

void qt_core_c_QLocale_toString_to_output_QTime_QLocale_FormatType(const QLocale* this_ptr, const QTime* time, QLocale::FormatType format, QString* output) {
  new(output) QString(this_ptr->toString(*time, format));
}

void qt_core_c_QLocale_toString_to_output_QTime_QString(const QLocale* this_ptr, const QTime* time, const QString* formatStr, QString* output) {
  new(output) QString(this_ptr->toString(*time, *formatStr));
}

void qt_core_c_QLocale_toString_to_output_double(const QLocale* this_ptr, double i, QString* output) {
  new(output) QString(this_ptr->toString(i));
}

void qt_core_c_QLocale_toString_to_output_double_char(const QLocale* this_ptr, double i, char f, QString* output) {
  new(output) QString(this_ptr->toString(i, f));
}

void qt_core_c_QLocale_toString_to_output_double_char_int(const QLocale* this_ptr, double i, char f, int prec, QString* output) {
  new(output) QString(this_ptr->toString(i, f, prec));
}

void qt_core_c_QLocale_toString_to_output_float(const QLocale* this_ptr, float i, QString* output) {
  new(output) QString(this_ptr->toString(i));
}

void qt_core_c_QLocale_toString_to_output_float_char(const QLocale* this_ptr, float i, char f, QString* output) {
  new(output) QString(this_ptr->toString(i, f));
}

void qt_core_c_QLocale_toString_to_output_float_char_int(const QLocale* this_ptr, float i, char f, int prec, QString* output) {
  new(output) QString(this_ptr->toString(i, f, prec));
}

void qt_core_c_QLocale_toString_to_output_int(const QLocale* this_ptr, int i, QString* output) {
  new(output) QString(this_ptr->toString(i));
}

void qt_core_c_QLocale_toString_to_output_qlonglong(const QLocale* this_ptr, qlonglong i, QString* output) {
  new(output) QString(this_ptr->toString(i));
}

void qt_core_c_QLocale_toString_to_output_qulonglong(const QLocale* this_ptr, qulonglong i, QString* output) {
  new(output) QString(this_ptr->toString(i));
}

void qt_core_c_QLocale_toString_to_output_short(const QLocale* this_ptr, short i, QString* output) {
  new(output) QString(this_ptr->toString(i));
}

void qt_core_c_QLocale_toString_to_output_unsigned_int(const QLocale* this_ptr, unsigned int i, QString* output) {
  new(output) QString(this_ptr->toString(i));
}

void qt_core_c_QLocale_toString_to_output_unsigned_short(const QLocale* this_ptr, unsigned short i, QString* output) {
  new(output) QString(this_ptr->toString(i));
}

void qt_core_c_QLocale_toTime_to_output_string(const QLocale* this_ptr, const QString* string, QTime* output) {
  new(output) QTime(this_ptr->toTime(*string));
}

void qt_core_c_QLocale_toTime_to_output_string_arg2(const QLocale* this_ptr, const QString* string, QLocale::FormatType arg2, QTime* output) {
  new(output) QTime(this_ptr->toTime(*string, arg2));
}

void qt_core_c_QLocale_toTime_to_output_string_format(const QLocale* this_ptr, const QString* string, const QString* format, QTime* output) {
  new(output) QTime(this_ptr->toTime(*string, *format));
}

unsigned int qt_core_c_QLocale_toUInt_QString(const QLocale* this_ptr, const QString* s) {
  return this_ptr->toUInt(*s);
}

unsigned int qt_core_c_QLocale_toUInt_QStringRef(const QLocale* this_ptr, const QStringRef* s) {
  return this_ptr->toUInt(*s);
}

unsigned int qt_core_c_QLocale_toUInt_QStringRef_bool(const QLocale* this_ptr, const QStringRef* s, bool* ok) {
  return this_ptr->toUInt(*s, ok);
}

unsigned int qt_core_c_QLocale_toUInt_QString_bool(const QLocale* this_ptr, const QString* s, bool* ok) {
  return this_ptr->toUInt(*s, ok);
}

qulonglong qt_core_c_QLocale_toULongLong_QString(const QLocale* this_ptr, const QString* s) {
  return this_ptr->toULongLong(*s);
}

qulonglong qt_core_c_QLocale_toULongLong_QStringRef(const QLocale* this_ptr, const QStringRef* s) {
  return this_ptr->toULongLong(*s);
}

qulonglong qt_core_c_QLocale_toULongLong_QStringRef_bool(const QLocale* this_ptr, const QStringRef* s, bool* ok) {
  return this_ptr->toULongLong(*s, ok);
}

qulonglong qt_core_c_QLocale_toULongLong_QString_bool(const QLocale* this_ptr, const QString* s, bool* ok) {
  return this_ptr->toULongLong(*s, ok);
}

unsigned short qt_core_c_QLocale_toUShort_QString(const QLocale* this_ptr, const QString* s) {
  return this_ptr->toUShort(*s);
}

unsigned short qt_core_c_QLocale_toUShort_QStringRef(const QLocale* this_ptr, const QStringRef* s) {
  return this_ptr->toUShort(*s);
}

unsigned short qt_core_c_QLocale_toUShort_QStringRef_bool(const QLocale* this_ptr, const QStringRef* s, bool* ok) {
  return this_ptr->toUShort(*s, ok);
}

unsigned short qt_core_c_QLocale_toUShort_QString_bool(const QLocale* this_ptr, const QString* s, bool* ok) {
  return this_ptr->toUShort(*s, ok);
}

void qt_core_c_QLocale_toUpper_to_output(const QLocale* this_ptr, const QString* str, QString* output) {
  new(output) QString(this_ptr->toUpper(*str));
}

void qt_core_c_QLocale_uiLanguages_to_output(const QLocale* this_ptr, QStringList* output) {
  new(output) QStringList(this_ptr->uiLanguages());
}

void qt_core_c_QLocale_weekdays_to_output(const QLocale* this_ptr, QList< Qt::DayOfWeek >* output) {
  new(output) QList< Qt::DayOfWeek >(this_ptr->weekdays());
}

void qt_core_c_QLocale_zeroDigit_to_output(const QLocale* this_ptr, QChar* output) {
  new(output) QChar(this_ptr->zeroDigit());
}

