#include "qt_gui_c_QRawFont.h"

unsigned int qt_gui_c_QRawFont_G_qHash_font(const QRawFont* font) {
  return qHash(*font);
}

unsigned int qt_gui_c_QRawFont_G_qHash_font_seed(const QRawFont* font, unsigned int seed) {
  return qHash(*font, seed);
}

void qt_gui_c_QRawFont_G_swap(QRawFont* value1, QRawFont* value2) {
  swap(*value1, *value2);
}

bool qt_gui_c_QRawFont_advancesForGlyphIndexes_glyphIndexes_advances_numGlyphs(const QRawFont* this_ptr, const quint32* glyphIndexes, QPointF* advances, int numGlyphs) {
  return this_ptr->advancesForGlyphIndexes(glyphIndexes, advances, numGlyphs);
}

bool qt_gui_c_QRawFont_advancesForGlyphIndexes_glyphIndexes_advances_numGlyphs_layoutFlags(const QRawFont* this_ptr, const quint32* glyphIndexes, QPointF* advances, int numGlyphs, unsigned int layoutFlags) {
  return this_ptr->advancesForGlyphIndexes(glyphIndexes, advances, numGlyphs, QFlags< QRawFont::LayoutFlag >(layoutFlags));
}

void qt_gui_c_QRawFont_advancesForGlyphIndexes_to_output_glyphIndexes(const QRawFont* this_ptr, const QVector< quint32 >* glyphIndexes, QVector< QPointF >* output) {
  new(output) QVector< QPointF >(this_ptr->advancesForGlyphIndexes(*glyphIndexes));
}

void qt_gui_c_QRawFont_advancesForGlyphIndexes_to_output_glyphIndexes_layoutFlags(const QRawFont* this_ptr, const QVector< quint32 >* glyphIndexes, unsigned int layoutFlags, QVector< QPointF >* output) {
  new(output) QVector< QPointF >(this_ptr->advancesForGlyphIndexes(*glyphIndexes, QFlags< QRawFont::LayoutFlag >(layoutFlags)));
}

QImage* qt_gui_c_QRawFont_alphaMapForGlyph_as_ptr_glyphIndex(const QRawFont* this_ptr, quint32 glyphIndex) {
  return new QImage(this_ptr->alphaMapForGlyph(glyphIndex));
}

QImage* qt_gui_c_QRawFont_alphaMapForGlyph_as_ptr_glyphIndex_antialiasingType(const QRawFont* this_ptr, quint32 glyphIndex, QRawFont::AntialiasingType antialiasingType) {
  return new QImage(this_ptr->alphaMapForGlyph(glyphIndex, antialiasingType));
}

QImage* qt_gui_c_QRawFont_alphaMapForGlyph_as_ptr_glyphIndex_antialiasingType_transform(const QRawFont* this_ptr, quint32 glyphIndex, QRawFont::AntialiasingType antialiasingType, const QTransform* transform) {
  return new QImage(this_ptr->alphaMapForGlyph(glyphIndex, antialiasingType, *transform));
}

double qt_gui_c_QRawFont_ascent(const QRawFont* this_ptr) {
  return this_ptr->ascent();
}

double qt_gui_c_QRawFont_averageCharWidth(const QRawFont* this_ptr) {
  return this_ptr->averageCharWidth();
}

void qt_gui_c_QRawFont_boundingRect_to_output(const QRawFont* this_ptr, quint32 glyphIndex, QRectF* output) {
  new(output) QRectF(this_ptr->boundingRect(glyphIndex));
}

double qt_gui_c_QRawFont_capHeight(const QRawFont* this_ptr) {
  return this_ptr->capHeight();
}

void qt_gui_c_QRawFont_constructor_fileName_pixelSize(const QString* fileName, double pixelSize, QRawFont* output) {
  new(output) QRawFont(*fileName, pixelSize);
}

void qt_gui_c_QRawFont_constructor_fileName_pixelSize_hintingPreference(const QString* fileName, double pixelSize, const QFont::HintingPreference* hintingPreference, QRawFont* output) {
  new(output) QRawFont(*fileName, pixelSize, *hintingPreference);
}

void qt_gui_c_QRawFont_constructor_fontData_pixelSize(const QByteArray* fontData, double pixelSize, QRawFont* output) {
  new(output) QRawFont(*fontData, pixelSize);
}

void qt_gui_c_QRawFont_constructor_fontData_pixelSize_hintingPreference(const QByteArray* fontData, double pixelSize, const QFont::HintingPreference* hintingPreference, QRawFont* output) {
  new(output) QRawFont(*fontData, pixelSize, *hintingPreference);
}

void qt_gui_c_QRawFont_constructor_no_args(QRawFont* output) {
  new(output) QRawFont();
}

void qt_gui_c_QRawFont_constructor_other(const QRawFont* other, QRawFont* output) {
  new(output) QRawFont(*other);
}

double qt_gui_c_QRawFont_descent(const QRawFont* this_ptr) {
  return this_ptr->descent();
}

void qt_gui_c_QRawFont_destructor(QRawFont* this_ptr) {
  qt_gui_c_call_destructor(this_ptr);
}

void qt_gui_c_QRawFont_familyName_to_output(const QRawFont* this_ptr, QString* output) {
  new(output) QString(this_ptr->familyName());
}

void qt_gui_c_QRawFont_fontTable_to_output(const QRawFont* this_ptr, const char* tagName, QByteArray* output) {
  new(output) QByteArray(this_ptr->fontTable(tagName));
}

void qt_gui_c_QRawFont_fromFont_to_output_font(const QFont* font, QRawFont* output) {
  new(output) QRawFont(QRawFont::fromFont(*font));
}

void qt_gui_c_QRawFont_fromFont_to_output_font_writingSystem(const QFont* font, const QFontDatabase::WritingSystem* writingSystem, QRawFont* output) {
  new(output) QRawFont(QRawFont::fromFont(*font, *writingSystem));
}

bool qt_gui_c_QRawFont_glyphIndexesForChars(const QRawFont* this_ptr, const QChar* chars, int numChars, quint32* glyphIndexes, int* numGlyphs) {
  return this_ptr->glyphIndexesForChars(chars, numChars, glyphIndexes, numGlyphs);
}

void qt_gui_c_QRawFont_glyphIndexesForString_to_output(const QRawFont* this_ptr, const QString* text, QVector< quint32 >* output) {
  new(output) QVector< quint32 >(this_ptr->glyphIndexesForString(*text));
}

bool qt_gui_c_QRawFont_isValid(const QRawFont* this_ptr) {
  return this_ptr->isValid();
}

double qt_gui_c_QRawFont_leading(const QRawFont* this_ptr) {
  return this_ptr->leading();
}

double qt_gui_c_QRawFont_lineThickness(const QRawFont* this_ptr) {
  return this_ptr->lineThickness();
}

void qt_gui_c_QRawFont_loadFromData(QRawFont* this_ptr, const QByteArray* fontData, double pixelSize, const QFont::HintingPreference* hintingPreference) {
  this_ptr->loadFromData(*fontData, pixelSize, *hintingPreference);
}

void qt_gui_c_QRawFont_loadFromFile(QRawFont* this_ptr, const QString* fileName, double pixelSize, const QFont::HintingPreference* hintingPreference) {
  this_ptr->loadFromFile(*fileName, pixelSize, *hintingPreference);
}

double qt_gui_c_QRawFont_maxCharWidth(const QRawFont* this_ptr) {
  return this_ptr->maxCharWidth();
}

QRawFont* qt_gui_c_QRawFont_operator_assign(QRawFont* this_ptr, const QRawFont* other) {
  return &this_ptr->operator=(*other);
}

bool qt_gui_c_QRawFont_operator_eq(const QRawFont* this_ptr, const QRawFont* other) {
  return this_ptr->operator==(*other);
}

bool qt_gui_c_QRawFont_operator_neq(const QRawFont* this_ptr, const QRawFont* other) {
  return this_ptr->operator!=(*other);
}

void qt_gui_c_QRawFont_pathForGlyph_to_output(const QRawFont* this_ptr, quint32 glyphIndex, QPainterPath* output) {
  new(output) QPainterPath(this_ptr->pathForGlyph(glyphIndex));
}

double qt_gui_c_QRawFont_pixelSize(const QRawFont* this_ptr) {
  return this_ptr->pixelSize();
}

void qt_gui_c_QRawFont_setPixelSize(QRawFont* this_ptr, double pixelSize) {
  this_ptr->setPixelSize(pixelSize);
}

void qt_gui_c_QRawFont_styleName_to_output(const QRawFont* this_ptr, QString* output) {
  new(output) QString(this_ptr->styleName());
}

void qt_gui_c_QRawFont_supportedWritingSystems_to_output(const QRawFont* this_ptr, QList< QFontDatabase::WritingSystem >* output) {
  new(output) QList< QFontDatabase::WritingSystem >(this_ptr->supportedWritingSystems());
}

bool qt_gui_c_QRawFont_supportsCharacter_character(const QRawFont* this_ptr, const QChar* character) {
  return this_ptr->supportsCharacter(*character);
}

bool qt_gui_c_QRawFont_supportsCharacter_ucs4(const QRawFont* this_ptr, unsigned int ucs4) {
  return this_ptr->supportsCharacter(ucs4);
}

void qt_gui_c_QRawFont_swap(QRawFont* this_ptr, QRawFont* other) {
  this_ptr->swap(*other);
}

double qt_gui_c_QRawFont_underlinePosition(const QRawFont* this_ptr) {
  return this_ptr->underlinePosition();
}

double qt_gui_c_QRawFont_unitsPerEm(const QRawFont* this_ptr) {
  return this_ptr->unitsPerEm();
}

int qt_gui_c_QRawFont_weight(const QRawFont* this_ptr) {
  return this_ptr->weight();
}

double qt_gui_c_QRawFont_xHeight(const QRawFont* this_ptr) {
  return this_ptr->xHeight();
}

