#ifndef QT_GUI_C_QBRUSH_H
#define QT_GUI_C_QBRUSH_H

#include "qt_gui_c_global.h"

extern "C" {

QT_GUI_C_EXPORT QDataStream* qt_gui_c_QBrush_G_operator_shl(QDataStream* arg1, const QBrush* arg2);
QT_GUI_C_EXPORT void qt_gui_c_QBrush_G_operator_shl_to_output(const QDebug* arg1, const QBrush* arg2, QDebug* output);
QT_GUI_C_EXPORT QDataStream* qt_gui_c_QBrush_G_operator_shr(QDataStream* arg1, QBrush* arg2);
QT_GUI_C_EXPORT void qt_gui_c_QBrush_G_swap(QBrush* value1, QBrush* value2);
QT_GUI_C_EXPORT const QColor* qt_gui_c_QBrush_color(const QBrush* this_ptr);
QT_GUI_C_EXPORT void qt_gui_c_QBrush_constructor_QBrush(const QBrush* brush, QBrush* output);
QT_GUI_C_EXPORT void qt_gui_c_QBrush_constructor_QColor(const QColor* color, QBrush* output);
QT_GUI_C_EXPORT void qt_gui_c_QBrush_constructor_QColor_QPixmap(const QColor* color, const QPixmap* pixmap, QBrush* output);
QT_GUI_C_EXPORT void qt_gui_c_QBrush_constructor_QColor_Qt_BrushStyle(const QColor* color, const Qt::BrushStyle* bs, QBrush* output);
QT_GUI_C_EXPORT void qt_gui_c_QBrush_constructor_QGradient(const QGradient* gradient, QBrush* output);
QT_GUI_C_EXPORT void qt_gui_c_QBrush_constructor_QImage(const QImage* image, QBrush* output);
QT_GUI_C_EXPORT void qt_gui_c_QBrush_constructor_QPixmap(const QPixmap* pixmap, QBrush* output);
QT_GUI_C_EXPORT void qt_gui_c_QBrush_constructor_Qt_BrushStyle(const Qt::BrushStyle* bs, QBrush* output);
QT_GUI_C_EXPORT void qt_gui_c_QBrush_constructor_Qt_GlobalColor(const Qt::GlobalColor* color, QBrush* output);
QT_GUI_C_EXPORT void qt_gui_c_QBrush_constructor_Qt_GlobalColor_QPixmap(const Qt::GlobalColor* color, const QPixmap* pixmap, QBrush* output);
QT_GUI_C_EXPORT void qt_gui_c_QBrush_constructor_Qt_GlobalColor_Qt_BrushStyle(const Qt::GlobalColor* color, const Qt::BrushStyle* bs, QBrush* output);
QT_GUI_C_EXPORT void qt_gui_c_QBrush_constructor_no_args(QBrush* output);
QT_GUI_C_EXPORT void qt_gui_c_QBrush_convert_to_QVariant_to_output(const QBrush* this_ptr, QVariant* output);
QT_GUI_C_EXPORT void qt_gui_c_QBrush_destructor(QBrush* this_ptr);
QT_GUI_C_EXPORT const QGradient* qt_gui_c_QBrush_gradient(const QBrush* this_ptr);
QT_GUI_C_EXPORT bool qt_gui_c_QBrush_isDetached(const QBrush* this_ptr);
QT_GUI_C_EXPORT bool qt_gui_c_QBrush_isOpaque(const QBrush* this_ptr);
QT_GUI_C_EXPORT const QMatrix* qt_gui_c_QBrush_matrix(const QBrush* this_ptr);
QT_GUI_C_EXPORT QBrush* qt_gui_c_QBrush_operator_assign(QBrush* this_ptr, const QBrush* brush);
QT_GUI_C_EXPORT bool qt_gui_c_QBrush_operator_eq(const QBrush* this_ptr, const QBrush* b);
QT_GUI_C_EXPORT bool qt_gui_c_QBrush_operator_neq(const QBrush* this_ptr, const QBrush* b);
QT_GUI_C_EXPORT void qt_gui_c_QBrush_setColor_QColor(QBrush* this_ptr, const QColor* color);
QT_GUI_C_EXPORT void qt_gui_c_QBrush_setColor_Qt_GlobalColor(QBrush* this_ptr, const Qt::GlobalColor* color);
QT_GUI_C_EXPORT void qt_gui_c_QBrush_setMatrix(QBrush* this_ptr, const QMatrix* mat);
QT_GUI_C_EXPORT void qt_gui_c_QBrush_setStyle(QBrush* this_ptr, const Qt::BrushStyle* arg1);
QT_GUI_C_EXPORT void qt_gui_c_QBrush_setTexture(QBrush* this_ptr, const QPixmap* pixmap);
QT_GUI_C_EXPORT void qt_gui_c_QBrush_setTextureImage(QBrush* this_ptr, const QImage* image);
QT_GUI_C_EXPORT void qt_gui_c_QBrush_setTransform(QBrush* this_ptr, const QTransform* arg1);
QT_GUI_C_EXPORT void qt_gui_c_QBrush_swap(QBrush* this_ptr, QBrush* other);
QT_GUI_C_EXPORT QImage* qt_gui_c_QBrush_textureImage_as_ptr(const QBrush* this_ptr);
QT_GUI_C_EXPORT QPixmap* qt_gui_c_QBrush_texture_as_ptr(const QBrush* this_ptr);
QT_GUI_C_EXPORT void qt_gui_c_QBrush_transform_to_output(const QBrush* this_ptr, QTransform* output);

} // extern "C"

#endif // QT_GUI_C_QBRUSH_H
