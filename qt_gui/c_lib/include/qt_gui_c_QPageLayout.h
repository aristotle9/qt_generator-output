#ifndef QT_GUI_C_QPAGELAYOUT_H
#define QT_GUI_C_QPAGELAYOUT_H

#include "qt_gui_c_global.h"

extern "C" {

QT_GUI_C_EXPORT bool qt_gui_c_QPageLayout_G_operator_neq(const QPageLayout* lhs, const QPageLayout* rhs);
QT_GUI_C_EXPORT void qt_gui_c_QPageLayout_G_operator_shl_to_output(const QDebug* dbg, const QPageLayout* pageLayout, QDebug* output);
QT_GUI_C_EXPORT void qt_gui_c_QPageLayout_G_swap(QPageLayout* value1, QPageLayout* value2);
QT_GUI_C_EXPORT void qt_gui_c_QPageLayout_constructor_no_args(QPageLayout* output);
QT_GUI_C_EXPORT void qt_gui_c_QPageLayout_constructor_other(const QPageLayout* other, QPageLayout* output);
QT_GUI_C_EXPORT void qt_gui_c_QPageLayout_constructor_pageSize_orientation_margins(const QPageSize* pageSize, QPageLayout::Orientation orientation, const QMarginsF* margins, QPageLayout* output);
QT_GUI_C_EXPORT void qt_gui_c_QPageLayout_constructor_pageSize_orientation_margins_units(const QPageSize* pageSize, QPageLayout::Orientation orientation, const QMarginsF* margins, QPageLayout::Unit units, QPageLayout* output);
QT_GUI_C_EXPORT void qt_gui_c_QPageLayout_constructor_pageSize_orientation_margins_units_minMargins(const QPageSize* pageSize, QPageLayout::Orientation orientation, const QMarginsF* margins, QPageLayout::Unit units, const QMarginsF* minMargins, QPageLayout* output);
QT_GUI_C_EXPORT void qt_gui_c_QPageLayout_destructor(QPageLayout* this_ptr);
QT_GUI_C_EXPORT void qt_gui_c_QPageLayout_fullRectPixels_to_output(const QPageLayout* this_ptr, int resolution, QRect* output);
QT_GUI_C_EXPORT void qt_gui_c_QPageLayout_fullRectPoints_to_output(const QPageLayout* this_ptr, QRect* output);
QT_GUI_C_EXPORT void qt_gui_c_QPageLayout_fullRect_to_output_no_args(const QPageLayout* this_ptr, QRectF* output);
QT_GUI_C_EXPORT void qt_gui_c_QPageLayout_fullRect_to_output_units(const QPageLayout* this_ptr, QPageLayout::Unit units, QRectF* output);
QT_GUI_C_EXPORT bool qt_gui_c_QPageLayout_isEquivalentTo(const QPageLayout* this_ptr, const QPageLayout* other);
QT_GUI_C_EXPORT bool qt_gui_c_QPageLayout_isValid(const QPageLayout* this_ptr);
QT_GUI_C_EXPORT void qt_gui_c_QPageLayout_marginsPixels_to_output(const QPageLayout* this_ptr, int resolution, QMargins* output);
QT_GUI_C_EXPORT void qt_gui_c_QPageLayout_marginsPoints_to_output(const QPageLayout* this_ptr, QMargins* output);
QT_GUI_C_EXPORT void qt_gui_c_QPageLayout_margins_to_output_no_args(const QPageLayout* this_ptr, QMarginsF* output);
QT_GUI_C_EXPORT void qt_gui_c_QPageLayout_margins_to_output_units(const QPageLayout* this_ptr, QPageLayout::Unit units, QMarginsF* output);
QT_GUI_C_EXPORT void qt_gui_c_QPageLayout_maximumMargins_to_output(const QPageLayout* this_ptr, QMarginsF* output);
QT_GUI_C_EXPORT void qt_gui_c_QPageLayout_minimumMargins_to_output(const QPageLayout* this_ptr, QMarginsF* output);
QT_GUI_C_EXPORT QPageLayout::Mode qt_gui_c_QPageLayout_mode(const QPageLayout* this_ptr);
QT_GUI_C_EXPORT QPageLayout* qt_gui_c_QPageLayout_operator_assign(QPageLayout* this_ptr, const QPageLayout* other);
QT_GUI_C_EXPORT QPageLayout::Orientation qt_gui_c_QPageLayout_orientation(const QPageLayout* this_ptr);
QT_GUI_C_EXPORT void qt_gui_c_QPageLayout_pageSize_to_output(const QPageLayout* this_ptr, QPageSize* output);
QT_GUI_C_EXPORT void qt_gui_c_QPageLayout_paintRectPixels_to_output(const QPageLayout* this_ptr, int resolution, QRect* output);
QT_GUI_C_EXPORT void qt_gui_c_QPageLayout_paintRectPoints_to_output(const QPageLayout* this_ptr, QRect* output);
QT_GUI_C_EXPORT void qt_gui_c_QPageLayout_paintRect_to_output_no_args(const QPageLayout* this_ptr, QRectF* output);
QT_GUI_C_EXPORT void qt_gui_c_QPageLayout_paintRect_to_output_units(const QPageLayout* this_ptr, QPageLayout::Unit units, QRectF* output);
QT_GUI_C_EXPORT bool qt_gui_c_QPageLayout_setBottomMargin(QPageLayout* this_ptr, double bottomMargin);
QT_GUI_C_EXPORT bool qt_gui_c_QPageLayout_setLeftMargin(QPageLayout* this_ptr, double leftMargin);
QT_GUI_C_EXPORT bool qt_gui_c_QPageLayout_setMargins(QPageLayout* this_ptr, const QMarginsF* margins);
QT_GUI_C_EXPORT void qt_gui_c_QPageLayout_setMinimumMargins(QPageLayout* this_ptr, const QMarginsF* minMargins);
QT_GUI_C_EXPORT void qt_gui_c_QPageLayout_setMode(QPageLayout* this_ptr, QPageLayout::Mode mode);
QT_GUI_C_EXPORT void qt_gui_c_QPageLayout_setOrientation(QPageLayout* this_ptr, QPageLayout::Orientation orientation);
QT_GUI_C_EXPORT void qt_gui_c_QPageLayout_setPageSize_pageSize(QPageLayout* this_ptr, const QPageSize* pageSize);
QT_GUI_C_EXPORT void qt_gui_c_QPageLayout_setPageSize_pageSize_minMargins(QPageLayout* this_ptr, const QPageSize* pageSize, const QMarginsF* minMargins);
QT_GUI_C_EXPORT bool qt_gui_c_QPageLayout_setRightMargin(QPageLayout* this_ptr, double rightMargin);
QT_GUI_C_EXPORT bool qt_gui_c_QPageLayout_setTopMargin(QPageLayout* this_ptr, double topMargin);
QT_GUI_C_EXPORT void qt_gui_c_QPageLayout_setUnits(QPageLayout* this_ptr, QPageLayout::Unit units);
QT_GUI_C_EXPORT void qt_gui_c_QPageLayout_swap(QPageLayout* this_ptr, QPageLayout* other);
QT_GUI_C_EXPORT QPageLayout::Unit qt_gui_c_QPageLayout_units(const QPageLayout* this_ptr);

} // extern "C"

#endif // QT_GUI_C_QPAGELAYOUT_H
