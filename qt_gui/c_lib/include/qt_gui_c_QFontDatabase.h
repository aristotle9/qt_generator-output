#ifndef QT_GUI_C_QFONTDATABASE_H
#define QT_GUI_C_QFONTDATABASE_H

#include "qt_gui_c_global.h"

extern "C" {

QT_GUI_C_EXPORT int qt_gui_c_QFontDatabase_addApplicationFont(const QString* fileName);
QT_GUI_C_EXPORT int qt_gui_c_QFontDatabase_addApplicationFontFromData(const QByteArray* fontData);
QT_GUI_C_EXPORT void qt_gui_c_QFontDatabase_applicationFontFamilies_to_output(int id, QStringList* output);
QT_GUI_C_EXPORT bool qt_gui_c_QFontDatabase_bold(const QFontDatabase* this_ptr, const QString* family, const QString* style);
QT_GUI_C_EXPORT void qt_gui_c_QFontDatabase_delete(QFontDatabase* this_ptr);
QT_GUI_C_EXPORT void qt_gui_c_QFontDatabase_families_to_output_no_args(const QFontDatabase* this_ptr, QStringList* output);
QT_GUI_C_EXPORT void qt_gui_c_QFontDatabase_families_to_output_writingSystem(const QFontDatabase* this_ptr, QFontDatabase::WritingSystem writingSystem, QStringList* output);
QT_GUI_C_EXPORT void qt_gui_c_QFontDatabase_font_to_output(const QFontDatabase* this_ptr, const QString* family, const QString* style, int pointSize, QFont* output);
QT_GUI_C_EXPORT bool qt_gui_c_QFontDatabase_hasFamily(const QFontDatabase* this_ptr, const QString* family);
QT_GUI_C_EXPORT bool qt_gui_c_QFontDatabase_isBitmapScalable_family(const QFontDatabase* this_ptr, const QString* family);
QT_GUI_C_EXPORT bool qt_gui_c_QFontDatabase_isBitmapScalable_family_style(const QFontDatabase* this_ptr, const QString* family, const QString* style);
QT_GUI_C_EXPORT bool qt_gui_c_QFontDatabase_isFixedPitch_family(const QFontDatabase* this_ptr, const QString* family);
QT_GUI_C_EXPORT bool qt_gui_c_QFontDatabase_isFixedPitch_family_style(const QFontDatabase* this_ptr, const QString* family, const QString* style);
QT_GUI_C_EXPORT bool qt_gui_c_QFontDatabase_isPrivateFamily(const QFontDatabase* this_ptr, const QString* family);
QT_GUI_C_EXPORT bool qt_gui_c_QFontDatabase_isScalable_family(const QFontDatabase* this_ptr, const QString* family);
QT_GUI_C_EXPORT bool qt_gui_c_QFontDatabase_isScalable_family_style(const QFontDatabase* this_ptr, const QString* family, const QString* style);
QT_GUI_C_EXPORT bool qt_gui_c_QFontDatabase_isSmoothlyScalable_family(const QFontDatabase* this_ptr, const QString* family);
QT_GUI_C_EXPORT bool qt_gui_c_QFontDatabase_isSmoothlyScalable_family_style(const QFontDatabase* this_ptr, const QString* family, const QString* style);
QT_GUI_C_EXPORT bool qt_gui_c_QFontDatabase_italic(const QFontDatabase* this_ptr, const QString* family, const QString* style);
QT_GUI_C_EXPORT QFontDatabase* qt_gui_c_QFontDatabase_new();
QT_GUI_C_EXPORT void qt_gui_c_QFontDatabase_pointSizes_to_output_family(QFontDatabase* this_ptr, const QString* family, QList< int >* output);
QT_GUI_C_EXPORT void qt_gui_c_QFontDatabase_pointSizes_to_output_family_style(QFontDatabase* this_ptr, const QString* family, const QString* style, QList< int >* output);
QT_GUI_C_EXPORT bool qt_gui_c_QFontDatabase_removeAllApplicationFonts();
QT_GUI_C_EXPORT bool qt_gui_c_QFontDatabase_removeApplicationFont(int id);
QT_GUI_C_EXPORT void qt_gui_c_QFontDatabase_smoothSizes_to_output(QFontDatabase* this_ptr, const QString* family, const QString* style, QList< int >* output);
QT_GUI_C_EXPORT void qt_gui_c_QFontDatabase_standardSizes_to_output(QList< int >* output);
QT_GUI_C_EXPORT void qt_gui_c_QFontDatabase_styleString_to_output_font(QFontDatabase* this_ptr, const QFont* font, QString* output);
QT_GUI_C_EXPORT void qt_gui_c_QFontDatabase_styleString_to_output_fontInfo(QFontDatabase* this_ptr, const QFontInfo* fontInfo, QString* output);
QT_GUI_C_EXPORT void qt_gui_c_QFontDatabase_styles_to_output(const QFontDatabase* this_ptr, const QString* family, QStringList* output);
QT_GUI_C_EXPORT bool qt_gui_c_QFontDatabase_supportsThreadedFontRendering();
QT_GUI_C_EXPORT void qt_gui_c_QFontDatabase_systemFont_to_output(QFontDatabase::SystemFont type, QFont* output);
QT_GUI_C_EXPORT int qt_gui_c_QFontDatabase_weight(const QFontDatabase* this_ptr, const QString* family, const QString* style);
QT_GUI_C_EXPORT void qt_gui_c_QFontDatabase_writingSystemName_to_output(QFontDatabase::WritingSystem writingSystem, QString* output);
QT_GUI_C_EXPORT void qt_gui_c_QFontDatabase_writingSystemSample_to_output(QFontDatabase::WritingSystem writingSystem, QString* output);
QT_GUI_C_EXPORT void qt_gui_c_QFontDatabase_writingSystems_to_output_family(const QFontDatabase* this_ptr, const QString* family, QList< QFontDatabase::WritingSystem >* output);
QT_GUI_C_EXPORT void qt_gui_c_QFontDatabase_writingSystems_to_output_no_args(const QFontDatabase* this_ptr, QList< QFontDatabase::WritingSystem >* output);

} // extern "C"

#endif // QT_GUI_C_QFONTDATABASE_H
