#ifndef QT_GUI_C_QCLIPBOARD_H
#define QT_GUI_C_QCLIPBOARD_H

#include "qt_gui_c_global.h"

extern "C" {

QT_GUI_C_EXPORT QClipboard* qt_gui_c_QClipboard_G_static_cast_QClipboard_ptr(QObject* ptr);
QT_GUI_C_EXPORT QObject* qt_gui_c_QClipboard_G_static_cast_QObject_ptr(QClipboard* ptr);
QT_GUI_C_EXPORT void qt_gui_c_QClipboard_clear_mode(QClipboard* this_ptr, QClipboard::Mode mode);
QT_GUI_C_EXPORT void qt_gui_c_QClipboard_clear_no_args(QClipboard* this_ptr);
QT_GUI_C_EXPORT QImage* qt_gui_c_QClipboard_image_as_ptr_mode(const QClipboard* this_ptr, QClipboard::Mode mode);
QT_GUI_C_EXPORT QImage* qt_gui_c_QClipboard_image_as_ptr_no_args(const QClipboard* this_ptr);
QT_GUI_C_EXPORT const QMetaObject* qt_gui_c_QClipboard_metaObject(const QClipboard* this_ptr);
QT_GUI_C_EXPORT const QMimeData* qt_gui_c_QClipboard_mimeData_mode(const QClipboard* this_ptr, QClipboard::Mode mode);
QT_GUI_C_EXPORT const QMimeData* qt_gui_c_QClipboard_mimeData_no_args(const QClipboard* this_ptr);
QT_GUI_C_EXPORT bool qt_gui_c_QClipboard_ownsClipboard(const QClipboard* this_ptr);
QT_GUI_C_EXPORT bool qt_gui_c_QClipboard_ownsFindBuffer(const QClipboard* this_ptr);
QT_GUI_C_EXPORT bool qt_gui_c_QClipboard_ownsSelection(const QClipboard* this_ptr);
QT_GUI_C_EXPORT QPixmap* qt_gui_c_QClipboard_pixmap_as_ptr_mode(const QClipboard* this_ptr, QClipboard::Mode mode);
QT_GUI_C_EXPORT QPixmap* qt_gui_c_QClipboard_pixmap_as_ptr_no_args(const QClipboard* this_ptr);
QT_GUI_C_EXPORT int qt_gui_c_QClipboard_qt_metacall(QClipboard* this_ptr, const QMetaObject::Call* arg1, int arg2, void** arg3);
QT_GUI_C_EXPORT void* qt_gui_c_QClipboard_qt_metacast(QClipboard* this_ptr, const char* arg1);
QT_GUI_C_EXPORT void qt_gui_c_QClipboard_setImage_arg1(QClipboard* this_ptr, const QImage* arg1);
QT_GUI_C_EXPORT void qt_gui_c_QClipboard_setImage_arg1_mode(QClipboard* this_ptr, const QImage* arg1, QClipboard::Mode mode);
QT_GUI_C_EXPORT void qt_gui_c_QClipboard_setMimeData_data(QClipboard* this_ptr, QMimeData* data);
QT_GUI_C_EXPORT void qt_gui_c_QClipboard_setMimeData_data_mode(QClipboard* this_ptr, QMimeData* data, QClipboard::Mode mode);
QT_GUI_C_EXPORT void qt_gui_c_QClipboard_setPixmap_arg1(QClipboard* this_ptr, const QPixmap* arg1);
QT_GUI_C_EXPORT void qt_gui_c_QClipboard_setPixmap_arg1_mode(QClipboard* this_ptr, const QPixmap* arg1, QClipboard::Mode mode);
QT_GUI_C_EXPORT void qt_gui_c_QClipboard_setText_arg1(QClipboard* this_ptr, const QString* arg1);
QT_GUI_C_EXPORT void qt_gui_c_QClipboard_setText_arg1_mode(QClipboard* this_ptr, const QString* arg1, QClipboard::Mode mode);
QT_GUI_C_EXPORT bool qt_gui_c_QClipboard_supportsFindBuffer(const QClipboard* this_ptr);
QT_GUI_C_EXPORT bool qt_gui_c_QClipboard_supportsSelection(const QClipboard* this_ptr);
QT_GUI_C_EXPORT void qt_gui_c_QClipboard_text_to_output_mode(const QClipboard* this_ptr, QClipboard::Mode mode, QString* output);
QT_GUI_C_EXPORT void qt_gui_c_QClipboard_text_to_output_no_args(const QClipboard* this_ptr, QString* output);
QT_GUI_C_EXPORT void qt_gui_c_QClipboard_text_to_output_subtype(const QClipboard* this_ptr, QString* subtype, QString* output);
QT_GUI_C_EXPORT void qt_gui_c_QClipboard_text_to_output_subtype_mode(const QClipboard* this_ptr, QString* subtype, QClipboard::Mode mode, QString* output);
QT_GUI_C_EXPORT void qt_gui_c_QClipboard_trUtf8_to_output(const char* s, const char* c, int n, QString* output);
QT_GUI_C_EXPORT void qt_gui_c_QClipboard_tr_to_output(const char* s, const char* c, int n, QString* output);

} // extern "C"

#endif // QT_GUI_C_QCLIPBOARD_H
