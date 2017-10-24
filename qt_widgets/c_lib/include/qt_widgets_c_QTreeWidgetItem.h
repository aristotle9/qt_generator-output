#ifndef QT_WIDGETS_C_QTREEWIDGETITEM_H
#define QT_WIDGETS_C_QTREEWIDGETITEM_H

#include "qt_widgets_c_global.h"

extern "C" {

QT_WIDGETS_C_EXPORT void qt_widgets_c_QTreeWidgetItem_addChild(QTreeWidgetItem* this_ptr, QTreeWidgetItem* child);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QTreeWidgetItem_addChildren(QTreeWidgetItem* this_ptr, const QList< QTreeWidgetItem* >* children);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QTreeWidgetItem_backgroundColor_to_output(const QTreeWidgetItem* this_ptr, int column, QColor* output);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QTreeWidgetItem_background_to_output(const QTreeWidgetItem* this_ptr, int column, QBrush* output);
QT_WIDGETS_C_EXPORT QTreeWidgetItem* qt_widgets_c_QTreeWidgetItem_child(const QTreeWidgetItem* this_ptr, int index);
QT_WIDGETS_C_EXPORT int qt_widgets_c_QTreeWidgetItem_childCount(const QTreeWidgetItem* this_ptr);
QT_WIDGETS_C_EXPORT QTreeWidgetItem* qt_widgets_c_QTreeWidgetItem_clone(const QTreeWidgetItem* this_ptr);
QT_WIDGETS_C_EXPORT int qt_widgets_c_QTreeWidgetItem_columnCount(const QTreeWidgetItem* this_ptr);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QTreeWidgetItem_data_to_output(const QTreeWidgetItem* this_ptr, int column, int role, QVariant* output);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QTreeWidgetItem_delete(QTreeWidgetItem* this_ptr);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QTreeWidgetItem_font_to_output(const QTreeWidgetItem* this_ptr, int column, QFont* output);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QTreeWidgetItem_foreground_to_output(const QTreeWidgetItem* this_ptr, int column, QBrush* output);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QTreeWidgetItem_icon_to_output(const QTreeWidgetItem* this_ptr, int column, QIcon* output);
QT_WIDGETS_C_EXPORT int qt_widgets_c_QTreeWidgetItem_indexOfChild(const QTreeWidgetItem* this_ptr, QTreeWidgetItem* child);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QTreeWidgetItem_insertChild(QTreeWidgetItem* this_ptr, int index, QTreeWidgetItem* child);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QTreeWidgetItem_insertChildren(QTreeWidgetItem* this_ptr, int index, const QList< QTreeWidgetItem* >* children);
QT_WIDGETS_C_EXPORT bool qt_widgets_c_QTreeWidgetItem_isDisabled(const QTreeWidgetItem* this_ptr);
QT_WIDGETS_C_EXPORT bool qt_widgets_c_QTreeWidgetItem_isExpanded(const QTreeWidgetItem* this_ptr);
QT_WIDGETS_C_EXPORT bool qt_widgets_c_QTreeWidgetItem_isFirstColumnSpanned(const QTreeWidgetItem* this_ptr);
QT_WIDGETS_C_EXPORT bool qt_widgets_c_QTreeWidgetItem_isHidden(const QTreeWidgetItem* this_ptr);
QT_WIDGETS_C_EXPORT bool qt_widgets_c_QTreeWidgetItem_isSelected(const QTreeWidgetItem* this_ptr);
QT_WIDGETS_C_EXPORT QTreeWidgetItem* qt_widgets_c_QTreeWidgetItem_new_no_args();
QT_WIDGETS_C_EXPORT QTreeWidgetItem* qt_widgets_c_QTreeWidgetItem_new_other(const QTreeWidgetItem* other);
QT_WIDGETS_C_EXPORT QTreeWidgetItem* qt_widgets_c_QTreeWidgetItem_new_parent(QTreeWidgetItem* parent);
QT_WIDGETS_C_EXPORT QTreeWidgetItem* qt_widgets_c_QTreeWidgetItem_new_parent_after(QTreeWidgetItem* parent, QTreeWidgetItem* after);
QT_WIDGETS_C_EXPORT QTreeWidgetItem* qt_widgets_c_QTreeWidgetItem_new_parent_after_type(QTreeWidgetItem* parent, QTreeWidgetItem* after, int type);
QT_WIDGETS_C_EXPORT QTreeWidgetItem* qt_widgets_c_QTreeWidgetItem_new_parent_strings(QTreeWidgetItem* parent, const QStringList* strings);
QT_WIDGETS_C_EXPORT QTreeWidgetItem* qt_widgets_c_QTreeWidgetItem_new_parent_strings_type(QTreeWidgetItem* parent, const QStringList* strings, int type);
QT_WIDGETS_C_EXPORT QTreeWidgetItem* qt_widgets_c_QTreeWidgetItem_new_parent_type(QTreeWidgetItem* parent, int type);
QT_WIDGETS_C_EXPORT QTreeWidgetItem* qt_widgets_c_QTreeWidgetItem_new_strings(const QStringList* strings);
QT_WIDGETS_C_EXPORT QTreeWidgetItem* qt_widgets_c_QTreeWidgetItem_new_strings_type(const QStringList* strings, int type);
QT_WIDGETS_C_EXPORT QTreeWidgetItem* qt_widgets_c_QTreeWidgetItem_new_type(int type);
QT_WIDGETS_C_EXPORT QTreeWidgetItem* qt_widgets_c_QTreeWidgetItem_new_view(QTreeWidget* view);
QT_WIDGETS_C_EXPORT QTreeWidgetItem* qt_widgets_c_QTreeWidgetItem_new_view_after(QTreeWidget* view, QTreeWidgetItem* after);
QT_WIDGETS_C_EXPORT QTreeWidgetItem* qt_widgets_c_QTreeWidgetItem_new_view_after_type(QTreeWidget* view, QTreeWidgetItem* after, int type);
QT_WIDGETS_C_EXPORT QTreeWidgetItem* qt_widgets_c_QTreeWidgetItem_new_view_strings(QTreeWidget* view, const QStringList* strings);
QT_WIDGETS_C_EXPORT QTreeWidgetItem* qt_widgets_c_QTreeWidgetItem_new_view_strings_type(QTreeWidget* view, const QStringList* strings, int type);
QT_WIDGETS_C_EXPORT QTreeWidgetItem* qt_widgets_c_QTreeWidgetItem_new_view_type(QTreeWidget* view, int type);
QT_WIDGETS_C_EXPORT QTreeWidgetItem* qt_widgets_c_QTreeWidgetItem_operator_assign(QTreeWidgetItem* this_ptr, const QTreeWidgetItem* other);
QT_WIDGETS_C_EXPORT bool qt_widgets_c_QTreeWidgetItem_operator_lt(const QTreeWidgetItem* this_ptr, const QTreeWidgetItem* other);
QT_WIDGETS_C_EXPORT QTreeWidgetItem* qt_widgets_c_QTreeWidgetItem_parent(const QTreeWidgetItem* this_ptr);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QTreeWidgetItem_read(QTreeWidgetItem* this_ptr, QDataStream* in);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QTreeWidgetItem_removeChild(QTreeWidgetItem* this_ptr, QTreeWidgetItem* child);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QTreeWidgetItem_setBackground(QTreeWidgetItem* this_ptr, int column, const QBrush* brush);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QTreeWidgetItem_setBackgroundColor(QTreeWidgetItem* this_ptr, int column, const QColor* color);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QTreeWidgetItem_setCheckState(QTreeWidgetItem* this_ptr, int column, const Qt::CheckState* state);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QTreeWidgetItem_setChildIndicatorPolicy(QTreeWidgetItem* this_ptr, const QTreeWidgetItem::ChildIndicatorPolicy* policy);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QTreeWidgetItem_setData(QTreeWidgetItem* this_ptr, int column, int role, const QVariant* value);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QTreeWidgetItem_setDisabled(QTreeWidgetItem* this_ptr, bool disabled);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QTreeWidgetItem_setExpanded(QTreeWidgetItem* this_ptr, bool expand);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QTreeWidgetItem_setFirstColumnSpanned(QTreeWidgetItem* this_ptr, bool span);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QTreeWidgetItem_setFont(QTreeWidgetItem* this_ptr, int column, const QFont* font);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QTreeWidgetItem_setForeground(QTreeWidgetItem* this_ptr, int column, const QBrush* brush);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QTreeWidgetItem_setHidden(QTreeWidgetItem* this_ptr, bool hide);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QTreeWidgetItem_setIcon(QTreeWidgetItem* this_ptr, int column, const QIcon* icon);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QTreeWidgetItem_setSelected(QTreeWidgetItem* this_ptr, bool select);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QTreeWidgetItem_setSizeHint(QTreeWidgetItem* this_ptr, int column, const QSize* size);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QTreeWidgetItem_setStatusTip(QTreeWidgetItem* this_ptr, int column, const QString* statusTip);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QTreeWidgetItem_setText(QTreeWidgetItem* this_ptr, int column, const QString* text);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QTreeWidgetItem_setTextAlignment(QTreeWidgetItem* this_ptr, int column, int alignment);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QTreeWidgetItem_setTextColor(QTreeWidgetItem* this_ptr, int column, const QColor* color);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QTreeWidgetItem_setToolTip(QTreeWidgetItem* this_ptr, int column, const QString* toolTip);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QTreeWidgetItem_setWhatsThis(QTreeWidgetItem* this_ptr, int column, const QString* whatsThis);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QTreeWidgetItem_sizeHint_to_output(const QTreeWidgetItem* this_ptr, int column, QSize* output);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QTreeWidgetItem_sortChildren(QTreeWidgetItem* this_ptr, int column, const Qt::SortOrder* order);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QTreeWidgetItem_statusTip_to_output(const QTreeWidgetItem* this_ptr, int column, QString* output);
QT_WIDGETS_C_EXPORT QTreeWidgetItem* qt_widgets_c_QTreeWidgetItem_takeChild(QTreeWidgetItem* this_ptr, int index);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QTreeWidgetItem_takeChildren_to_output(QTreeWidgetItem* this_ptr, QList< QTreeWidgetItem* >* output);
QT_WIDGETS_C_EXPORT int qt_widgets_c_QTreeWidgetItem_textAlignment(const QTreeWidgetItem* this_ptr, int column);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QTreeWidgetItem_textColor_to_output(const QTreeWidgetItem* this_ptr, int column, QColor* output);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QTreeWidgetItem_text_to_output(const QTreeWidgetItem* this_ptr, int column, QString* output);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QTreeWidgetItem_toolTip_to_output(const QTreeWidgetItem* this_ptr, int column, QString* output);
QT_WIDGETS_C_EXPORT QTreeWidget* qt_widgets_c_QTreeWidgetItem_treeWidget(const QTreeWidgetItem* this_ptr);
QT_WIDGETS_C_EXPORT int qt_widgets_c_QTreeWidgetItem_type(const QTreeWidgetItem* this_ptr);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QTreeWidgetItem_whatsThis_to_output(const QTreeWidgetItem* this_ptr, int column, QString* output);
QT_WIDGETS_C_EXPORT void qt_widgets_c_QTreeWidgetItem_write(const QTreeWidgetItem* this_ptr, QDataStream* out);

} // extern "C"

#endif // QT_WIDGETS_C_QTREEWIDGETITEM_H
