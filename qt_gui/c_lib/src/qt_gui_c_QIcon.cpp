#include "qt_gui_c_QIcon.h"

void qt_gui_c_QIcon_G_operator_shl_to_output(const QDebug* dbg, const QIcon* arg2, QDebug* output) {
  new(output) QDebug(operator<<(*dbg, *arg2));
}

void qt_gui_c_QIcon_G_qt_findAtNxFile_to_output_baseFileName_targetDevicePixelRatio(const QString* baseFileName, double targetDevicePixelRatio, QString* output) {
  new(output) QString(qt_findAtNxFile(*baseFileName, targetDevicePixelRatio));
}

void qt_gui_c_QIcon_G_qt_findAtNxFile_to_output_baseFileName_targetDevicePixelRatio_sourceDevicePixelRatio(const QString* baseFileName, double targetDevicePixelRatio, double* sourceDevicePixelRatio, QString* output) {
  new(output) QString(qt_findAtNxFile(*baseFileName, targetDevicePixelRatio, sourceDevicePixelRatio));
}

void qt_gui_c_QIcon_G_swap(QIcon* value1, QIcon* value2) {
  swap(*value1, *value2);
}

void qt_gui_c_QIcon_actualSize_to_output_size(const QIcon* this_ptr, const QSize* size, QSize* output) {
  new(output) QSize(this_ptr->actualSize(*size));
}

void qt_gui_c_QIcon_actualSize_to_output_size_mode(const QIcon* this_ptr, const QSize* size, QIcon::Mode mode, QSize* output) {
  new(output) QSize(this_ptr->actualSize(*size, mode));
}

void qt_gui_c_QIcon_actualSize_to_output_size_mode_state(const QIcon* this_ptr, const QSize* size, QIcon::Mode mode, QIcon::State state, QSize* output) {
  new(output) QSize(this_ptr->actualSize(*size, mode, state));
}

void qt_gui_c_QIcon_actualSize_to_output_window_size(const QIcon* this_ptr, QWindow* window, const QSize* size, QSize* output) {
  new(output) QSize(this_ptr->actualSize(window, *size));
}

void qt_gui_c_QIcon_actualSize_to_output_window_size_mode(const QIcon* this_ptr, QWindow* window, const QSize* size, QIcon::Mode mode, QSize* output) {
  new(output) QSize(this_ptr->actualSize(window, *size, mode));
}

void qt_gui_c_QIcon_actualSize_to_output_window_size_mode_state(const QIcon* this_ptr, QWindow* window, const QSize* size, QIcon::Mode mode, QIcon::State state, QSize* output) {
  new(output) QSize(this_ptr->actualSize(window, *size, mode, state));
}

void qt_gui_c_QIcon_addFile_fileName(QIcon* this_ptr, const QString* fileName) {
  this_ptr->addFile(*fileName);
}

void qt_gui_c_QIcon_addFile_fileName_size(QIcon* this_ptr, const QString* fileName, const QSize* size) {
  this_ptr->addFile(*fileName, *size);
}

void qt_gui_c_QIcon_addFile_fileName_size_mode(QIcon* this_ptr, const QString* fileName, const QSize* size, QIcon::Mode mode) {
  this_ptr->addFile(*fileName, *size, mode);
}

void qt_gui_c_QIcon_addFile_fileName_size_mode_state(QIcon* this_ptr, const QString* fileName, const QSize* size, QIcon::Mode mode, QIcon::State state) {
  this_ptr->addFile(*fileName, *size, mode, state);
}

void qt_gui_c_QIcon_addPixmap_pixmap(QIcon* this_ptr, const QPixmap* pixmap) {
  this_ptr->addPixmap(*pixmap);
}

void qt_gui_c_QIcon_addPixmap_pixmap_mode(QIcon* this_ptr, const QPixmap* pixmap, QIcon::Mode mode) {
  this_ptr->addPixmap(*pixmap, mode);
}

void qt_gui_c_QIcon_addPixmap_pixmap_mode_state(QIcon* this_ptr, const QPixmap* pixmap, QIcon::Mode mode, QIcon::State state) {
  this_ptr->addPixmap(*pixmap, mode, state);
}

void qt_gui_c_QIcon_availableSizes_to_output_mode(const QIcon* this_ptr, QIcon::Mode mode, QList< QSize >* output) {
  new(output) QList< QSize >(this_ptr->availableSizes(mode));
}

void qt_gui_c_QIcon_availableSizes_to_output_mode_state(const QIcon* this_ptr, QIcon::Mode mode, QIcon::State state, QList< QSize >* output) {
  new(output) QList< QSize >(this_ptr->availableSizes(mode, state));
}

void qt_gui_c_QIcon_availableSizes_to_output_no_args(const QIcon* this_ptr, QList< QSize >* output) {
  new(output) QList< QSize >(this_ptr->availableSizes());
}

qint64 qt_gui_c_QIcon_cacheKey(const QIcon* this_ptr) {
  return this_ptr->cacheKey();
}

void qt_gui_c_QIcon_constructor_engine(QIconEngine* engine, QIcon* output) {
  new(output) QIcon(engine);
}

void qt_gui_c_QIcon_constructor_fileName(const QString* fileName, QIcon* output) {
  new(output) QIcon(*fileName);
}

void qt_gui_c_QIcon_constructor_no_args(QIcon* output) {
  new(output) QIcon();
}

void qt_gui_c_QIcon_constructor_other(const QIcon* other, QIcon* output) {
  new(output) QIcon(*other);
}

void qt_gui_c_QIcon_constructor_pixmap(const QPixmap* pixmap, QIcon* output) {
  new(output) QIcon(*pixmap);
}

void qt_gui_c_QIcon_convert_to_QVariant_to_output(const QIcon* this_ptr, QVariant* output) {
  new(output) QVariant(this_ptr->operator QVariant());
}

void qt_gui_c_QIcon_destructor(QIcon* this_ptr) {
  qt_gui_c_call_destructor(this_ptr);
}

void qt_gui_c_QIcon_detach(QIcon* this_ptr) {
  this_ptr->detach();
}

void qt_gui_c_QIcon_fromTheme_to_output_name(const QString* name, QIcon* output) {
  new(output) QIcon(QIcon::fromTheme(*name));
}

void qt_gui_c_QIcon_fromTheme_to_output_name_fallback(const QString* name, const QIcon* fallback, QIcon* output) {
  new(output) QIcon(QIcon::fromTheme(*name, *fallback));
}

bool qt_gui_c_QIcon_hasThemeIcon(const QString* name) {
  return QIcon::hasThemeIcon(*name);
}

bool qt_gui_c_QIcon_isDetached(const QIcon* this_ptr) {
  return this_ptr->isDetached();
}

bool qt_gui_c_QIcon_isMask(const QIcon* this_ptr) {
  return this_ptr->isMask();
}

bool qt_gui_c_QIcon_isNull(const QIcon* this_ptr) {
  return this_ptr->isNull();
}

void qt_gui_c_QIcon_name_to_output(const QIcon* this_ptr, QString* output) {
  new(output) QString(this_ptr->name());
}

QIcon* qt_gui_c_QIcon_operator_assign(QIcon* this_ptr, const QIcon* other) {
  return &this_ptr->operator=(*other);
}

QPixmap* qt_gui_c_QIcon_pixmap_as_ptr_extent(const QIcon* this_ptr, int extent) {
  return new QPixmap(this_ptr->pixmap(extent));
}

QPixmap* qt_gui_c_QIcon_pixmap_as_ptr_extent_mode(const QIcon* this_ptr, int extent, QIcon::Mode mode) {
  return new QPixmap(this_ptr->pixmap(extent, mode));
}

QPixmap* qt_gui_c_QIcon_pixmap_as_ptr_extent_mode_state(const QIcon* this_ptr, int extent, QIcon::Mode mode, QIcon::State state) {
  return new QPixmap(this_ptr->pixmap(extent, mode, state));
}

QPixmap* qt_gui_c_QIcon_pixmap_as_ptr_size(const QIcon* this_ptr, const QSize* size) {
  return new QPixmap(this_ptr->pixmap(*size));
}

QPixmap* qt_gui_c_QIcon_pixmap_as_ptr_size_mode(const QIcon* this_ptr, const QSize* size, QIcon::Mode mode) {
  return new QPixmap(this_ptr->pixmap(*size, mode));
}

QPixmap* qt_gui_c_QIcon_pixmap_as_ptr_size_mode_state(const QIcon* this_ptr, const QSize* size, QIcon::Mode mode, QIcon::State state) {
  return new QPixmap(this_ptr->pixmap(*size, mode, state));
}

QPixmap* qt_gui_c_QIcon_pixmap_as_ptr_w_h(const QIcon* this_ptr, int w, int h) {
  return new QPixmap(this_ptr->pixmap(w, h));
}

QPixmap* qt_gui_c_QIcon_pixmap_as_ptr_w_h_mode(const QIcon* this_ptr, int w, int h, QIcon::Mode mode) {
  return new QPixmap(this_ptr->pixmap(w, h, mode));
}

QPixmap* qt_gui_c_QIcon_pixmap_as_ptr_w_h_mode_state(const QIcon* this_ptr, int w, int h, QIcon::Mode mode, QIcon::State state) {
  return new QPixmap(this_ptr->pixmap(w, h, mode, state));
}

QPixmap* qt_gui_c_QIcon_pixmap_as_ptr_window_size(const QIcon* this_ptr, QWindow* window, const QSize* size) {
  return new QPixmap(this_ptr->pixmap(window, *size));
}

QPixmap* qt_gui_c_QIcon_pixmap_as_ptr_window_size_mode(const QIcon* this_ptr, QWindow* window, const QSize* size, QIcon::Mode mode) {
  return new QPixmap(this_ptr->pixmap(window, *size, mode));
}

QPixmap* qt_gui_c_QIcon_pixmap_as_ptr_window_size_mode_state(const QIcon* this_ptr, QWindow* window, const QSize* size, QIcon::Mode mode, QIcon::State state) {
  return new QPixmap(this_ptr->pixmap(window, *size, mode, state));
}

void qt_gui_c_QIcon_setIsMask(QIcon* this_ptr, bool isMask) {
  this_ptr->setIsMask(isMask);
}

void qt_gui_c_QIcon_setThemeName(const QString* path) {
  QIcon::setThemeName(*path);
}

void qt_gui_c_QIcon_setThemeSearchPaths(const QStringList* searchpath) {
  QIcon::setThemeSearchPaths(*searchpath);
}

void qt_gui_c_QIcon_swap(QIcon* this_ptr, QIcon* other) {
  this_ptr->swap(*other);
}

void qt_gui_c_QIcon_themeName_to_output(QString* output) {
  new(output) QString(QIcon::themeName());
}

void qt_gui_c_QIcon_themeSearchPaths_to_output(QStringList* output) {
  new(output) QStringList(QIcon::themeSearchPaths());
}

