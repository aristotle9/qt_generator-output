#ifndef QT_GUI_C_QICON_H
#define QT_GUI_C_QICON_H

#include "qt_gui_c_global.h"

extern "C" {

QT_GUI_C_EXPORT void qt_gui_c_QIcon_G_operator_shl_to_output(const QDebug* dbg, const QIcon* arg2, QDebug* output);
QT_GUI_C_EXPORT void qt_gui_c_QIcon_G_qt_findAtNxFile_to_output_baseFileName_targetDevicePixelRatio(const QString* baseFileName, double targetDevicePixelRatio, QString* output);
QT_GUI_C_EXPORT void qt_gui_c_QIcon_G_qt_findAtNxFile_to_output_baseFileName_targetDevicePixelRatio_sourceDevicePixelRatio(const QString* baseFileName, double targetDevicePixelRatio, double* sourceDevicePixelRatio, QString* output);
QT_GUI_C_EXPORT void qt_gui_c_QIcon_G_swap(QIcon* value1, QIcon* value2);
QT_GUI_C_EXPORT void qt_gui_c_QIcon_actualSize_to_output_size(const QIcon* this_ptr, const QSize* size, QSize* output);
QT_GUI_C_EXPORT void qt_gui_c_QIcon_actualSize_to_output_size_mode(const QIcon* this_ptr, const QSize* size, QIcon::Mode mode, QSize* output);
QT_GUI_C_EXPORT void qt_gui_c_QIcon_actualSize_to_output_size_mode_state(const QIcon* this_ptr, const QSize* size, QIcon::Mode mode, QIcon::State state, QSize* output);
QT_GUI_C_EXPORT void qt_gui_c_QIcon_actualSize_to_output_window_size(const QIcon* this_ptr, QWindow* window, const QSize* size, QSize* output);
QT_GUI_C_EXPORT void qt_gui_c_QIcon_actualSize_to_output_window_size_mode(const QIcon* this_ptr, QWindow* window, const QSize* size, QIcon::Mode mode, QSize* output);
QT_GUI_C_EXPORT void qt_gui_c_QIcon_actualSize_to_output_window_size_mode_state(const QIcon* this_ptr, QWindow* window, const QSize* size, QIcon::Mode mode, QIcon::State state, QSize* output);
QT_GUI_C_EXPORT void qt_gui_c_QIcon_addFile_fileName(QIcon* this_ptr, const QString* fileName);
QT_GUI_C_EXPORT void qt_gui_c_QIcon_addFile_fileName_size(QIcon* this_ptr, const QString* fileName, const QSize* size);
QT_GUI_C_EXPORT void qt_gui_c_QIcon_addFile_fileName_size_mode(QIcon* this_ptr, const QString* fileName, const QSize* size, QIcon::Mode mode);
QT_GUI_C_EXPORT void qt_gui_c_QIcon_addFile_fileName_size_mode_state(QIcon* this_ptr, const QString* fileName, const QSize* size, QIcon::Mode mode, QIcon::State state);
QT_GUI_C_EXPORT void qt_gui_c_QIcon_addPixmap_pixmap(QIcon* this_ptr, const QPixmap* pixmap);
QT_GUI_C_EXPORT void qt_gui_c_QIcon_addPixmap_pixmap_mode(QIcon* this_ptr, const QPixmap* pixmap, QIcon::Mode mode);
QT_GUI_C_EXPORT void qt_gui_c_QIcon_addPixmap_pixmap_mode_state(QIcon* this_ptr, const QPixmap* pixmap, QIcon::Mode mode, QIcon::State state);
QT_GUI_C_EXPORT void qt_gui_c_QIcon_availableSizes_to_output_mode(const QIcon* this_ptr, QIcon::Mode mode, QList< QSize >* output);
QT_GUI_C_EXPORT void qt_gui_c_QIcon_availableSizes_to_output_mode_state(const QIcon* this_ptr, QIcon::Mode mode, QIcon::State state, QList< QSize >* output);
QT_GUI_C_EXPORT void qt_gui_c_QIcon_availableSizes_to_output_no_args(const QIcon* this_ptr, QList< QSize >* output);
QT_GUI_C_EXPORT qint64 qt_gui_c_QIcon_cacheKey(const QIcon* this_ptr);
QT_GUI_C_EXPORT void qt_gui_c_QIcon_constructor_engine(QIconEngine* engine, QIcon* output);
QT_GUI_C_EXPORT void qt_gui_c_QIcon_constructor_fileName(const QString* fileName, QIcon* output);
QT_GUI_C_EXPORT void qt_gui_c_QIcon_constructor_no_args(QIcon* output);
QT_GUI_C_EXPORT void qt_gui_c_QIcon_constructor_other(const QIcon* other, QIcon* output);
QT_GUI_C_EXPORT void qt_gui_c_QIcon_constructor_pixmap(const QPixmap* pixmap, QIcon* output);
QT_GUI_C_EXPORT void qt_gui_c_QIcon_convert_to_QVariant_to_output(const QIcon* this_ptr, QVariant* output);
QT_GUI_C_EXPORT void qt_gui_c_QIcon_destructor(QIcon* this_ptr);
QT_GUI_C_EXPORT void qt_gui_c_QIcon_detach(QIcon* this_ptr);
QT_GUI_C_EXPORT void qt_gui_c_QIcon_fromTheme_to_output_name(const QString* name, QIcon* output);
QT_GUI_C_EXPORT void qt_gui_c_QIcon_fromTheme_to_output_name_fallback(const QString* name, const QIcon* fallback, QIcon* output);
QT_GUI_C_EXPORT bool qt_gui_c_QIcon_hasThemeIcon(const QString* name);
QT_GUI_C_EXPORT bool qt_gui_c_QIcon_isDetached(const QIcon* this_ptr);
QT_GUI_C_EXPORT bool qt_gui_c_QIcon_isMask(const QIcon* this_ptr);
QT_GUI_C_EXPORT bool qt_gui_c_QIcon_isNull(const QIcon* this_ptr);
QT_GUI_C_EXPORT void qt_gui_c_QIcon_name_to_output(const QIcon* this_ptr, QString* output);
QT_GUI_C_EXPORT QIcon* qt_gui_c_QIcon_operator_assign(QIcon* this_ptr, const QIcon* other);
QT_GUI_C_EXPORT QPixmap* qt_gui_c_QIcon_pixmap_as_ptr_extent(const QIcon* this_ptr, int extent);
QT_GUI_C_EXPORT QPixmap* qt_gui_c_QIcon_pixmap_as_ptr_extent_mode(const QIcon* this_ptr, int extent, QIcon::Mode mode);
QT_GUI_C_EXPORT QPixmap* qt_gui_c_QIcon_pixmap_as_ptr_extent_mode_state(const QIcon* this_ptr, int extent, QIcon::Mode mode, QIcon::State state);
QT_GUI_C_EXPORT QPixmap* qt_gui_c_QIcon_pixmap_as_ptr_size(const QIcon* this_ptr, const QSize* size);
QT_GUI_C_EXPORT QPixmap* qt_gui_c_QIcon_pixmap_as_ptr_size_mode(const QIcon* this_ptr, const QSize* size, QIcon::Mode mode);
QT_GUI_C_EXPORT QPixmap* qt_gui_c_QIcon_pixmap_as_ptr_size_mode_state(const QIcon* this_ptr, const QSize* size, QIcon::Mode mode, QIcon::State state);
QT_GUI_C_EXPORT QPixmap* qt_gui_c_QIcon_pixmap_as_ptr_w_h(const QIcon* this_ptr, int w, int h);
QT_GUI_C_EXPORT QPixmap* qt_gui_c_QIcon_pixmap_as_ptr_w_h_mode(const QIcon* this_ptr, int w, int h, QIcon::Mode mode);
QT_GUI_C_EXPORT QPixmap* qt_gui_c_QIcon_pixmap_as_ptr_w_h_mode_state(const QIcon* this_ptr, int w, int h, QIcon::Mode mode, QIcon::State state);
QT_GUI_C_EXPORT QPixmap* qt_gui_c_QIcon_pixmap_as_ptr_window_size(const QIcon* this_ptr, QWindow* window, const QSize* size);
QT_GUI_C_EXPORT QPixmap* qt_gui_c_QIcon_pixmap_as_ptr_window_size_mode(const QIcon* this_ptr, QWindow* window, const QSize* size, QIcon::Mode mode);
QT_GUI_C_EXPORT QPixmap* qt_gui_c_QIcon_pixmap_as_ptr_window_size_mode_state(const QIcon* this_ptr, QWindow* window, const QSize* size, QIcon::Mode mode, QIcon::State state);
QT_GUI_C_EXPORT void qt_gui_c_QIcon_setIsMask(QIcon* this_ptr, bool isMask);
QT_GUI_C_EXPORT void qt_gui_c_QIcon_setThemeName(const QString* path);
QT_GUI_C_EXPORT void qt_gui_c_QIcon_setThemeSearchPaths(const QStringList* searchpath);
QT_GUI_C_EXPORT void qt_gui_c_QIcon_swap(QIcon* this_ptr, QIcon* other);
QT_GUI_C_EXPORT void qt_gui_c_QIcon_themeName_to_output(QString* output);
QT_GUI_C_EXPORT void qt_gui_c_QIcon_themeSearchPaths_to_output(QStringList* output);

} // extern "C"

#endif // QT_GUI_C_QICON_H
