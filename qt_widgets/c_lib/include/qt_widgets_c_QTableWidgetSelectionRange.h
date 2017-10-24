#ifndef QT_WIDGETS_C_QTABLEWIDGETSELECTIONRANGE_H
#define QT_WIDGETS_C_QTABLEWIDGETSELECTIONRANGE_H

#include "qt_widgets_c_global.h"

extern "C" {

QT_WIDGETS_C_EXPORT int qt_widgets_c_QTableWidgetSelectionRange_bottomRow(const QTableWidgetSelectionRange* this_ptr);
QT_WIDGETS_C_EXPORT int qt_widgets_c_QTableWidgetSelectionRange_columnCount(const QTableWidgetSelectionRange* this_ptr);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QTableWidgetSelectionRange_constructor_no_args(QTableWidgetSelectionRange* output);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QTableWidgetSelectionRange_constructor_other(const QTableWidgetSelectionRange* other, QTableWidgetSelectionRange* output);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QTableWidgetSelectionRange_constructor_top_left_bottom_right(int top, int left, int bottom, int right, QTableWidgetSelectionRange* output);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QTableWidgetSelectionRange_destructor(QTableWidgetSelectionRange* this_ptr);
QT_WIDGETS_C_EXPORT int qt_widgets_c_QTableWidgetSelectionRange_leftColumn(const QTableWidgetSelectionRange* this_ptr);
QT_WIDGETS_C_EXPORT int qt_widgets_c_QTableWidgetSelectionRange_rightColumn(const QTableWidgetSelectionRange* this_ptr);
QT_WIDGETS_C_EXPORT int qt_widgets_c_QTableWidgetSelectionRange_rowCount(const QTableWidgetSelectionRange* this_ptr);
QT_WIDGETS_C_EXPORT int qt_widgets_c_QTableWidgetSelectionRange_topRow(const QTableWidgetSelectionRange* this_ptr);

} // extern "C"

#endif // QT_WIDGETS_C_QTABLEWIDGETSELECTIONRANGE_H