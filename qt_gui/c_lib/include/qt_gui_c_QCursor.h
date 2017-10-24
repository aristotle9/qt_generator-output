#ifndef QT_GUI_C_QCURSOR_H
#define QT_GUI_C_QCURSOR_H

#include "qt_gui_c_global.h"

extern "C" {

QT_GUI_C_EXPORT QDataStream* qt_gui_c_QCursor_G_operator_shl(QDataStream* outS, const QCursor* cursor);
QT_GUI_C_EXPORT void qt_gui_c_QCursor_G_operator_shl_to_output(const QDebug* arg1, const QCursor* arg2, QDebug* output);
QT_GUI_C_EXPORT QDataStream* qt_gui_c_QCursor_G_operator_shr(QDataStream* inS, QCursor* cursor);
QT_GUI_C_EXPORT void qt_gui_c_QCursor_G_swap(QCursor* value1, QCursor* value2);
QT_GUI_C_EXPORT const QBitmap* qt_gui_c_QCursor_bitmap(const QCursor* this_ptr);
QT_GUI_C_EXPORT void qt_gui_c_QCursor_convert_to_QVariant_to_output(const QCursor* this_ptr, QVariant* output);
QT_GUI_C_EXPORT void qt_gui_c_QCursor_delete(QCursor* this_ptr);
QT_GUI_C_EXPORT void qt_gui_c_QCursor_hotSpot_to_output(const QCursor* this_ptr, QPoint* output);
QT_GUI_C_EXPORT const QBitmap* qt_gui_c_QCursor_mask(const QCursor* this_ptr);
QT_GUI_C_EXPORT QCursor* qt_gui_c_QCursor_new_bitmap_mask(const QBitmap* bitmap, const QBitmap* mask);
QT_GUI_C_EXPORT QCursor* qt_gui_c_QCursor_new_bitmap_mask_hotX(const QBitmap* bitmap, const QBitmap* mask, int hotX);
QT_GUI_C_EXPORT QCursor* qt_gui_c_QCursor_new_bitmap_mask_hotX_hotY(const QBitmap* bitmap, const QBitmap* mask, int hotX, int hotY);
QT_GUI_C_EXPORT QCursor* qt_gui_c_QCursor_new_cursor(const QCursor* cursor);
QT_GUI_C_EXPORT QCursor* qt_gui_c_QCursor_new_no_args();
QT_GUI_C_EXPORT QCursor* qt_gui_c_QCursor_new_pixmap(const QPixmap* pixmap);
QT_GUI_C_EXPORT QCursor* qt_gui_c_QCursor_new_pixmap_hotX(const QPixmap* pixmap, int hotX);
QT_GUI_C_EXPORT QCursor* qt_gui_c_QCursor_new_pixmap_hotX_hotY(const QPixmap* pixmap, int hotX, int hotY);
QT_GUI_C_EXPORT QCursor* qt_gui_c_QCursor_new_shape(const Qt::CursorShape* shape);
QT_GUI_C_EXPORT QCursor* qt_gui_c_QCursor_operator_assign(QCursor* this_ptr, const QCursor* cursor);
QT_GUI_C_EXPORT QPixmap* qt_gui_c_QCursor_pixmap_as_ptr(const QCursor* this_ptr);
QT_GUI_C_EXPORT void qt_gui_c_QCursor_pos_to_output_no_args(QPoint* output);
QT_GUI_C_EXPORT void qt_gui_c_QCursor_pos_to_output_screen(const QScreen* screen, QPoint* output);
QT_GUI_C_EXPORT void qt_gui_c_QCursor_setPos_p(const QPoint* p);
QT_GUI_C_EXPORT void qt_gui_c_QCursor_setPos_screen_p(QScreen* screen, const QPoint* p);
QT_GUI_C_EXPORT void qt_gui_c_QCursor_setPos_screen_x_y(QScreen* screen, int x, int y);
QT_GUI_C_EXPORT void qt_gui_c_QCursor_setPos_x_y(int x, int y);
QT_GUI_C_EXPORT void qt_gui_c_QCursor_setShape(QCursor* this_ptr, const Qt::CursorShape* newShape);
QT_GUI_C_EXPORT void qt_gui_c_QCursor_swap(QCursor* this_ptr, QCursor* other);

} // extern "C"

#endif // QT_GUI_C_QCURSOR_H
