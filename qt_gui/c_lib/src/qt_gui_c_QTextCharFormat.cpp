#include "qt_gui_c_QTextCharFormat.h"

QTextCharFormat* qt_gui_c_QTextCharFormat_G_static_cast_QTextCharFormat_ptr(QTextFormat* ptr) {
  return static_cast<QTextCharFormat*>(ptr);
}

QTextFormat* qt_gui_c_QTextCharFormat_G_static_cast_QTextFormat_ptr(QTextCharFormat* ptr) {
  return static_cast<QTextFormat*>(ptr);
}

void qt_gui_c_QTextCharFormat_anchorHref_to_output(const QTextCharFormat* this_ptr, QString* output) {
  new(output) QString(this_ptr->anchorHref());
}

void qt_gui_c_QTextCharFormat_anchorName_to_output(const QTextCharFormat* this_ptr, QString* output) {
  new(output) QString(this_ptr->anchorName());
}

void qt_gui_c_QTextCharFormat_anchorNames_to_output(const QTextCharFormat* this_ptr, QStringList* output) {
  new(output) QStringList(this_ptr->anchorNames());
}

void qt_gui_c_QTextCharFormat_constructor(QTextCharFormat* output) {
  new(output) QTextCharFormat();
}

void qt_gui_c_QTextCharFormat_destructor(QTextCharFormat* this_ptr) {
  qt_gui_c_call_destructor(this_ptr);
}

void qt_gui_c_QTextCharFormat_fontFamily_to_output(const QTextCharFormat* this_ptr, QString* output) {
  new(output) QString(this_ptr->fontFamily());
}

bool qt_gui_c_QTextCharFormat_fontFixedPitch(const QTextCharFormat* this_ptr) {
  return this_ptr->fontFixedPitch();
}

bool qt_gui_c_QTextCharFormat_fontItalic(const QTextCharFormat* this_ptr) {
  return this_ptr->fontItalic();
}

bool qt_gui_c_QTextCharFormat_fontKerning(const QTextCharFormat* this_ptr) {
  return this_ptr->fontKerning();
}

double qt_gui_c_QTextCharFormat_fontLetterSpacing(const QTextCharFormat* this_ptr) {
  return this_ptr->fontLetterSpacing();
}

bool qt_gui_c_QTextCharFormat_fontOverline(const QTextCharFormat* this_ptr) {
  return this_ptr->fontOverline();
}

double qt_gui_c_QTextCharFormat_fontPointSize(const QTextCharFormat* this_ptr) {
  return this_ptr->fontPointSize();
}

int qt_gui_c_QTextCharFormat_fontStretch(const QTextCharFormat* this_ptr) {
  return this_ptr->fontStretch();
}

bool qt_gui_c_QTextCharFormat_fontStrikeOut(const QTextCharFormat* this_ptr) {
  return this_ptr->fontStrikeOut();
}

bool qt_gui_c_QTextCharFormat_fontUnderline(const QTextCharFormat* this_ptr) {
  return this_ptr->fontUnderline();
}

int qt_gui_c_QTextCharFormat_fontWeight(const QTextCharFormat* this_ptr) {
  return this_ptr->fontWeight();
}

double qt_gui_c_QTextCharFormat_fontWordSpacing(const QTextCharFormat* this_ptr) {
  return this_ptr->fontWordSpacing();
}

void qt_gui_c_QTextCharFormat_font_to_output(const QTextCharFormat* this_ptr, QFont* output) {
  new(output) QFont(this_ptr->font());
}

bool qt_gui_c_QTextCharFormat_isAnchor(const QTextCharFormat* this_ptr) {
  return this_ptr->isAnchor();
}

bool qt_gui_c_QTextCharFormat_isValid(const QTextCharFormat* this_ptr) {
  return this_ptr->isValid();
}

void qt_gui_c_QTextCharFormat_setAnchor(QTextCharFormat* this_ptr, bool anchor) {
  this_ptr->setAnchor(anchor);
}

void qt_gui_c_QTextCharFormat_setAnchorHref(QTextCharFormat* this_ptr, const QString* value) {
  this_ptr->setAnchorHref(*value);
}

void qt_gui_c_QTextCharFormat_setAnchorName(QTextCharFormat* this_ptr, const QString* name) {
  this_ptr->setAnchorName(*name);
}

void qt_gui_c_QTextCharFormat_setAnchorNames(QTextCharFormat* this_ptr, const QStringList* names) {
  this_ptr->setAnchorNames(*names);
}

void qt_gui_c_QTextCharFormat_setFontCapitalization(QTextCharFormat* this_ptr, const QFont::Capitalization* capitalization) {
  this_ptr->setFontCapitalization(*capitalization);
}

void qt_gui_c_QTextCharFormat_setFontFamily(QTextCharFormat* this_ptr, const QString* family) {
  this_ptr->setFontFamily(*family);
}

void qt_gui_c_QTextCharFormat_setFontFixedPitch(QTextCharFormat* this_ptr, bool fixedPitch) {
  this_ptr->setFontFixedPitch(fixedPitch);
}

void qt_gui_c_QTextCharFormat_setFontHintingPreference(QTextCharFormat* this_ptr, const QFont::HintingPreference* hintingPreference) {
  this_ptr->setFontHintingPreference(*hintingPreference);
}

void qt_gui_c_QTextCharFormat_setFontItalic(QTextCharFormat* this_ptr, bool italic) {
  this_ptr->setFontItalic(italic);
}

void qt_gui_c_QTextCharFormat_setFontKerning(QTextCharFormat* this_ptr, bool enable) {
  this_ptr->setFontKerning(enable);
}

void qt_gui_c_QTextCharFormat_setFontLetterSpacing(QTextCharFormat* this_ptr, double spacing) {
  this_ptr->setFontLetterSpacing(spacing);
}

void qt_gui_c_QTextCharFormat_setFontLetterSpacingType(QTextCharFormat* this_ptr, const QFont::SpacingType* letterSpacingType) {
  this_ptr->setFontLetterSpacingType(*letterSpacingType);
}

void qt_gui_c_QTextCharFormat_setFontOverline(QTextCharFormat* this_ptr, bool overline) {
  this_ptr->setFontOverline(overline);
}

void qt_gui_c_QTextCharFormat_setFontPointSize(QTextCharFormat* this_ptr, double size) {
  this_ptr->setFontPointSize(size);
}

void qt_gui_c_QTextCharFormat_setFontStretch(QTextCharFormat* this_ptr, int factor) {
  this_ptr->setFontStretch(factor);
}

void qt_gui_c_QTextCharFormat_setFontStrikeOut(QTextCharFormat* this_ptr, bool strikeOut) {
  this_ptr->setFontStrikeOut(strikeOut);
}

void qt_gui_c_QTextCharFormat_setFontStyleHint_hint(QTextCharFormat* this_ptr, const QFont::StyleHint* hint) {
  this_ptr->setFontStyleHint(*hint);
}

void qt_gui_c_QTextCharFormat_setFontStyleHint_hint_strategy(QTextCharFormat* this_ptr, const QFont::StyleHint* hint, const QFont::StyleStrategy* strategy) {
  this_ptr->setFontStyleHint(*hint, *strategy);
}

void qt_gui_c_QTextCharFormat_setFontStyleStrategy(QTextCharFormat* this_ptr, const QFont::StyleStrategy* strategy) {
  this_ptr->setFontStyleStrategy(*strategy);
}

void qt_gui_c_QTextCharFormat_setFontUnderline(QTextCharFormat* this_ptr, bool underline) {
  this_ptr->setFontUnderline(underline);
}

void qt_gui_c_QTextCharFormat_setFontWeight(QTextCharFormat* this_ptr, int weight) {
  this_ptr->setFontWeight(weight);
}

void qt_gui_c_QTextCharFormat_setFontWordSpacing(QTextCharFormat* this_ptr, double spacing) {
  this_ptr->setFontWordSpacing(spacing);
}

void qt_gui_c_QTextCharFormat_setFont_font(QTextCharFormat* this_ptr, const QFont* font) {
  this_ptr->setFont(*font);
}

void qt_gui_c_QTextCharFormat_setFont_font_behavior(QTextCharFormat* this_ptr, const QFont* font, QTextCharFormat::FontPropertiesInheritanceBehavior behavior) {
  this_ptr->setFont(*font, behavior);
}

void qt_gui_c_QTextCharFormat_setTableCellColumnSpan(QTextCharFormat* this_ptr, int tableCellColumnSpan) {
  this_ptr->setTableCellColumnSpan(tableCellColumnSpan);
}

void qt_gui_c_QTextCharFormat_setTableCellRowSpan(QTextCharFormat* this_ptr, int tableCellRowSpan) {
  this_ptr->setTableCellRowSpan(tableCellRowSpan);
}

void qt_gui_c_QTextCharFormat_setTextOutline(QTextCharFormat* this_ptr, const QPen* pen) {
  this_ptr->setTextOutline(*pen);
}

void qt_gui_c_QTextCharFormat_setToolTip(QTextCharFormat* this_ptr, const QString* tip) {
  this_ptr->setToolTip(*tip);
}

void qt_gui_c_QTextCharFormat_setUnderlineColor(QTextCharFormat* this_ptr, const QColor* color) {
  this_ptr->setUnderlineColor(*color);
}

void qt_gui_c_QTextCharFormat_setUnderlineStyle(QTextCharFormat* this_ptr, QTextCharFormat::UnderlineStyle style) {
  this_ptr->setUnderlineStyle(style);
}

void qt_gui_c_QTextCharFormat_setVerticalAlignment(QTextCharFormat* this_ptr, QTextCharFormat::VerticalAlignment alignment) {
  this_ptr->setVerticalAlignment(alignment);
}

int qt_gui_c_QTextCharFormat_tableCellColumnSpan(const QTextCharFormat* this_ptr) {
  return this_ptr->tableCellColumnSpan();
}

int qt_gui_c_QTextCharFormat_tableCellRowSpan(const QTextCharFormat* this_ptr) {
  return this_ptr->tableCellRowSpan();
}

void qt_gui_c_QTextCharFormat_textOutline_to_output(const QTextCharFormat* this_ptr, QPen* output) {
  new(output) QPen(this_ptr->textOutline());
}

void qt_gui_c_QTextCharFormat_toolTip_to_output(const QTextCharFormat* this_ptr, QString* output) {
  new(output) QString(this_ptr->toolTip());
}

void qt_gui_c_QTextCharFormat_underlineColor_to_output(const QTextCharFormat* this_ptr, QColor* output) {
  new(output) QColor(this_ptr->underlineColor());
}

QTextCharFormat::UnderlineStyle qt_gui_c_QTextCharFormat_underlineStyle(const QTextCharFormat* this_ptr) {
  return this_ptr->underlineStyle();
}

QTextCharFormat::VerticalAlignment qt_gui_c_QTextCharFormat_verticalAlignment(const QTextCharFormat* this_ptr) {
  return this_ptr->verticalAlignment();
}

