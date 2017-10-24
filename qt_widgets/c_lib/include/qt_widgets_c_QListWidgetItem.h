#ifndef QT_WIDGETS_C_QLISTWIDGETITEM_H
#define QT_WIDGETS_C_QLISTWIDGETITEM_H

#include "qt_widgets_c_global.h"

extern "C" {

QT_WIDGETS_C_EXPORT void qt_widgets_c_QListWidgetItem_backgroundColor_to_output(const QListWidgetItem* this_ptr, QColor* output);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QListWidgetItem_background_to_output(const QListWidgetItem* this_ptr, QBrush* output);
QT_WIDGETS_C_EXPORT QListWidgetItem* qt_widgets_c_QListWidgetItem_clone(const QListWidgetItem* this_ptr);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QListWidgetItem_data_to_output(const QListWidgetItem* this_ptr, int role, QVariant* output);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QListWidgetItem_delete(QListWidgetItem* this_ptr);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QListWidgetItem_font_to_output(const QListWidgetItem* this_ptr, QFont* output);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QListWidgetItem_foreground_to_output(const QListWidgetItem* this_ptr, QBrush* output);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QListWidgetItem_icon_to_output(const QListWidgetItem* this_ptr, QIcon* output);
QT_WIDGETS_C_EXPORT bool qt_widgets_c_QListWidgetItem_isHidden(const QListWidgetItem* this_ptr);
QT_WIDGETS_C_EXPORT bool qt_widgets_c_QListWidgetItem_isSelected(const QListWidgetItem* this_ptr);
QT_WIDGETS_C_EXPORT QListWidget* qt_widgets_c_QListWidgetItem_listWidget(const QListWidgetItem* this_ptr);
QT_WIDGETS_C_EXPORT QListWidgetItem* qt_widgets_c_QListWidgetItem_new_icon_text(const QIcon* icon, const QString* text);
QT_WIDGETS_C_EXPORT QListWidgetItem* qt_widgets_c_QListWidgetItem_new_icon_text_view(const QIcon* icon, const QString* text, QListWidget* view);
QT_WIDGETS_C_EXPORT QListWidgetItem* qt_widgets_c_QListWidgetItem_new_icon_text_view_type(const QIcon* icon, const QString* text, QListWidget* view, int type);
QT_WIDGETS_C_EXPORT QListWidgetItem* qt_widgets_c_QListWidgetItem_new_no_args();
QT_WIDGETS_C_EXPORT QListWidgetItem* qt_widgets_c_QListWidgetItem_new_other(const QListWidgetItem* other);
QT_WIDGETS_C_EXPORT QListWidgetItem* qt_widgets_c_QListWidgetItem_new_text(const QString* text);
QT_WIDGETS_C_EXPORT QListWidgetItem* qt_widgets_c_QListWidgetItem_new_text_view(const QString* text, QListWidget* view);
QT_WIDGETS_C_EXPORT QListWidgetItem* qt_widgets_c_QListWidgetItem_new_text_view_type(const QString* text, QListWidget* view, int type);
QT_WIDGETS_C_EXPORT QListWidgetItem* qt_widgets_c_QListWidgetItem_new_view(QListWidget* view);
QT_WIDGETS_C_EXPORT QListWidgetItem* qt_widgets_c_QListWidgetItem_new_view_type(QListWidget* view, int type);
QT_WIDGETS_C_EXPORT QListWidgetItem* qt_widgets_c_QListWidgetItem_operator_assign(QListWidgetItem* this_ptr, const QListWidgetItem* other);
QT_WIDGETS_C_EXPORT bool qt_widgets_c_QListWidgetItem_operator_lt(const QListWidgetItem* this_ptr, const QListWidgetItem* other);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QListWidgetItem_read(QListWidgetItem* this_ptr, QDataStream* in);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QListWidgetItem_setBackground(QListWidgetItem* this_ptr, const QBrush* brush);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QListWidgetItem_setBackgroundColor(QListWidgetItem* this_ptr, const QColor* color);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QListWidgetItem_setCheckState(QListWidgetItem* this_ptr, const Qt::CheckState* state);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QListWidgetItem_setData(QListWidgetItem* this_ptr, int role, const QVariant* value);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QListWidgetItem_setFont(QListWidgetItem* this_ptr, const QFont* font);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QListWidgetItem_setForeground(QListWidgetItem* this_ptr, const QBrush* brush);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QListWidgetItem_setHidden(QListWidgetItem* this_ptr, bool hide);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QListWidgetItem_setIcon(QListWidgetItem* this_ptr, const QIcon* icon);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QListWidgetItem_setSelected(QListWidgetItem* this_ptr, bool select);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QListWidgetItem_setSizeHint(QListWidgetItem* this_ptr, const QSize* size);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QListWidgetItem_setStatusTip(QListWidgetItem* this_ptr, const QString* statusTip);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QListWidgetItem_setText(QListWidgetItem* this_ptr, const QString* text);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QListWidgetItem_setTextAlignment(QListWidgetItem* this_ptr, int alignment);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QListWidgetItem_setTextColor(QListWidgetItem* this_ptr, const QColor* color);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QListWidgetItem_setToolTip(QListWidgetItem* this_ptr, const QString* toolTip);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QListWidgetItem_setWhatsThis(QListWidgetItem* this_ptr, const QString* whatsThis);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QListWidgetItem_sizeHint_to_output(const QListWidgetItem* this_ptr, QSize* output);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QListWidgetItem_statusTip_to_output(const QListWidgetItem* this_ptr, QString* output);
QT_WIDGETS_C_EXPORT int qt_widgets_c_QListWidgetItem_textAlignment(const QListWidgetItem* this_ptr);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QListWidgetItem_textColor_to_output(const QListWidgetItem* this_ptr, QColor* output);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QListWidgetItem_text_to_output(const QListWidgetItem* this_ptr, QString* output);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QListWidgetItem_toolTip_to_output(const QListWidgetItem* this_ptr, QString* output);
QT_WIDGETS_C_EXPORT int qt_widgets_c_QListWidgetItem_type(const QListWidgetItem* this_ptr);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QListWidgetItem_whatsThis_to_output(const QListWidgetItem* this_ptr, QString* output);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QListWidgetItem_write(const QListWidgetItem* this_ptr, QDataStream* out);

} // extern "C"

#endif // QT_WIDGETS_C_QLISTWIDGETITEM_H
