#include "base/main.cpp"
#include "base/table.hpp"
#include "base/types.hpp"
#include <iostream>

int main() {
  std::unique_ptr<Value> vptr = parseJson("{\"key\": 123}");
  if (vptr) {
    std::cout << vptr->toString() << std::endl;
  } else {
    std::cout << "This is null" << std::endl;
  }
}
