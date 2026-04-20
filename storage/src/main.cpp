#include "base/main.cpp"
#include "base/types.hpp"
#include <iostream>

int main() {
  Value v = Value(ValueType::INT, 42);
  std::cout << v.get_value_as_string() << std::endl;
  return 0;
}
