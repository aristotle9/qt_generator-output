#include "qt_gui_c_QFont.h"

void qt_gui_c_QFont_G_operator_shl_to_output(const QDebug* arg1, const QFont* arg2, QDebug* output) {
  new(output) QDebug(operator<<(*arg1, *arg2));
}

unsigned int qt_gui_c_QFont_G_qHash_font(const QFont* font) {
  return qHash(*font);
}

unsigned int qt_gui_c_QFont_G_qHash_font_seed(const QFont* font, unsigned int seed) {
  return qHash(*font, seed);
}

void qt_gui_c_QFont_G_swap(QFont* value1, QFont* value2) {
  swap(*value1, *value2);
}

bool qt_gui_c_QFont_bold(const QFont* this_ptr) {
  return this_ptr->bold();
}

void qt_gui_c_QFont_cacheStatistics() {
  QFont::cacheStatistics();
}

QFont::Capitalization qt_gui_c_QFont_capitalization(const QFont* this_ptr) {
  return this_ptr->capitalization();
}

void qt_gui_c_QFont_cleanup() {
  QFont::cleanup();
}

void qt_gui_c_QFont_constructor_arg1(const QFont* arg1, QFont* output) {
  new(output) QFont(*arg1);
}

void qt_gui_c_QFont_constructor_arg1_pd(const QFont* arg1, QPaintDevice* pd, QFont* output) {
  new(output) QFont(*arg1, pd);
}

void qt_gui_c_QFont_constructor_family(const QString* family, QFont* output) {
  new(output) QFont(*family);
}

void qt_gui_c_QFont_constructor_family_pointSize(const QString* family, int pointSize, QFont* output) {
  new(output) QFont(*family, pointSize);
}

void qt_gui_c_QFont_constructor_family_pointSize_weight(const QString* family, int pointSize, int weight, QFont* output) {
  new(output) QFont(*family, pointSize, weight);
}

void qt_gui_c_QFont_constructor_family_pointSize_weight_italic(const QString* family, int pointSize, int weight, bool italic, QFont* output) {
  new(output) QFont(*family, pointSize, weight, italic);
}

void qt_gui_c_QFont_constructor_no_args(QFont* output) {
  new(output) QFont();
}

void qt_gui_c_QFont_convert_to_QVariant_to_output(const QFont* this_ptr, QVariant* output) {
  new(output) QVariant(this_ptr->operator QVariant());
}

void qt_gui_c_QFont_defaultFamily_to_output(const QFont* this_ptr, QString* output) {
  new(output) QString(this_ptr->defaultFamily());
}

void qt_gui_c_QFont_destructor(QFont* this_ptr) {
  qt_gui_c_call_destructor(this_ptr);
}

bool qt_gui_c_QFont_exactMatch(const QFont* this_ptr) {
  return this_ptr->exactMatch();
}

void qt_gui_c_QFont_family_to_output(const QFont* this_ptr, QString* output) {
  new(output) QString(this_ptr->family());
}

bool qt_gui_c_QFont_fixedPitch(const QFont* this_ptr) {
  return this_ptr->fixedPitch();
}

bool qt_gui_c_QFont_fromString(QFont* this_ptr, const QString* arg1) {
  return this_ptr->fromString(*arg1);
}

QFont::HintingPreference qt_gui_c_QFont_hintingPreference(const QFont* this_ptr) {
  return this_ptr->hintingPreference();
}

void qt_gui_c_QFont_initialize() {
  QFont::initialize();
}

void qt_gui_c_QFont_insertSubstitution(const QString* arg1, const QString* arg2) {
  QFont::insertSubstitution(*arg1, *arg2);
}

void qt_gui_c_QFont_insertSubstitutions(const QString* arg1, const QStringList* arg2) {
  QFont::insertSubstitutions(*arg1, *arg2);
}

bool qt_gui_c_QFont_isCopyOf(const QFont* this_ptr, const QFont* arg1) {
  return this_ptr->isCopyOf(*arg1);
}

bool qt_gui_c_QFont_italic(const QFont* this_ptr) {
  return this_ptr->italic();
}

bool qt_gui_c_QFont_kerning(const QFont* this_ptr) {
  return this_ptr->kerning();
}

void qt_gui_c_QFont_key_to_output(const QFont* this_ptr, QString* output) {
  new(output) QString(this_ptr->key());
}

void qt_gui_c_QFont_lastResortFamily_to_output(const QFont* this_ptr, QString* output) {
  new(output) QString(this_ptr->lastResortFamily());
}

void qt_gui_c_QFont_lastResortFont_to_output(const QFont* this_ptr, QString* output) {
  new(output) QString(this_ptr->lastResortFont());
}

double qt_gui_c_QFont_letterSpacing(const QFont* this_ptr) {
  return this_ptr->letterSpacing();
}

QFont::SpacingType qt_gui_c_QFont_letterSpacingType(const QFont* this_ptr) {
  return this_ptr->letterSpacingType();
}

QFont* qt_gui_c_QFont_operator_assign(QFont* this_ptr, const QFont* arg1) {
  return &this_ptr->operator=(*arg1);
}

bool qt_gui_c_QFont_operator_eq(const QFont* this_ptr, const QFont* arg1) {
  return this_ptr->operator==(*arg1);
}

bool qt_gui_c_QFont_operator_lt(const QFont* this_ptr, const QFont* arg1) {
  return this_ptr->operator<(*arg1);
}

bool qt_gui_c_QFont_operator_neq(const QFont* this_ptr, const QFont* arg1) {
  return this_ptr->operator!=(*arg1);
}

bool qt_gui_c_QFont_overline(const QFont* this_ptr) {
  return this_ptr->overline();
}

int qt_gui_c_QFont_pixelSize(const QFont* this_ptr) {
  return this_ptr->pixelSize();
}

int qt_gui_c_QFont_pointSize(const QFont* this_ptr) {
  return this_ptr->pointSize();
}

double qt_gui_c_QFont_pointSizeF(const QFont* this_ptr) {
  return this_ptr->pointSizeF();
}

bool qt_gui_c_QFont_rawMode(const QFont* this_ptr) {
  return this_ptr->rawMode();
}

void qt_gui_c_QFont_rawName_to_output(const QFont* this_ptr, QString* output) {
  new(output) QString(this_ptr->rawName());
}

void qt_gui_c_QFont_removeSubstitutions(const QString* arg1) {
  QFont::removeSubstitutions(*arg1);
}

void qt_gui_c_QFont_resolve_mask(QFont* this_ptr, unsigned int mask) {
  this_ptr->resolve(mask);
}

unsigned int qt_gui_c_QFont_resolve_no_args(const QFont* this_ptr) {
  return this_ptr->resolve();
}

void qt_gui_c_QFont_resolve_to_output(const QFont* this_ptr, const QFont* arg1, QFont* output) {
  new(output) QFont(this_ptr->resolve(*arg1));
}

void qt_gui_c_QFont_setBold(QFont* this_ptr, bool arg1) {
  this_ptr->setBold(arg1);
}

void qt_gui_c_QFont_setCapitalization(QFont* this_ptr, QFont::Capitalization arg1) {
  this_ptr->setCapitalization(arg1);
}

void qt_gui_c_QFont_setFamily(QFont* this_ptr, const QString* arg1) {
  this_ptr->setFamily(*arg1);
}

void qt_gui_c_QFont_setFixedPitch(QFont* this_ptr, bool arg1) {
  this_ptr->setFixedPitch(arg1);
}

void qt_gui_c_QFont_setHintingPreference(QFont* this_ptr, QFont::HintingPreference hintingPreference) {
  this_ptr->setHintingPreference(hintingPreference);
}

void qt_gui_c_QFont_setItalic(QFont* this_ptr, bool b) {
  this_ptr->setItalic(b);
}

void qt_gui_c_QFont_setKerning(QFont* this_ptr, bool arg1) {
  this_ptr->setKerning(arg1);
}

void qt_gui_c_QFont_setLetterSpacing(QFont* this_ptr, QFont::SpacingType type, double spacing) {
  this_ptr->setLetterSpacing(type, spacing);
}

void qt_gui_c_QFont_setOverline(QFont* this_ptr, bool arg1) {
  this_ptr->setOverline(arg1);
}

void qt_gui_c_QFont_setPixelSize(QFont* this_ptr, int arg1) {
  this_ptr->setPixelSize(arg1);
}

void qt_gui_c_QFont_setPointSize(QFont* this_ptr, int arg1) {
  this_ptr->setPointSize(arg1);
}

void qt_gui_c_QFont_setPointSizeF(QFont* this_ptr, double arg1) {
  this_ptr->setPointSizeF(arg1);
}

void qt_gui_c_QFont_setRawMode(QFont* this_ptr, bool arg1) {
  this_ptr->setRawMode(arg1);
}

void qt_gui_c_QFont_setRawName(QFont* this_ptr, const QString* arg1) {
  this_ptr->setRawName(*arg1);
}

void qt_gui_c_QFont_setStretch(QFont* this_ptr, int arg1) {
  this_ptr->setStretch(arg1);
}

void qt_gui_c_QFont_setStrikeOut(QFont* this_ptr, bool arg1) {
  this_ptr->setStrikeOut(arg1);
}

void qt_gui_c_QFont_setStyle(QFont* this_ptr, QFont::Style style) {
  this_ptr->setStyle(style);
}

void qt_gui_c_QFont_setStyleHint_arg1(QFont* this_ptr, QFont::StyleHint arg1) {
  this_ptr->setStyleHint(arg1);
}

void qt_gui_c_QFont_setStyleHint_arg1_arg2(QFont* this_ptr, QFont::StyleHint arg1, QFont::StyleStrategy arg2) {
  this_ptr->setStyleHint(arg1, arg2);
}

void qt_gui_c_QFont_setStyleName(QFont* this_ptr, const QString* arg1) {
  this_ptr->setStyleName(*arg1);
}

void qt_gui_c_QFont_setStyleStrategy(QFont* this_ptr, QFont::StyleStrategy s) {
  this_ptr->setStyleStrategy(s);
}

void qt_gui_c_QFont_setUnderline(QFont* this_ptr, bool arg1) {
  this_ptr->setUnderline(arg1);
}

void qt_gui_c_QFont_setWeight(QFont* this_ptr, int arg1) {
  this_ptr->setWeight(arg1);
}

void qt_gui_c_QFont_setWordSpacing(QFont* this_ptr, double spacing) {
  this_ptr->setWordSpacing(spacing);
}

int qt_gui_c_QFont_stretch(const QFont* this_ptr) {
  return this_ptr->stretch();
}

bool qt_gui_c_QFont_strikeOut(const QFont* this_ptr) {
  return this_ptr->strikeOut();
}

QFont::Style qt_gui_c_QFont_style(const QFont* this_ptr) {
  return this_ptr->style();
}

QFont::StyleHint qt_gui_c_QFont_styleHint(const QFont* this_ptr) {
  return this_ptr->styleHint();
}

void qt_gui_c_QFont_styleName_to_output(const QFont* this_ptr, QString* output) {
  new(output) QString(this_ptr->styleName());
}

QFont::StyleStrategy qt_gui_c_QFont_styleStrategy(const QFont* this_ptr) {
  return this_ptr->styleStrategy();
}

void qt_gui_c_QFont_substitute_to_output(const QString* arg1, QString* output) {
  new(output) QString(QFont::substitute(*arg1));
}

void qt_gui_c_QFont_substitutes_to_output(const QString* arg1, QStringList* output) {
  new(output) QStringList(QFont::substitutes(*arg1));
}

void qt_gui_c_QFont_substitutions_to_output(QStringList* output) {
  new(output) QStringList(QFont::substitutions());
}

void qt_gui_c_QFont_swap(QFont* this_ptr, QFont* other) {
  this_ptr->swap(*other);
}

void qt_gui_c_QFont_toString_to_output(const QFont* this_ptr, QString* output) {
  new(output) QString(this_ptr->toString());
}

bool qt_gui_c_QFont_underline(const QFont* this_ptr) {
  return this_ptr->underline();
}

int qt_gui_c_QFont_weight(const QFont* this_ptr) {
  return this_ptr->weight();
}

double qt_gui_c_QFont_wordSpacing(const QFont* this_ptr) {
  return this_ptr->wordSpacing();
}

