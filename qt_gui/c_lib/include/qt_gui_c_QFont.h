#ifndef QT_GUI_C_QFONT_H
#define QT_GUI_C_QFONT_H

#include "qt_gui_c_global.h"

extern "C" {

QT_GUI_C_EXPORT void qt_gui_c_QFont_G_operator_shl_to_output(const QDebug* arg1, const QFont* arg2, QDebug* output);
QT_GUI_C_EXPORT unsigned int qt_gui_c_QFont_G_qHash_font(const QFont* font);
QT_GUI_C_EXPORT unsigned int qt_gui_c_QFont_G_qHash_font_seed(const QFont* font, unsigned int seed);
QT_GUI_C_EXPORT void qt_gui_c_QFont_G_swap(QFont* value1, QFont* value2);
QT_GUI_C_EXPORT bool qt_gui_c_QFont_bold(const QFont* this_ptr);
QT_GUI_C_EXPORT void qt_gui_c_QFont_cacheStatistics();
QT_GUI_C_EXPORT QFont::Capitalization qt_gui_c_QFont_capitalization(const QFont* this_ptr);
QT_GUI_C_EXPORT void qt_gui_c_QFont_cleanup();
QT_GUI_C_EXPORT void qt_gui_c_QFont_constructor_arg1(const QFont* arg1, QFont* output);
QT_GUI_C_EXPORT void qt_gui_c_QFont_constructor_arg1_pd(const QFont* arg1, QPaintDevice* pd, QFont* output);
QT_GUI_C_EXPORT void qt_gui_c_QFont_constructor_family(const QString* family, QFont* output);
QT_GUI_C_EXPORT void qt_gui_c_QFont_constructor_family_pointSize(const QString* family, int pointSize, QFont* output);
QT_GUI_C_EXPORT void qt_gui_c_QFont_constructor_family_pointSize_weight(const QString* family, int pointSize, int weight, QFont* output);
QT_GUI_C_EXPORT void qt_gui_c_QFont_constructor_family_pointSize_weight_italic(const QString* family, int pointSize, int weight, bool italic, QFont* output);
QT_GUI_C_EXPORT void qt_gui_c_QFont_constructor_no_args(QFont* output);
QT_GUI_C_EXPORT void qt_gui_c_QFont_convert_to_QVariant_to_output(const QFont* this_ptr, QVariant* output);
QT_GUI_C_EXPORT void qt_gui_c_QFont_defaultFamily_to_output(const QFont* this_ptr, QString* output);
QT_GUI_C_EXPORT void qt_gui_c_QFont_destructor(QFont* this_ptr);
QT_GUI_C_EXPORT bool qt_gui_c_QFont_exactMatch(const QFont* this_ptr);
QT_GUI_C_EXPORT void qt_gui_c_QFont_family_to_output(const QFont* this_ptr, QString* output);
QT_GUI_C_EXPORT bool qt_gui_c_QFont_fixedPitch(const QFont* this_ptr);
QT_GUI_C_EXPORT bool qt_gui_c_QFont_fromString(QFont* this_ptr, const QString* arg1);
QT_GUI_C_EXPORT QFont::HintingPreference qt_gui_c_QFont_hintingPreference(const QFont* this_ptr);
QT_GUI_C_EXPORT void qt_gui_c_QFont_initialize();
QT_GUI_C_EXPORT void qt_gui_c_QFont_insertSubstitution(const QString* arg1, const QString* arg2);
QT_GUI_C_EXPORT void qt_gui_c_QFont_insertSubstitutions(const QString* arg1, const QStringList* arg2);
QT_GUI_C_EXPORT bool qt_gui_c_QFont_isCopyOf(const QFont* this_ptr, const QFont* arg1);
QT_GUI_C_EXPORT bool qt_gui_c_QFont_italic(const QFont* this_ptr);
QT_GUI_C_EXPORT bool qt_gui_c_QFont_kerning(const QFont* this_ptr);
QT_GUI_C_EXPORT void qt_gui_c_QFont_key_to_output(const QFont* this_ptr, QString* output);
QT_GUI_C_EXPORT void qt_gui_c_QFont_lastResortFamily_to_output(const QFont* this_ptr, QString* output);
QT_GUI_C_EXPORT void qt_gui_c_QFont_lastResortFont_to_output(const QFont* this_ptr, QString* output);
QT_GUI_C_EXPORT double qt_gui_c_QFont_letterSpacing(const QFont* this_ptr);
QT_GUI_C_EXPORT QFont::SpacingType qt_gui_c_QFont_letterSpacingType(const QFont* this_ptr);
QT_GUI_C_EXPORT QFont* qt_gui_c_QFont_operator_assign(QFont* this_ptr, const QFont* arg1);
QT_GUI_C_EXPORT bool qt_gui_c_QFont_operator_eq(const QFont* this_ptr, const QFont* arg1);
QT_GUI_C_EXPORT bool qt_gui_c_QFont_operator_lt(const QFont* this_ptr, const QFont* arg1);
QT_GUI_C_EXPORT bool qt_gui_c_QFont_operator_neq(const QFont* this_ptr, const QFont* arg1);
QT_GUI_C_EXPORT bool qt_gui_c_QFont_overline(const QFont* this_ptr);
QT_GUI_C_EXPORT int qt_gui_c_QFont_pixelSize(const QFont* this_ptr);
QT_GUI_C_EXPORT int qt_gui_c_QFont_pointSize(const QFont* this_ptr);
QT_GUI_C_EXPORT double qt_gui_c_QFont_pointSizeF(const QFont* this_ptr);
QT_GUI_C_EXPORT bool qt_gui_c_QFont_rawMode(const QFont* this_ptr);
QT_GUI_C_EXPORT void qt_gui_c_QFont_rawName_to_output(const QFont* this_ptr, QString* output);
QT_GUI_C_EXPORT void qt_gui_c_QFont_removeSubstitutions(const QString* arg1);
QT_GUI_C_EXPORT void qt_gui_c_QFont_resolve_mask(QFont* this_ptr, unsigned int mask);
QT_GUI_C_EXPORT unsigned int qt_gui_c_QFont_resolve_no_args(const QFont* this_ptr);
QT_GUI_C_EXPORT void qt_gui_c_QFont_resolve_to_output(const QFont* this_ptr, const QFont* arg1, QFont* output);
QT_GUI_C_EXPORT void qt_gui_c_QFont_setBold(QFont* this_ptr, bool arg1);
QT_GUI_C_EXPORT void qt_gui_c_QFont_setCapitalization(QFont* this_ptr, QFont::Capitalization arg1);
QT_GUI_C_EXPORT void qt_gui_c_QFont_setFamily(QFont* this_ptr, const QString* arg1);
QT_GUI_C_EXPORT void qt_gui_c_QFont_setFixedPitch(QFont* this_ptr, bool arg1);
QT_GUI_C_EXPORT void qt_gui_c_QFont_setHintingPreference(QFont* this_ptr, QFont::HintingPreference hintingPreference);
QT_GUI_C_EXPORT void qt_gui_c_QFont_setItalic(QFont* this_ptr, bool b);
QT_GUI_C_EXPORT void qt_gui_c_QFont_setKerning(QFont* this_ptr, bool arg1);
QT_GUI_C_EXPORT void qt_gui_c_QFont_setLetterSpacing(QFont* this_ptr, QFont::SpacingType type, double spacing);
QT_GUI_C_EXPORT void qt_gui_c_QFont_setOverline(QFont* this_ptr, bool arg1);
QT_GUI_C_EXPORT void qt_gui_c_QFont_setPixelSize(QFont* this_ptr, int arg1);
QT_GUI_C_EXPORT void qt_gui_c_QFont_setPointSize(QFont* this_ptr, int arg1);
QT_GUI_C_EXPORT void qt_gui_c_QFont_setPointSizeF(QFont* this_ptr, double arg1);
QT_GUI_C_EXPORT void qt_gui_c_QFont_setRawMode(QFont* this_ptr, bool arg1);
QT_GUI_C_EXPORT void qt_gui_c_QFont_setRawName(QFont* this_ptr, const QString* arg1);
QT_GUI_C_EXPORT void qt_gui_c_QFont_setStretch(QFont* this_ptr, int arg1);
QT_GUI_C_EXPORT void qt_gui_c_QFont_setStrikeOut(QFont* this_ptr, bool arg1);
QT_GUI_C_EXPORT void qt_gui_c_QFont_setStyle(QFont* this_ptr, QFont::Style style);
QT_GUI_C_EXPORT void qt_gui_c_QFont_setStyleHint_arg1(QFont* this_ptr, QFont::StyleHint arg1);
QT_GUI_C_EXPORT void qt_gui_c_QFont_setStyleHint_arg1_arg2(QFont* this_ptr, QFont::StyleHint arg1, QFont::StyleStrategy arg2);
QT_GUI_C_EXPORT void qt_gui_c_QFont_setStyleName(QFont* this_ptr, const QString* arg1);
QT_GUI_C_EXPORT void qt_gui_c_QFont_setStyleStrategy(QFont* this_ptr, QFont::StyleStrategy s);
QT_GUI_C_EXPORT void qt_gui_c_QFont_setUnderline(QFont* this_ptr, bool arg1);
QT_GUI_C_EXPORT void qt_gui_c_QFont_setWeight(QFont* this_ptr, int arg1);
QT_GUI_C_EXPORT void qt_gui_c_QFont_setWordSpacing(QFont* this_ptr, double spacing);
QT_GUI_C_EXPORT int qt_gui_c_QFont_stretch(const QFont* this_ptr);
QT_GUI_C_EXPORT bool qt_gui_c_QFont_strikeOut(const QFont* this_ptr);
QT_GUI_C_EXPORT QFont::Style qt_gui_c_QFont_style(const QFont* this_ptr);
QT_GUI_C_EXPORT QFont::StyleHint qt_gui_c_QFont_styleHint(const QFont* this_ptr);
QT_GUI_C_EXPORT void qt_gui_c_QFont_styleName_to_output(const QFont* this_ptr, QString* output);
QT_GUI_C_EXPORT QFont::StyleStrategy qt_gui_c_QFont_styleStrategy(const QFont* this_ptr);
QT_GUI_C_EXPORT void qt_gui_c_QFont_substitute_to_output(const QString* arg1, QString* output);
QT_GUI_C_EXPORT void qt_gui_c_QFont_substitutes_to_output(const QString* arg1, QStringList* output);
QT_GUI_C_EXPORT void qt_gui_c_QFont_substitutions_to_output(QStringList* output);
QT_GUI_C_EXPORT void qt_gui_c_QFont_swap(QFont* this_ptr, QFont* other);
QT_GUI_C_EXPORT void qt_gui_c_QFont_toString_to_output(const QFont* this_ptr, QString* output);
QT_GUI_C_EXPORT bool qt_gui_c_QFont_underline(const QFont* this_ptr);
QT_GUI_C_EXPORT int qt_gui_c_QFont_weight(const QFont* this_ptr);
QT_GUI_C_EXPORT double qt_gui_c_QFont_wordSpacing(const QFont* this_ptr);

} // extern "C"

#endif // QT_GUI_C_QFONT_H
