#ifndef QT_GUI_C_QPAGEDPAINTDEVICE_H
#define QT_GUI_C_QPAGEDPAINTDEVICE_H

#include "qt_gui_c_global.h"

extern "C" {

QT_GUI_C_EXPORT QPagedPaintDevice* qt_gui_c_QPagedPaintDevice_G_dynamic_cast_QPagedPaintDevice_ptr(QPaintDevice* ptr);
QT_GUI_C_EXPORT QPagedPaintDevice* qt_gui_c_QPagedPaintDevice_G_static_cast_QPagedPaintDevice_ptr(QPaintDevice* ptr);
QT_GUI_C_EXPORT QPaintDevice* qt_gui_c_QPagedPaintDevice_G_static_cast_QPaintDevice_ptr(QPagedPaintDevice* ptr);
QT_GUI_C_EXPORT double qt_gui_c_QPagedPaintDevice_Margins_bottom(const QPagedPaintDevice::Margins* this_ptr);
QT_GUI_C_EXPORT void qt_gui_c_QPagedPaintDevice_Margins_destructor(QPagedPaintDevice::Margins* this_ptr);
QT_GUI_C_EXPORT double qt_gui_c_QPagedPaintDevice_Margins_left(const QPagedPaintDevice::Margins* this_ptr);
QT_GUI_C_EXPORT double qt_gui_c_QPagedPaintDevice_Margins_right(const QPagedPaintDevice::Margins* this_ptr);
QT_GUI_C_EXPORT void qt_gui_c_QPagedPaintDevice_Margins_set_bottom(QPagedPaintDevice::Margins* this_ptr, double value);
QT_GUI_C_EXPORT void qt_gui_c_QPagedPaintDevice_Margins_set_left(QPagedPaintDevice::Margins* this_ptr, double value);
QT_GUI_C_EXPORT void qt_gui_c_QPagedPaintDevice_Margins_set_right(QPagedPaintDevice::Margins* this_ptr, double value);
QT_GUI_C_EXPORT void qt_gui_c_QPagedPaintDevice_Margins_set_top(QPagedPaintDevice::Margins* this_ptr, double value);
QT_GUI_C_EXPORT double qt_gui_c_QPagedPaintDevice_Margins_top(const QPagedPaintDevice::Margins* this_ptr);
QT_GUI_C_EXPORT void qt_gui_c_QPagedPaintDevice_delete(QPagedPaintDevice* this_ptr);
QT_GUI_C_EXPORT void qt_gui_c_QPagedPaintDevice_margins_to_output(const QPagedPaintDevice* this_ptr, QPagedPaintDevice::Margins* output);
QT_GUI_C_EXPORT bool qt_gui_c_QPagedPaintDevice_newPage(QPagedPaintDevice* this_ptr);
QT_GUI_C_EXPORT void qt_gui_c_QPagedPaintDevice_pageLayout_to_output(const QPagedPaintDevice* this_ptr, QPageLayout* output);
QT_GUI_C_EXPORT QPagedPaintDevice::PageSize qt_gui_c_QPagedPaintDevice_pageSize(const QPagedPaintDevice* this_ptr);
QT_GUI_C_EXPORT void qt_gui_c_QPagedPaintDevice_pageSizeMM_to_output(const QPagedPaintDevice* this_ptr, QSizeF* output);
QT_GUI_C_EXPORT void qt_gui_c_QPagedPaintDevice_setMargins(QPagedPaintDevice* this_ptr, const QPagedPaintDevice::Margins* margins);
QT_GUI_C_EXPORT bool qt_gui_c_QPagedPaintDevice_setPageLayout(QPagedPaintDevice* this_ptr, const QPageLayout* pageLayout);
QT_GUI_C_EXPORT bool qt_gui_c_QPagedPaintDevice_setPageMargins_margins(QPagedPaintDevice* this_ptr, const QMarginsF* margins);
QT_GUI_C_EXPORT bool qt_gui_c_QPagedPaintDevice_setPageMargins_margins_units(QPagedPaintDevice* this_ptr, const QMarginsF* margins, const QPageLayout::Unit* units);
QT_GUI_C_EXPORT bool qt_gui_c_QPagedPaintDevice_setPageOrientation(QPagedPaintDevice* this_ptr, const QPageLayout::Orientation* orientation);
QT_GUI_C_EXPORT void qt_gui_c_QPagedPaintDevice_setPageSizeMM(QPagedPaintDevice* this_ptr, const QSizeF* size);
QT_GUI_C_EXPORT bool qt_gui_c_QPagedPaintDevice_setPageSize_pageSize(QPagedPaintDevice* this_ptr, const QPageSize* pageSize);
QT_GUI_C_EXPORT void qt_gui_c_QPagedPaintDevice_setPageSize_size(QPagedPaintDevice* this_ptr, QPagedPaintDevice::PageSize size);

} // extern "C"

#endif // QT_GUI_C_QPAGEDPAINTDEVICE_H
