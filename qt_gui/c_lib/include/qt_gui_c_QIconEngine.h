#ifndef QT_GUI_C_QICONENGINE_H
#define QT_GUI_C_QICONENGINE_H

#include "qt_gui_c_global.h"

extern "C" {

QT_GUI_C_EXPORT void qt_gui_c_QIconEngine_AvailableSizesArgument_delete(QIconEngine::AvailableSizesArgument* this_ptr);
QT_GUI_C_EXPORT const QIcon::Mode* qt_gui_c_QIconEngine_AvailableSizesArgument_mode(const QIconEngine::AvailableSizesArgument* this_ptr);
QT_GUI_C_EXPORT QIcon::Mode* qt_gui_c_QIconEngine_AvailableSizesArgument_mode_mut(QIconEngine::AvailableSizesArgument* this_ptr);
QT_GUI_C_EXPORT void qt_gui_c_QIconEngine_AvailableSizesArgument_set_mode(QIconEngine::AvailableSizesArgument* this_ptr, const QIcon::Mode* value);
QT_GUI_C_EXPORT void qt_gui_c_QIconEngine_AvailableSizesArgument_set_sizes(QIconEngine::AvailableSizesArgument* this_ptr, const QList< QSize >* value);
QT_GUI_C_EXPORT void qt_gui_c_QIconEngine_AvailableSizesArgument_set_state(QIconEngine::AvailableSizesArgument* this_ptr, const QIcon::State* value);
QT_GUI_C_EXPORT const QList< QSize >* qt_gui_c_QIconEngine_AvailableSizesArgument_sizes(const QIconEngine::AvailableSizesArgument* this_ptr);
QT_GUI_C_EXPORT QList< QSize >* qt_gui_c_QIconEngine_AvailableSizesArgument_sizes_mut(QIconEngine::AvailableSizesArgument* this_ptr);
QT_GUI_C_EXPORT const QIcon::State* qt_gui_c_QIconEngine_AvailableSizesArgument_state(const QIconEngine::AvailableSizesArgument* this_ptr);
QT_GUI_C_EXPORT QIcon::State* qt_gui_c_QIconEngine_AvailableSizesArgument_state_mut(QIconEngine::AvailableSizesArgument* this_ptr);
QT_GUI_C_EXPORT void qt_gui_c_QIconEngine_ScaledPixmapArgument_delete(QIconEngine::ScaledPixmapArgument* this_ptr);
QT_GUI_C_EXPORT const QIcon::Mode* qt_gui_c_QIconEngine_ScaledPixmapArgument_mode(const QIconEngine::ScaledPixmapArgument* this_ptr);
QT_GUI_C_EXPORT QIcon::Mode* qt_gui_c_QIconEngine_ScaledPixmapArgument_mode_mut(QIconEngine::ScaledPixmapArgument* this_ptr);
QT_GUI_C_EXPORT const QPixmap* qt_gui_c_QIconEngine_ScaledPixmapArgument_pixmap(const QIconEngine::ScaledPixmapArgument* this_ptr);
QT_GUI_C_EXPORT QPixmap* qt_gui_c_QIconEngine_ScaledPixmapArgument_pixmap_mut(QIconEngine::ScaledPixmapArgument* this_ptr);
QT_GUI_C_EXPORT double qt_gui_c_QIconEngine_ScaledPixmapArgument_scale(const QIconEngine::ScaledPixmapArgument* this_ptr);
QT_GUI_C_EXPORT void qt_gui_c_QIconEngine_ScaledPixmapArgument_set_mode(QIconEngine::ScaledPixmapArgument* this_ptr, const QIcon::Mode* value);
QT_GUI_C_EXPORT void qt_gui_c_QIconEngine_ScaledPixmapArgument_set_pixmap(QIconEngine::ScaledPixmapArgument* this_ptr, const QPixmap* value);
QT_GUI_C_EXPORT void qt_gui_c_QIconEngine_ScaledPixmapArgument_set_scale(QIconEngine::ScaledPixmapArgument* this_ptr, double value);
QT_GUI_C_EXPORT void qt_gui_c_QIconEngine_ScaledPixmapArgument_set_size(QIconEngine::ScaledPixmapArgument* this_ptr, const QSize* value);
QT_GUI_C_EXPORT void qt_gui_c_QIconEngine_ScaledPixmapArgument_set_state(QIconEngine::ScaledPixmapArgument* this_ptr, const QIcon::State* value);
QT_GUI_C_EXPORT const QSize* qt_gui_c_QIconEngine_ScaledPixmapArgument_size(const QIconEngine::ScaledPixmapArgument* this_ptr);
QT_GUI_C_EXPORT QSize* qt_gui_c_QIconEngine_ScaledPixmapArgument_size_mut(QIconEngine::ScaledPixmapArgument* this_ptr);
QT_GUI_C_EXPORT const QIcon::State* qt_gui_c_QIconEngine_ScaledPixmapArgument_state(const QIconEngine::ScaledPixmapArgument* this_ptr);
QT_GUI_C_EXPORT QIcon::State* qt_gui_c_QIconEngine_ScaledPixmapArgument_state_mut(QIconEngine::ScaledPixmapArgument* this_ptr);
QT_GUI_C_EXPORT void qt_gui_c_QIconEngine_actualSize_to_output(QIconEngine* this_ptr, const QSize* size, const QIcon::Mode* mode, const QIcon::State* state, QSize* output);
QT_GUI_C_EXPORT void qt_gui_c_QIconEngine_addFile(QIconEngine* this_ptr, const QString* fileName, const QSize* size, const QIcon::Mode* mode, const QIcon::State* state);
QT_GUI_C_EXPORT void qt_gui_c_QIconEngine_addPixmap(QIconEngine* this_ptr, const QPixmap* pixmap, const QIcon::Mode* mode, const QIcon::State* state);
QT_GUI_C_EXPORT void qt_gui_c_QIconEngine_availableSizes_to_output_mode(const QIconEngine* this_ptr, const QIcon::Mode* mode, QList< QSize >* output);
QT_GUI_C_EXPORT void qt_gui_c_QIconEngine_availableSizes_to_output_mode_state(const QIconEngine* this_ptr, const QIcon::Mode* mode, const QIcon::State* state, QList< QSize >* output);
QT_GUI_C_EXPORT void qt_gui_c_QIconEngine_availableSizes_to_output_no_args(const QIconEngine* this_ptr, QList< QSize >* output);
QT_GUI_C_EXPORT QIconEngine* qt_gui_c_QIconEngine_clone(const QIconEngine* this_ptr);
QT_GUI_C_EXPORT void qt_gui_c_QIconEngine_delete(QIconEngine* this_ptr);
QT_GUI_C_EXPORT void qt_gui_c_QIconEngine_iconName_to_output(const QIconEngine* this_ptr, QString* output);
QT_GUI_C_EXPORT bool qt_gui_c_QIconEngine_isNull(const QIconEngine* this_ptr);
QT_GUI_C_EXPORT void qt_gui_c_QIconEngine_key_to_output(const QIconEngine* this_ptr, QString* output);
QT_GUI_C_EXPORT void qt_gui_c_QIconEngine_paint(QIconEngine* this_ptr, QPainter* painter, const QRect* rect, const QIcon::Mode* mode, const QIcon::State* state);
QT_GUI_C_EXPORT QPixmap* qt_gui_c_QIconEngine_pixmap_as_ptr(QIconEngine* this_ptr, const QSize* size, const QIcon::Mode* mode, const QIcon::State* state);
QT_GUI_C_EXPORT bool qt_gui_c_QIconEngine_read(QIconEngine* this_ptr, QDataStream* in);
QT_GUI_C_EXPORT QPixmap* qt_gui_c_QIconEngine_scaledPixmap_as_ptr(QIconEngine* this_ptr, const QSize* size, const QIcon::Mode* mode, const QIcon::State* state, double scale);
QT_GUI_C_EXPORT void qt_gui_c_QIconEngine_virtual_hook(QIconEngine* this_ptr, int id, void* data);
QT_GUI_C_EXPORT bool qt_gui_c_QIconEngine_write(const QIconEngine* this_ptr, QDataStream* out);

} // extern "C"

#endif // QT_GUI_C_QICONENGINE_H
