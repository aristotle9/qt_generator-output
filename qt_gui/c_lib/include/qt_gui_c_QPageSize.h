#ifndef QT_GUI_C_QPAGESIZE_H
#define QT_GUI_C_QPAGESIZE_H

#include "qt_gui_c_global.h"

extern "C" {

QT_GUI_C_EXPORT bool qt_gui_c_QPageSize_G_operator_neq(const QPageSize* lhs, const QPageSize* rhs);
QT_GUI_C_EXPORT void qt_gui_c_QPageSize_G_operator_shl_to_output(const QDebug* dbg, const QPageSize* pageSize, QDebug* output);
QT_GUI_C_EXPORT void qt_gui_c_QPageSize_G_swap(QPageSize* value1, QPageSize* value2);
QT_GUI_C_EXPORT void qt_gui_c_QPageSize_constructor_no_args(QPageSize* output);
QT_GUI_C_EXPORT void qt_gui_c_QPageSize_constructor_other(const QPageSize* other, QPageSize* output);
QT_GUI_C_EXPORT void qt_gui_c_QPageSize_constructor_pageSizeId(QPageSize::PageSizeId pageSizeId, QPageSize* output);
QT_GUI_C_EXPORT void qt_gui_c_QPageSize_constructor_pointSize(const QSize* pointSize, QPageSize* output);
QT_GUI_C_EXPORT void qt_gui_c_QPageSize_constructor_pointSize_name(const QSize* pointSize, const QString* name, QPageSize* output);
QT_GUI_C_EXPORT void qt_gui_c_QPageSize_constructor_pointSize_name_matchPolicy(const QSize* pointSize, const QString* name, QPageSize::SizeMatchPolicy matchPolicy, QPageSize* output);
QT_GUI_C_EXPORT void qt_gui_c_QPageSize_constructor_size_units(const QSizeF* size, QPageSize::Unit units, QPageSize* output);
QT_GUI_C_EXPORT void qt_gui_c_QPageSize_constructor_size_units_name(const QSizeF* size, QPageSize::Unit units, const QString* name, QPageSize* output);
QT_GUI_C_EXPORT void qt_gui_c_QPageSize_constructor_size_units_name_matchPolicy(const QSizeF* size, QPageSize::Unit units, const QString* name, QPageSize::SizeMatchPolicy matchPolicy, QPageSize* output);
QT_GUI_C_EXPORT void qt_gui_c_QPageSize_definitionSize_to_output_no_args(const QPageSize* this_ptr, QSizeF* output);
QT_GUI_C_EXPORT void qt_gui_c_QPageSize_definitionSize_to_output_pageSizeId(QPageSize::PageSizeId pageSizeId, QSizeF* output);
QT_GUI_C_EXPORT QPageSize::Unit qt_gui_c_QPageSize_definitionUnits_no_args(const QPageSize* this_ptr);
QT_GUI_C_EXPORT QPageSize::Unit qt_gui_c_QPageSize_definitionUnits_pageSizeId(QPageSize::PageSizeId pageSizeId);
QT_GUI_C_EXPORT void qt_gui_c_QPageSize_destructor(QPageSize* this_ptr);
QT_GUI_C_EXPORT QPageSize::PageSizeId qt_gui_c_QPageSize_id_no_args(const QPageSize* this_ptr);
QT_GUI_C_EXPORT QPageSize::PageSizeId qt_gui_c_QPageSize_id_pointSize(const QSize* pointSize);
QT_GUI_C_EXPORT QPageSize::PageSizeId qt_gui_c_QPageSize_id_pointSize_matchPolicy(const QSize* pointSize, QPageSize::SizeMatchPolicy matchPolicy);
QT_GUI_C_EXPORT QPageSize::PageSizeId qt_gui_c_QPageSize_id_size_units(const QSizeF* size, QPageSize::Unit units);
QT_GUI_C_EXPORT QPageSize::PageSizeId qt_gui_c_QPageSize_id_size_units_matchPolicy(const QSizeF* size, QPageSize::Unit units, QPageSize::SizeMatchPolicy matchPolicy);
QT_GUI_C_EXPORT QPageSize::PageSizeId qt_gui_c_QPageSize_id_windowsId(int windowsId);
QT_GUI_C_EXPORT bool qt_gui_c_QPageSize_isEquivalentTo(const QPageSize* this_ptr, const QPageSize* other);
QT_GUI_C_EXPORT bool qt_gui_c_QPageSize_isValid(const QPageSize* this_ptr);
QT_GUI_C_EXPORT void qt_gui_c_QPageSize_key_to_output_no_args(const QPageSize* this_ptr, QString* output);
QT_GUI_C_EXPORT void qt_gui_c_QPageSize_key_to_output_pageSizeId(QPageSize::PageSizeId pageSizeId, QString* output);
QT_GUI_C_EXPORT void qt_gui_c_QPageSize_name_to_output_no_args(const QPageSize* this_ptr, QString* output);
QT_GUI_C_EXPORT void qt_gui_c_QPageSize_name_to_output_pageSizeId(QPageSize::PageSizeId pageSizeId, QString* output);
QT_GUI_C_EXPORT QPageSize* qt_gui_c_QPageSize_operator_assign(QPageSize* this_ptr, const QPageSize* other);
QT_GUI_C_EXPORT void qt_gui_c_QPageSize_rectPixels_to_output(const QPageSize* this_ptr, int resolution, QRect* output);
QT_GUI_C_EXPORT void qt_gui_c_QPageSize_rectPoints_to_output(const QPageSize* this_ptr, QRect* output);
QT_GUI_C_EXPORT void qt_gui_c_QPageSize_rect_to_output(const QPageSize* this_ptr, QPageSize::Unit units, QRectF* output);
QT_GUI_C_EXPORT void qt_gui_c_QPageSize_sizePixels_to_output_pageSizeId_resolution(QPageSize::PageSizeId pageSizeId, int resolution, QSize* output);
QT_GUI_C_EXPORT void qt_gui_c_QPageSize_sizePixels_to_output_resolution(const QPageSize* this_ptr, int resolution, QSize* output);
QT_GUI_C_EXPORT void qt_gui_c_QPageSize_sizePoints_to_output_no_args(const QPageSize* this_ptr, QSize* output);
QT_GUI_C_EXPORT void qt_gui_c_QPageSize_sizePoints_to_output_pageSizeId(QPageSize::PageSizeId pageSizeId, QSize* output);
QT_GUI_C_EXPORT void qt_gui_c_QPageSize_size_to_output_pageSizeId_units(QPageSize::PageSizeId pageSizeId, QPageSize::Unit units, QSizeF* output);
QT_GUI_C_EXPORT void qt_gui_c_QPageSize_size_to_output_units(const QPageSize* this_ptr, QPageSize::Unit units, QSizeF* output);
QT_GUI_C_EXPORT void qt_gui_c_QPageSize_swap(QPageSize* this_ptr, QPageSize* other);
QT_GUI_C_EXPORT int qt_gui_c_QPageSize_windowsId_no_args(const QPageSize* this_ptr);
QT_GUI_C_EXPORT int qt_gui_c_QPageSize_windowsId_pageSizeId(QPageSize::PageSizeId pageSizeId);

} // extern "C"

#endif // QT_GUI_C_QPAGESIZE_H
