#ifndef QT_GUI_C_QDRAG_H
#define QT_GUI_C_QDRAG_H

#include "qt_gui_c_global.h"

extern "C" {

QT_GUI_C_EXPORT QDrag* qt_gui_c_QDrag_G_static_cast_QDrag_ptr(QObject* ptr);
QT_GUI_C_EXPORT QObject* qt_gui_c_QDrag_G_static_cast_QObject_ptr(QDrag* ptr);
QT_GUI_C_EXPORT void qt_gui_c_QDrag_cancel();
QT_GUI_C_EXPORT void qt_gui_c_QDrag_delete(QDrag* this_ptr);
QT_GUI_C_EXPORT QPixmap* qt_gui_c_QDrag_dragCursor_as_ptr(const QDrag* this_ptr, const Qt::DropAction* action);
QT_GUI_C_EXPORT void qt_gui_c_QDrag_hotSpot_to_output(const QDrag* this_ptr, QPoint* output);
QT_GUI_C_EXPORT const QMetaObject* qt_gui_c_QDrag_metaObject(const QDrag* this_ptr);
QT_GUI_C_EXPORT QMimeData* qt_gui_c_QDrag_mimeData(const QDrag* this_ptr);
QT_GUI_C_EXPORT QDrag* qt_gui_c_QDrag_new(QObject* dragSource);
QT_GUI_C_EXPORT QPixmap* qt_gui_c_QDrag_pixmap_as_ptr(const QDrag* this_ptr);
QT_GUI_C_EXPORT int qt_gui_c_QDrag_qt_metacall(QDrag* this_ptr, const QMetaObject::Call* arg1, int arg2, void** arg3);
QT_GUI_C_EXPORT void* qt_gui_c_QDrag_qt_metacast(QDrag* this_ptr, const char* arg1);
QT_GUI_C_EXPORT void qt_gui_c_QDrag_setDragCursor(QDrag* this_ptr, const QPixmap* cursor, const Qt::DropAction* action);
QT_GUI_C_EXPORT void qt_gui_c_QDrag_setHotSpot(QDrag* this_ptr, const QPoint* hotspot);
QT_GUI_C_EXPORT void qt_gui_c_QDrag_setMimeData(QDrag* this_ptr, QMimeData* data);
QT_GUI_C_EXPORT void qt_gui_c_QDrag_setPixmap(QDrag* this_ptr, const QPixmap* arg1);
QT_GUI_C_EXPORT QObject* qt_gui_c_QDrag_source(const QDrag* this_ptr);
QT_GUI_C_EXPORT QObject* qt_gui_c_QDrag_target(const QDrag* this_ptr);
QT_GUI_C_EXPORT void qt_gui_c_QDrag_trUtf8_to_output(const char* s, const char* c, int n, QString* output);
QT_GUI_C_EXPORT void qt_gui_c_QDrag_tr_to_output(const char* s, const char* c, int n, QString* output);

} // extern "C"

#endif // QT_GUI_C_QDRAG_H
