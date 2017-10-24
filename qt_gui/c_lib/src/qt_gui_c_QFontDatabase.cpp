#include "qt_gui_c_QFontDatabase.h"

int qt_gui_c_QFontDatabase_addApplicationFont(const QString* fileName) {
  return QFontDatabase::addApplicationFont(*fileName);
}

int qt_gui_c_QFontDatabase_addApplicationFontFromData(const QByteArray* fontData) {
  return QFontDatabase::addApplicationFontFromData(*fontData);
}

void qt_gui_c_QFontDatabase_applicationFontFamilies_to_output(int id, QStringList* output) {
  new(output) QStringList(QFontDatabase::applicationFontFamilies(id));
}

bool qt_gui_c_QFontDatabase_bold(const QFontDatabase* this_ptr, const QString* family, const QString* style) {
  return this_ptr->bold(*family, *style);
}

void qt_gui_c_QFontDatabase_delete(QFontDatabase* this_ptr) {
  delete this_ptr;
}

void qt_gui_c_QFontDatabase_families_to_output_no_args(const QFontDatabase* this_ptr, QStringList* output) {
  new(output) QStringList(this_ptr->families());
}

void qt_gui_c_QFontDatabase_families_to_output_writingSystem(const QFontDatabase* this_ptr, QFontDatabase::WritingSystem writingSystem, QStringList* output) {
  new(output) QStringList(this_ptr->families(writingSystem));
}

void qt_gui_c_QFontDatabase_font_to_output(const QFontDatabase* this_ptr, const QString* family, const QString* style, int pointSize, QFont* output) {
  new(output) QFont(this_ptr->font(*family, *style, pointSize));
}

bool qt_gui_c_QFontDatabase_hasFamily(const QFontDatabase* this_ptr, const QString* family) {
  return this_ptr->hasFamily(*family);
}

bool qt_gui_c_QFontDatabase_isBitmapScalable_family(const QFontDatabase* this_ptr, const QString* family) {
  return this_ptr->isBitmapScalable(*family);
}

bool qt_gui_c_QFontDatabase_isBitmapScalable_family_style(const QFontDatabase* this_ptr, const QString* family, const QString* style) {
  return this_ptr->isBitmapScalable(*family, *style);
}

bool qt_gui_c_QFontDatabase_isFixedPitch_family(const QFontDatabase* this_ptr, const QString* family) {
  return this_ptr->isFixedPitch(*family);
}

bool qt_gui_c_QFontDatabase_isFixedPitch_family_style(const QFontDatabase* this_ptr, const QString* family, const QString* style) {
  return this_ptr->isFixedPitch(*family, *style);
}

bool qt_gui_c_QFontDatabase_isPrivateFamily(const QFontDatabase* this_ptr, const QString* family) {
  return this_ptr->isPrivateFamily(*family);
}

bool qt_gui_c_QFontDatabase_isScalable_family(const QFontDatabase* this_ptr, const QString* family) {
  return this_ptr->isScalable(*family);
}

bool qt_gui_c_QFontDatabase_isScalable_family_style(const QFontDatabase* this_ptr, const QString* family, const QString* style) {
  return this_ptr->isScalable(*family, *style);
}

bool qt_gui_c_QFontDatabase_isSmoothlyScalable_family(const QFontDatabase* this_ptr, const QString* family) {
  return this_ptr->isSmoothlyScalable(*family);
}

bool qt_gui_c_QFontDatabase_isSmoothlyScalable_family_style(const QFontDatabase* this_ptr, const QString* family, const QString* style) {
  return this_ptr->isSmoothlyScalable(*family, *style);
}

bool qt_gui_c_QFontDatabase_italic(const QFontDatabase* this_ptr, const QString* family, const QString* style) {
  return this_ptr->italic(*family, *style);
}

QFontDatabase* qt_gui_c_QFontDatabase_new() {
  return new QFontDatabase();
}

void qt_gui_c_QFontDatabase_pointSizes_to_output_family(QFontDatabase* this_ptr, const QString* family, QList< int >* output) {
  new(output) QList< int >(this_ptr->pointSizes(*family));
}

void qt_gui_c_QFontDatabase_pointSizes_to_output_family_style(QFontDatabase* this_ptr, const QString* family, const QString* style, QList< int >* output) {
  new(output) QList< int >(this_ptr->pointSizes(*family, *style));
}

bool qt_gui_c_QFontDatabase_removeAllApplicationFonts() {
  return QFontDatabase::removeAllApplicationFonts();
}

bool qt_gui_c_QFontDatabase_removeApplicationFont(int id) {
  return QFontDatabase::removeApplicationFont(id);
}

void qt_gui_c_QFontDatabase_smoothSizes_to_output(QFontDatabase* this_ptr, const QString* family, const QString* style, QList< int >* output) {
  new(output) QList< int >(this_ptr->smoothSizes(*family, *style));
}

void qt_gui_c_QFontDatabase_standardSizes_to_output(QList< int >* output) {
  new(output) QList< int >(QFontDatabase::standardSizes());
}

void qt_gui_c_QFontDatabase_styleString_to_output_font(QFontDatabase* this_ptr, const QFont* font, QString* output) {
  new(output) QString(this_ptr->styleString(*font));
}

void qt_gui_c_QFontDatabase_styleString_to_output_fontInfo(QFontDatabase* this_ptr, const QFontInfo* fontInfo, QString* output) {
  new(output) QString(this_ptr->styleString(*fontInfo));
}

void qt_gui_c_QFontDatabase_styles_to_output(const QFontDatabase* this_ptr, const QString* family, QStringList* output) {
  new(output) QStringList(this_ptr->styles(*family));
}

bool qt_gui_c_QFontDatabase_supportsThreadedFontRendering() {
  return QFontDatabase::supportsThreadedFontRendering();
}

void qt_gui_c_QFontDatabase_systemFont_to_output(QFontDatabase::SystemFont type, QFont* output) {
  new(output) QFont(QFontDatabase::systemFont(type));
}

int qt_gui_c_QFontDatabase_weight(const QFontDatabase* this_ptr, const QString* family, const QString* style) {
  return this_ptr->weight(*family, *style);
}

void qt_gui_c_QFontDatabase_writingSystemName_to_output(QFontDatabase::WritingSystem writingSystem, QString* output) {
  new(output) QString(QFontDatabase::writingSystemName(writingSystem));
}

void qt_gui_c_QFontDatabase_writingSystemSample_to_output(QFontDatabase::WritingSystem writingSystem, QString* output) {
  new(output) QString(QFontDatabase::writingSystemSample(writingSystem));
}

void qt_gui_c_QFontDatabase_writingSystems_to_output_family(const QFontDatabase* this_ptr, const QString* family, QList< QFontDatabase::WritingSystem >* output) {
  new(output) QList< QFontDatabase::WritingSystem >(this_ptr->writingSystems(*family));
}

void qt_gui_c_QFontDatabase_writingSystems_to_output_no_args(const QFontDatabase* this_ptr, QList< QFontDatabase::WritingSystem >* output) {
  new(output) QList< QFontDatabase::WritingSystem >(this_ptr->writingSystems());
}

