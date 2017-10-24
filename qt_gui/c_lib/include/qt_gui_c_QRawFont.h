#ifndef QT_GUI_C_QRAWFONT_H
#define QT_GUI_C_QRAWFONT_H

#include "qt_gui_c_global.h"

extern "C" {

QT_GUI_C_EXPORT unsigned int qt_gui_c_QRawFont_G_qHash_font(const QRawFont* font);
QT_GUI_C_EXPORT unsigned int qt_gui_c_QRawFont_G_qHash_font_seed(const QRawFont* font, unsigned int seed);
QT_GUI_C_EXPORT void qt_gui_c_QRawFont_G_swap(QRawFont* value1, QRawFont* value2);
QT_GUI_C_EXPORT bool qt_gui_c_QRawFont_advancesForGlyphIndexes_glyphIndexes_advances_numGlyphs(const QRawFont* this_ptr, const quint32* glyphIndexes, QPointF* advances, int numGlyphs);
QT_GUI_C_EXPORT bool qt_gui_c_QRawFont_advancesForGlyphIndexes_glyphIndexes_advances_numGlyphs_layoutFlags(const QRawFont* this_ptr, const quint32* glyphIndexes, QPointF* advances, int numGlyphs, unsigned int layoutFlags);
QT_GUI_C_EXPORT void qt_gui_c_QRawFont_advancesForGlyphIndexes_to_output_glyphIndexes(const QRawFont* this_ptr, const QVector< quint32 >* glyphIndexes, QVector< QPointF >* output);
QT_GUI_C_EXPORT void qt_gui_c_QRawFont_advancesForGlyphIndexes_to_output_glyphIndexes_layoutFlags(const QRawFont* this_ptr, const QVector< quint32 >* glyphIndexes, unsigned int layoutFlags, QVector< QPointF >* output);
QT_GUI_C_EXPORT QImage* qt_gui_c_QRawFont_alphaMapForGlyph_as_ptr_glyphIndex(const QRawFont* this_ptr, quint32 glyphIndex);
QT_GUI_C_EXPORT QImage* qt_gui_c_QRawFont_alphaMapForGlyph_as_ptr_glyphIndex_antialiasingType(const QRawFont* this_ptr, quint32 glyphIndex, QRawFont::AntialiasingType antialiasingType);
QT_GUI_C_EXPORT QImage* qt_gui_c_QRawFont_alphaMapForGlyph_as_ptr_glyphIndex_antialiasingType_transform(const QRawFont* this_ptr, quint32 glyphIndex, QRawFont::AntialiasingType antialiasingType, const QTransform* transform);
QT_GUI_C_EXPORT double qt_gui_c_QRawFont_ascent(const QRawFont* this_ptr);
QT_GUI_C_EXPORT double qt_gui_c_QRawFont_averageCharWidth(const QRawFont* this_ptr);
QT_GUI_C_EXPORT void qt_gui_c_QRawFont_boundingRect_to_output(const QRawFont* this_ptr, quint32 glyphIndex, QRectF* output);
QT_GUI_C_EXPORT double qt_gui_c_QRawFont_capHeight(const QRawFont* this_ptr);
QT_GUI_C_EXPORT void qt_gui_c_QRawFont_constructor_fileName_pixelSize(const QString* fileName, double pixelSize, QRawFont* output);
QT_GUI_C_EXPORT void qt_gui_c_QRawFont_constructor_fileName_pixelSize_hintingPreference(const QString* fileName, double pixelSize, const QFont::HintingPreference* hintingPreference, QRawFont* output);
QT_GUI_C_EXPORT void qt_gui_c_QRawFont_constructor_fontData_pixelSize(const QByteArray* fontData, double pixelSize, QRawFont* output);
QT_GUI_C_EXPORT void qt_gui_c_QRawFont_constructor_fontData_pixelSize_hintingPreference(const QByteArray* fontData, double pixelSize, const QFont::HintingPreference* hintingPreference, QRawFont* output);
QT_GUI_C_EXPORT void qt_gui_c_QRawFont_constructor_no_args(QRawFont* output);
QT_GUI_C_EXPORT void qt_gui_c_QRawFont_constructor_other(const QRawFont* other, QRawFont* output);
QT_GUI_C_EXPORT double qt_gui_c_QRawFont_descent(const QRawFont* this_ptr);
QT_GUI_C_EXPORT void qt_gui_c_QRawFont_destructor(QRawFont* this_ptr);
QT_GUI_C_EXPORT void qt_gui_c_QRawFont_familyName_to_output(const QRawFont* this_ptr, QString* output);
QT_GUI_C_EXPORT void qt_gui_c_QRawFont_fontTable_to_output(const QRawFont* this_ptr, const char* tagName, QByteArray* output);
QT_GUI_C_EXPORT void qt_gui_c_QRawFont_fromFont_to_output_font(const QFont* font, QRawFont* output);
QT_GUI_C_EXPORT void qt_gui_c_QRawFont_fromFont_to_output_font_writingSystem(const QFont* font, const QFontDatabase::WritingSystem* writingSystem, QRawFont* output);
QT_GUI_C_EXPORT bool qt_gui_c_QRawFont_glyphIndexesForChars(const QRawFont* this_ptr, const QChar* chars, int numChars, quint32* glyphIndexes, int* numGlyphs);
QT_GUI_C_EXPORT void qt_gui_c_QRawFont_glyphIndexesForString_to_output(const QRawFont* this_ptr, const QString* text, QVector< quint32 >* output);
QT_GUI_C_EXPORT bool qt_gui_c_QRawFont_isValid(const QRawFont* this_ptr);
QT_GUI_C_EXPORT double qt_gui_c_QRawFont_leading(const QRawFont* this_ptr);
QT_GUI_C_EXPORT double qt_gui_c_QRawFont_lineThickness(const QRawFont* this_ptr);
QT_GUI_C_EXPORT void qt_gui_c_QRawFont_loadFromData(QRawFont* this_ptr, const QByteArray* fontData, double pixelSize, const QFont::HintingPreference* hintingPreference);
QT_GUI_C_EXPORT void qt_gui_c_QRawFont_loadFromFile(QRawFont* this_ptr, const QString* fileName, double pixelSize, const QFont::HintingPreference* hintingPreference);
QT_GUI_C_EXPORT double qt_gui_c_QRawFont_maxCharWidth(const QRawFont* this_ptr);
QT_GUI_C_EXPORT QRawFont* qt_gui_c_QRawFont_operator_assign(QRawFont* this_ptr, const QRawFont* other);
QT_GUI_C_EXPORT bool qt_gui_c_QRawFont_operator_eq(const QRawFont* this_ptr, const QRawFont* other);
QT_GUI_C_EXPORT bool qt_gui_c_QRawFont_operator_neq(const QRawFont* this_ptr, const QRawFont* other);
QT_GUI_C_EXPORT void qt_gui_c_QRawFont_pathForGlyph_to_output(const QRawFont* this_ptr, quint32 glyphIndex, QPainterPath* output);
QT_GUI_C_EXPORT double qt_gui_c_QRawFont_pixelSize(const QRawFont* this_ptr);
QT_GUI_C_EXPORT void qt_gui_c_QRawFont_setPixelSize(QRawFont* this_ptr, double pixelSize);
QT_GUI_C_EXPORT void qt_gui_c_QRawFont_styleName_to_output(const QRawFont* this_ptr, QString* output);
QT_GUI_C_EXPORT void qt_gui_c_QRawFont_supportedWritingSystems_to_output(const QRawFont* this_ptr, QList< QFontDatabase::WritingSystem >* output);
QT_GUI_C_EXPORT bool qt_gui_c_QRawFont_supportsCharacter_character(const QRawFont* this_ptr, const QChar* character);
QT_GUI_C_EXPORT bool qt_gui_c_QRawFont_supportsCharacter_ucs4(const QRawFont* this_ptr, unsigned int ucs4);
QT_GUI_C_EXPORT void qt_gui_c_QRawFont_swap(QRawFont* this_ptr, QRawFont* other);
QT_GUI_C_EXPORT double qt_gui_c_QRawFont_underlinePosition(const QRawFont* this_ptr);
QT_GUI_C_EXPORT double qt_gui_c_QRawFont_unitsPerEm(const QRawFont* this_ptr);
QT_GUI_C_EXPORT int qt_gui_c_QRawFont_weight(const QRawFont* this_ptr);
QT_GUI_C_EXPORT double qt_gui_c_QRawFont_xHeight(const QRawFont* this_ptr);

} // extern "C"

#endif // QT_GUI_C_QRAWFONT_H
