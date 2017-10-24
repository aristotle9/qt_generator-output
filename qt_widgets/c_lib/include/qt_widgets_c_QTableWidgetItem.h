#ifndef QT_WIDGETS_C_QTABLEWIDGETITEM_H
#define QT_WIDGETS_C_QTABLEWIDGETITEM_H

#include "qt_widgets_c_global.h"

extern "C" {

QT_WIDGETS_C_EXPORT void qt_widgets_c_QTableWidgetItem_backgroundColor_to_output(const QTableWidgetItem* this_ptr, QColor* output);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QTableWidgetItem_background_to_output(const QTableWidgetItem* this_ptr, QBrush* output);
QT_WIDGETS_C_EXPORT QTableWidgetItem* qt_widgets_c_QTableWidgetItem_clone(const QTableWidgetItem* this_ptr);
QT_WIDGETS_C_EXPORT int qt_widgets_c_QTableWidgetItem_column(const QTableWidgetItem* this_ptr);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QTableWidgetItem_data_to_output(const QTableWidgetItem* this_ptr, int role, QVariant* output);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QTableWidgetItem_delete(QTableWidgetItem* this_ptr);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QTableWidgetItem_font_to_output(const QTableWidgetItem* this_ptr, QFont* output);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QTableWidgetItem_foreground_to_output(const QTableWidgetItem* this_ptr, QBrush* output);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QTableWidgetItem_icon_to_output(const QTableWidgetItem* this_ptr, QIcon* output);
QT_WIDGETS_C_EXPORT bool qt_widgets_c_QTableWidgetItem_isSelected(const QTableWidgetItem* this_ptr);
QT_WIDGETS_C_EXPORT QTableWidgetItem* qt_widgets_c_QTableWidgetItem_new_icon_text(const QIcon* icon, const QString* text);
QT_WIDGETS_C_EXPORT QTableWidgetItem* qt_widgets_c_QTableWidgetItem_new_icon_text_type(const QIcon* icon, const QString* text, int type);
QT_WIDGETS_C_EXPORT QTableWidgetItem* qt_widgets_c_QTableWidgetItem_new_no_args();
QT_WIDGETS_C_EXPORT QTableWidgetItem* qt_widgets_c_QTableWidgetItem_new_other(const QTableWidgetItem* other);
QT_WIDGETS_C_EXPORT QTableWidgetItem* qt_widgets_c_QTableWidgetItem_new_text(const QString* text);
QT_WIDGETS_C_EXPORT QTableWidgetItem* qt_widgets_c_QTableWidgetItem_new_text_type(const QString* text, int type);
QT_WIDGETS_C_EXPORT QTableWidgetItem* qt_widgets_c_QTableWidgetItem_new_type(int type);
QT_WIDGETS_C_EXPORT QTableWidgetItem* qt_widgets_c_QTableWidgetItem_operator_assign(QTableWidgetItem* this_ptr, const QTableWidgetItem* other);
QT_WIDGETS_C_EXPORT bool qt_widgets_c_QTableWidgetItem_operator_lt(const QTableWidgetItem* this_ptr, const QTableWidgetItem* other);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QTableWidgetItem_read(QTableWidgetItem* this_ptr, QDataStream* in);
QT_WIDGETS_C_EXPORT int qt_widgets_c_QTableWidgetItem_row(const QTableWidgetItem* this_ptr);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QTableWidgetItem_setBackground(QTableWidgetItem* this_ptr, const QBrush* brush);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QTableWidgetItem_setBackgroundColor(QTableWidgetItem* this_ptr, const QColor* color);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QTableWidgetItem_setCheckState(QTableWidgetItem* this_ptr, const Qt::CheckState* state);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QTableWidgetItem_setData(QTableWidgetItem* this_ptr, int role, const QVariant* value);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QTableWidgetItem_setFont(QTableWidgetItem* this_ptr, const QFont* font);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QTableWidgetItem_setForeground(QTableWidgetItem* this_ptr, const QBrush* brush);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QTableWidgetItem_setIcon(QTableWidgetItem* this_ptr, const QIcon* icon);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QTableWidgetItem_setSelected(QTableWidgetItem* this_ptr, bool select);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QTableWidgetItem_setSizeHint(QTableWidgetItem* this_ptr, const QSize* size);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QTableWidgetItem_setStatusTip(QTableWidgetItem* this_ptr, const QString* statusTip);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QTableWidgetItem_setText(QTableWidgetItem* this_ptr, const QString* text);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QTableWidgetItem_setTextAlignment(QTableWidgetItem* this_ptr, int alignment);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QTableWidgetItem_setTextColor(QTableWidgetItem* this_ptr, const QColor* color);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QTableWidgetItem_setToolTip(QTableWidgetItem* this_ptr, const QString* toolTip);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QTableWidgetItem_setWhatsThis(QTableWidgetItem* this_ptr, const QString* whatsThis);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QTableWidgetItem_sizeHint_to_output(const QTableWidgetItem* this_ptr, QSize* output);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QTableWidgetItem_statusTip_to_output(const QTableWidgetItem* this_ptr, QString* output);
QT_WIDGETS_C_EXPORT QTableWidget* qt_widgets_c_QTableWidgetItem_tableWidget(const QTableWidgetItem* this_ptr);
QT_WIDGETS_C_EXPORT int qt_widgets_c_QTableWidgetItem_textAlignment(const QTableWidgetItem* this_ptr);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QTableWidgetItem_textColor_to_output(const QTableWidgetItem* this_ptr, QColor* output);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QTableWidgetItem_text_to_output(const QTableWidgetItem* this_ptr, QString* output);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QTableWidgetItem_toolTip_to_output(const QTableWidgetItem* this_ptr, QString* output);
QT_WIDGETS_C_EXPORT int qt_widgets_c_QTableWidgetItem_type(const QTableWidgetItem* this_ptr);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QTableWidgetItem_whatsThis_to_output(const QTableWidgetItem* this_ptr, QString* output);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QTableWidgetItem_write(const QTableWidgetItem* this_ptr, QDataStream* out);

} // extern "C"

#endif // QT_WIDGETS_C_QTABLEWIDGETITEM_H
