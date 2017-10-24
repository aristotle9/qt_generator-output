#include <Qt3DInput>
#include <iostream>

int main() {
  std::cout << "pub const QT_3D_INPUT_VECTOR_VECTOR_AXIS_SETTING_MUT_PTR: usize = " << sizeof(QVector< Qt3DInput::QAxisSetting* >) << ";\n";
  std::cout << "pub const QT_3D_INPUT_VECTOR_VECTOR_ABSTRACT_ACTION_INPUT_MUT_PTR: usize = " << sizeof(QVector< Qt3DInput::QAbstractActionInput* >) << ";\n";
  std::cout << "pub const QT_3D_INPUT_VECTOR_VECTOR_ABSTRACT_AXIS_INPUT_MUT_PTR: usize = " << sizeof(QVector< Qt3DInput::QAbstractAxisInput* >) << ";\n";
  std::cout << "pub const QT_3D_INPUT_VECTOR_VECTOR_ACTION_MUT_PTR: usize = " << sizeof(QVector< Qt3DInput::QAction* >) << ";\n";
  std::cout << "pub const QT_3D_INPUT_VECTOR_VECTOR_AXIS_MUT_PTR: usize = " << sizeof(QVector< Qt3DInput::QAxis* >) << ";\n";
}
