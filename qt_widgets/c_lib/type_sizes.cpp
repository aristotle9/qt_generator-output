#include <QtWidgets>
#include <iostream>

int main() {
  std::cout << "pub const QT_WIDGETS_SIZE_POLICY_SIZE_POLICY: usize = " << sizeof(QSizePolicy) << ";\n";
  std::cout << "pub const QT_WIDGETS_STYLE_OPTION_FOCUS_RECT_STYLE_OPTION_FOCUS_RECT: usize = " << sizeof(QStyleOptionFocusRect) << ";\n";
  std::cout << "pub const QT_WIDGETS_STYLE_OPTION_TAB_BAR_BASE_STYLE_OPTION_TAB_BAR_BASE: usize = " << sizeof(QStyleOptionTabBarBase) << ";\n";
  std::cout << "pub const QT_WIDGETS_STYLE_OPTION_TOOL_BOX_STYLE_OPTION_TOOL_BOX: usize = " << sizeof(QStyleOptionToolBox) << ";\n";
  std::cout << "pub const QT_WIDGETS_STYLE_OPTION_TITLE_BAR_STYLE_OPTION_TITLE_BAR: usize = " << sizeof(QStyleOptionTitleBar) << ";\n";
  std::cout << "pub const QT_WIDGETS_STYLE_OPTION_SIZE_GRIP_STYLE_OPTION_SIZE_GRIP: usize = " << sizeof(QStyleOptionSizeGrip) << ";\n";
  std::cout << "pub const QT_WIDGETS_COLORMAP_COLORMAP: usize = " << sizeof(QColormap) << ";\n";
  std::cout << "pub const QT_WIDGETS_FORM_LAYOUT_TAKE_ROW_RESULT: usize = " << sizeof(QFormLayout::TakeRowResult) << ";\n";
  std::cout << "pub const QT_WIDGETS_TEXT_EDIT_EXTRA_SELECTION: usize = " << sizeof(QTextEdit::ExtraSelection) << ";\n";
  std::cout << "pub const QT_WIDGETS_STYLE_PAINTER_STYLE_PAINTER: usize = " << sizeof(QStylePainter) << ";\n";
  std::cout << "pub const QT_WIDGETS_TABLE_WIDGET_SELECTION_RANGE_TABLE_WIDGET_SELECTION_RANGE: usize = " << sizeof(QTableWidgetSelectionRange) << ";\n";
  std::cout << "pub const QT_WIDGETS_TREE_WIDGET_ITEM_ITERATOR_TREE_WIDGET_ITEM_ITERATOR: usize = " << sizeof(QTreeWidgetItemIterator) << ";\n";
  std::cout << "pub const QT_WIDGETS_LIST_LIST_ACTION_MUT_PTR: usize = " << sizeof(QList< QAction* >) << ";\n";
  std::cout << "pub const QT_WIDGETS_LIST_LIST_WIDGET_MUT_PTR: usize = " << sizeof(QList< QWidget* >) << ";\n";
  std::cout << "pub const QT_WIDGETS_LIST_LIST_GRAPHICS_WIDGET_MUT_PTR: usize = " << sizeof(QList< QGraphicsWidget* >) << ";\n";
  std::cout << "pub const QT_WIDGETS_LIST_LIST_ABSTRACT_BUTTON_MUT_PTR: usize = " << sizeof(QList< QAbstractButton* >) << ";\n";
  std::cout << "pub const QT_WIDGETS_LIST_LIST_GESTURE_MUT_PTR: usize = " << sizeof(QList< QGesture* >) << ";\n";
  std::cout << "pub const QT_WIDGETS_LIST_LIST_GRAPHICS_ITEM_MUT_PTR: usize = " << sizeof(QList< QGraphicsItem* >) << ";\n";
  std::cout << "pub const QT_WIDGETS_LIST_LIST_GRAPHICS_TRANSFORM_MUT_PTR: usize = " << sizeof(QList< QGraphicsTransform* >) << ";\n";
  std::cout << "pub const QT_WIDGETS_LIST_LIST_GRAPHICS_VIEW_MUT_PTR: usize = " << sizeof(QList< QGraphicsView* >) << ";\n";
  std::cout << "pub const QT_WIDGETS_LIST_LIST_QT_CORE_RECT_F: usize = " << sizeof(QList< QRectF >) << ";\n";
  std::cout << "pub const QT_WIDGETS_LIST_LIST_LIST_WIDGET_ITEM_MUT_PTR: usize = " << sizeof(QList< QListWidgetItem* >) << ";\n";
  std::cout << "pub const QT_WIDGETS_LIST_LIST_DOCK_WIDGET_MUT_PTR: usize = " << sizeof(QList< QDockWidget* >) << ";\n";
  std::cout << "pub const QT_WIDGETS_LIST_LIST_MDI_SUB_WINDOW_MUT_PTR: usize = " << sizeof(QList< QMdiSubWindow* >) << ";\n";
  std::cout << "pub const QT_WIDGETS_LIST_LIST_TEXT_EDIT_EXTRA_SELECTION: usize = " << sizeof(QList< QTextEdit::ExtraSelection >) << ";\n";
  std::cout << "pub const QT_WIDGETS_LIST_LIST_SCROLLER_MUT_PTR: usize = " << sizeof(QList< QScroller* >) << ";\n";
  std::cout << "pub const QT_WIDGETS_LIST_LIST_TABLE_WIDGET_SELECTION_RANGE: usize = " << sizeof(QList< QTableWidgetSelectionRange >) << ";\n";
  std::cout << "pub const QT_WIDGETS_LIST_LIST_TABLE_WIDGET_ITEM_MUT_PTR: usize = " << sizeof(QList< QTableWidgetItem* >) << ";\n";
  std::cout << "pub const QT_WIDGETS_LIST_LIST_TREE_WIDGET_ITEM_MUT_PTR: usize = " << sizeof(QList< QTreeWidgetItem* >) << ";\n";
  std::cout << "pub const QT_WIDGETS_LIST_LIST_UNDO_STACK_MUT_PTR: usize = " << sizeof(QList< QUndoStack* >) << ";\n";
  std::cout << "pub const QT_WIDGETS_LIST_LIST_WIZARD_WIZARD_BUTTON: usize = " << sizeof(QList< QWizard::WizardButton >) << ";\n";
  std::cout << "pub const QT_WIDGETS_MAP_MAP_QT_CORE_DATE_QT_GUI_TEXT_CHAR_FORMAT: usize = " << sizeof(QMap< QDate, QTextCharFormat >) << ";\n";
  std::cout << "pub const QT_WIDGETS_MAP_MAP_QT_CORE_QT_GESTURE_TYPE_BOOL: usize = " << sizeof(QMap< Qt::GestureType, bool >) << ";\n";
  std::cout << "pub const QT_WIDGETS_MAP_MAP_QT_CORE_QT_GESTURE_TYPE_WIDGET_MUT_PTR: usize = " << sizeof(QMap< Qt::GestureType, QWidget* >) << ";\n";
  std::cout << "pub const QT_WIDGETS_VECTOR_VECTOR_QT_GUI_COLOR: usize = " << sizeof(QVector< QColor >) << ";\n";
  std::cout << "pub const QT_WIDGETS_VECTOR_VECTOR_C_VOID_MUT_PTR: usize = " << sizeof(QVector< void* >) << ";\n";
  std::cout << "pub const QT_WIDGETS_PAIR_PAIR_C_DOUBLE_QT_CORE_POINT_F: usize = " << sizeof(QPair< double, QPointF >) << ";\n";
  std::cout << "pub const QT_WIDGETS_PAIR_PAIR_C_DOUBLE_C_DOUBLE: usize = " << sizeof(QPair< double, double >) << ";\n";
  std::cout << "pub const QT_WIDGETS_HASH_HASH_C_INT_ITEM_EDITOR_CREATOR_BASE_MUT_PTR: usize = " << sizeof(QHash< int, QItemEditorCreatorBase* >) << ";\n";
  std::cout << "pub const QT_WIDGETS_LIST_LIST_PAIR_PAIR_C_DOUBLE_QT_CORE_POINT_F: usize = " << sizeof(QList< QPair< double, QPointF > >) << ";\n";
  std::cout << "pub const QT_WIDGETS_LIST_LIST_PAIR_PAIR_C_DOUBLE_C_DOUBLE: usize = " << sizeof(QList< QPair< double, double > >) << ";\n";
}
