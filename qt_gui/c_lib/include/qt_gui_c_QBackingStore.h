#ifndef QT_GUI_C_QBACKINGSTORE_H
#define QT_GUI_C_QBACKINGSTORE_H

#include "qt_gui_c_global.h"

extern "C" {

QT_GUI_C_EXPORT void qt_gui_c_QBackingStore_beginPaint(QBackingStore* this_ptr, const QRegion* arg1);
QT_GUI_C_EXPORT void qt_gui_c_QBackingStore_delete(QBackingStore* this_ptr);
QT_GUI_C_EXPORT void qt_gui_c_QBackingStore_endPaint(QBackingStore* this_ptr);
QT_GUI_C_EXPORT void qt_gui_c_QBackingStore_flush_region(QBackingStore* this_ptr, const QRegion* region);
QT_GUI_C_EXPORT void qt_gui_c_QBackingStore_flush_region_window(QBackingStore* this_ptr, const QRegion* region, QWindow* window);
QT_GUI_C_EXPORT void qt_gui_c_QBackingStore_flush_region_window_offset(QBackingStore* this_ptr, const QRegion* region, QWindow* window, const QPoint* offset);
QT_GUI_C_EXPORT bool qt_gui_c_QBackingStore_hasStaticContents(const QBackingStore* this_ptr);
QT_GUI_C_EXPORT QBackingStore* qt_gui_c_QBackingStore_new(QWindow* window);
QT_GUI_C_EXPORT QPaintDevice* qt_gui_c_QBackingStore_paintDevice(QBackingStore* this_ptr);
QT_GUI_C_EXPORT void qt_gui_c_QBackingStore_resize(QBackingStore* this_ptr, const QSize* size);
QT_GUI_C_EXPORT bool qt_gui_c_QBackingStore_scroll(QBackingStore* this_ptr, const QRegion* area, int dx, int dy);
QT_GUI_C_EXPORT void qt_gui_c_QBackingStore_setStaticContents(QBackingStore* this_ptr, const QRegion* region);
QT_GUI_C_EXPORT void qt_gui_c_QBackingStore_size_to_output(const QBackingStore* this_ptr, QSize* output);
QT_GUI_C_EXPORT QRegion* qt_gui_c_QBackingStore_staticContents_as_ptr(const QBackingStore* this_ptr);
QT_GUI_C_EXPORT QWindow* qt_gui_c_QBackingStore_window(const QBackingStore* this_ptr);

} // extern "C"

#endif // QT_GUI_C_QBACKINGSTORE_H
