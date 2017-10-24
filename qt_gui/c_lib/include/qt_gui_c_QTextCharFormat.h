#ifndef QT_GUI_C_QTEXTCHARFORMAT_H
#define QT_GUI_C_QTEXTCHARFORMAT_H

#include "qt_gui_c_global.h"

extern "C" {

QT_GUI_C_EXPORT QTextCharFormat* qt_gui_c_QTextCharFormat_G_static_cast_QTextCharFormat_ptr(QTextFormat* ptr);
QT_GUI_C_EXPORT QTextFormat* qt_gui_c_QTextCharFormat_G_static_cast_QTextFormat_ptr(QTextCharFormat* ptr);
QT_GUI_C_EXPORT void qt_gui_c_QTextCharFormat_anchorHref_to_output(const QTextCharFormat* this_ptr, QString* output);
QT_GUI_C_EXPORT void qt_gui_c_QTextCharFormat_anchorName_to_output(const QTextCharFormat* this_ptr, QString* output);
QT_GUI_C_EXPORT void qt_gui_c_QTextCharFormat_anchorNames_to_output(const QTextCharFormat* this_ptr, QStringList* output);
QT_GUI_C_EXPORT void qt_gui_c_QTextCharFormat_constructor(QTextCharFormat* output);
QT_GUI_C_EXPORT void qt_gui_c_QTextCharFormat_destructor(QTextCharFormat* this_ptr);
QT_GUI_C_EXPORT void qt_gui_c_QTextCharFormat_fontFamily_to_output(const QTextCharFormat* this_ptr, QString* output);
QT_GUI_C_EXPORT bool qt_gui_c_QTextCharFormat_fontFixedPitch(const QTextCharFormat* this_ptr);
QT_GUI_C_EXPORT bool qt_gui_c_QTextCharFormat_fontItalic(const QTextCharFormat* this_ptr);
QT_GUI_C_EXPORT bool qt_gui_c_QTextCharFormat_fontKerning(const QTextCharFormat* this_ptr);
QT_GUI_C_EXPORT double qt_gui_c_QTextCharFormat_fontLetterSpacing(const QTextCharFormat* this_ptr);
QT_GUI_C_EXPORT bool qt_gui_c_QTextCharFormat_fontOverline(const QTextCharFormat* this_ptr);
QT_GUI_C_EXPORT double qt_gui_c_QTextCharFormat_fontPointSize(const QTextCharFormat* this_ptr);
QT_GUI_C_EXPORT int qt_gui_c_QTextCharFormat_fontStretch(const QTextCharFormat* this_ptr);
QT_GUI_C_EXPORT bool qt_gui_c_QTextCharFormat_fontStrikeOut(const QTextCharFormat* this_ptr);
QT_GUI_C_EXPORT bool qt_gui_c_QTextCharFormat_fontUnderline(const QTextCharFormat* this_ptr);
QT_GUI_C_EXPORT int qt_gui_c_QTextCharFormat_fontWeight(const QTextCharFormat* this_ptr);
QT_GUI_C_EXPORT double qt_gui_c_QTextCharFormat_fontWordSpacing(const QTextCharFormat* this_ptr);
QT_GUI_C_EXPORT void qt_gui_c_QTextCharFormat_font_to_output(const QTextCharFormat* this_ptr, QFont* output);
QT_GUI_C_EXPORT bool qt_gui_c_QTextCharFormat_isAnchor(const QTextCharFormat* this_ptr);
QT_GUI_C_EXPORT bool qt_gui_c_QTextCharFormat_isValid(const QTextCharFormat* this_ptr);
QT_GUI_C_EXPORT void qt_gui_c_QTextCharFormat_setAnchor(QTextCharFormat* this_ptr, bool anchor);
QT_GUI_C_EXPORT void qt_gui_c_QTextCharFormat_setAnchorHref(QTextCharFormat* this_ptr, const QString* value);
QT_GUI_C_EXPORT void qt_gui_c_QTextCharFormat_setAnchorName(QTextCharFormat* this_ptr, const QString* name);
QT_GUI_C_EXPORT void qt_gui_c_QTextCharFormat_setAnchorNames(QTextCharFormat* this_ptr, const QStringList* names);
QT_GUI_C_EXPORT void qt_gui_c_QTextCharFormat_setFontCapitalization(QTextCharFormat* this_ptr, const QFont::Capitalization* capitalization);
QT_GUI_C_EXPORT void qt_gui_c_QTextCharFormat_setFontFamily(QTextCharFormat* this_ptr, const QString* family);
QT_GUI_C_EXPORT void qt_gui_c_QTextCharFormat_setFontFixedPitch(QTextCharFormat* this_ptr, bool fixedPitch);
QT_GUI_C_EXPORT void qt_gui_c_QTextCharFormat_setFontHintingPreference(QTextCharFormat* this_ptr, const QFont::HintingPreference* hintingPreference);
QT_GUI_C_EXPORT void qt_gui_c_QTextCharFormat_setFontItalic(QTextCharFormat* this_ptr, bool italic);
QT_GUI_C_EXPORT void qt_gui_c_QTextCharFormat_setFontKerning(QTextCharFormat* this_ptr, bool enable);
QT_GUI_C_EXPORT void qt_gui_c_QTextCharFormat_setFontLetterSpacing(QTextCharFormat* this_ptr, double spacing);
QT_GUI_C_EXPORT void qt_gui_c_QTextCharFormat_setFontLetterSpacingType(QTextCharFormat* this_ptr, const QFont::SpacingType* letterSpacingType);
QT_GUI_C_EXPORT void qt_gui_c_QTextCharFormat_setFontOverline(QTextCharFormat* this_ptr, bool overline);
QT_GUI_C_EXPORT void qt_gui_c_QTextCharFormat_setFontPointSize(QTextCharFormat* this_ptr, double size);
QT_GUI_C_EXPORT void qt_gui_c_QTextCharFormat_setFontStretch(QTextCharFormat* this_ptr, int factor);
QT_GUI_C_EXPORT void qt_gui_c_QTextCharFormat_setFontStrikeOut(QTextCharFormat* this_ptr, bool strikeOut);
QT_GUI_C_EXPORT void qt_gui_c_QTextCharFormat_setFontStyleHint_hint(QTextCharFormat* this_ptr, const QFont::StyleHint* hint);
QT_GUI_C_EXPORT void qt_gui_c_QTextCharFormat_setFontStyleHint_hint_strategy(QTextCharFormat* this_ptr, const QFont::StyleHint* hint, const QFont::StyleStrategy* strategy);
QT_GUI_C_EXPORT void qt_gui_c_QTextCharFormat_setFontStyleStrategy(QTextCharFormat* this_ptr, const QFont::StyleStrategy* strategy);
QT_GUI_C_EXPORT void qt_gui_c_QTextCharFormat_setFontUnderline(QTextCharFormat* this_ptr, bool underline);
QT_GUI_C_EXPORT void qt_gui_c_QTextCharFormat_setFontWeight(QTextCharFormat* this_ptr, int weight);
QT_GUI_C_EXPORT void qt_gui_c_QTextCharFormat_setFontWordSpacing(QTextCharFormat* this_ptr, double spacing);
QT_GUI_C_EXPORT void qt_gui_c_QTextCharFormat_setFont_font(QTextCharFormat* this_ptr, const QFont* font);
QT_GUI_C_EXPORT void qt_gui_c_QTextCharFormat_setFont_font_behavior(QTextCharFormat* this_ptr, const QFont* font, QTextCharFormat::FontPropertiesInheritanceBehavior behavior);
QT_GUI_C_EXPORT void qt_gui_c_QTextCharFormat_setTableCellColumnSpan(QTextCharFormat* this_ptr, int tableCellColumnSpan);
QT_GUI_C_EXPORT void qt_gui_c_QTextCharFormat_setTableCellRowSpan(QTextCharFormat* this_ptr, int tableCellRowSpan);
QT_GUI_C_EXPORT void qt_gui_c_QTextCharFormat_setTextOutline(QTextCharFormat* this_ptr, const QPen* pen);
QT_GUI_C_EXPORT void qt_gui_c_QTextCharFormat_setToolTip(QTextCharFormat* this_ptr, const QString* tip);
QT_GUI_C_EXPORT void qt_gui_c_QTextCharFormat_setUnderlineColor(QTextCharFormat* this_ptr, const QColor* color);
QT_GUI_C_EXPORT void qt_gui_c_QTextCharFormat_setUnderlineStyle(QTextCharFormat* this_ptr, QTextCharFormat::UnderlineStyle style);
QT_GUI_C_EXPORT void qt_gui_c_QTextCharFormat_setVerticalAlignment(QTextCharFormat* this_ptr, QTextCharFormat::VerticalAlignment alignment);
QT_GUI_C_EXPORT int qt_gui_c_QTextCharFormat_tableCellColumnSpan(const QTextCharFormat* this_ptr);
QT_GUI_C_EXPORT int qt_gui_c_QTextCharFormat_tableCellRowSpan(const QTextCharFormat* this_ptr);
QT_GUI_C_EXPORT void qt_gui_c_QTextCharFormat_textOutline_to_output(const QTextCharFormat* this_ptr, QPen* output);
QT_GUI_C_EXPORT void qt_gui_c_QTextCharFormat_toolTip_to_output(const QTextCharFormat* this_ptr, QString* output);
QT_GUI_C_EXPORT void qt_gui_c_QTextCharFormat_underlineColor_to_output(const QTextCharFormat* this_ptr, QColor* output);
QT_GUI_C_EXPORT QTextCharFormat::UnderlineStyle qt_gui_c_QTextCharFormat_underlineStyle(const QTextCharFormat* this_ptr);
QT_GUI_C_EXPORT QTextCharFormat::VerticalAlignment qt_gui_c_QTextCharFormat_verticalAlignment(const QTextCharFormat* this_ptr);

} // extern "C"

#endif // QT_GUI_C_QTEXTCHARFORMAT_H
