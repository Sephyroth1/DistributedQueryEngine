#include <stdexcept>
#include <string>
#include <variant>

enum class ValueType { INT, FLOAT, STRING, BOOL };

struct Value {
  ValueType type;
  std::variant<int, float, std::string, bool> value;

  Value(ValueType type, std::variant<int, float, std::string, bool> value)
      : type(type), value(value) {}

  std::variant<int, float, std::string, bool> get_value() const {
    if (type == ValueType::INT)
      return std::get<int>(value);
    else if (type == ValueType::FLOAT)
      return std::get<float>(value);
    else if (type == ValueType::STRING)
      return std::get<std::string>(value);
    else if (type == ValueType::BOOL)
      return std::get<bool>(value);

    throw std::runtime_error("Invalid value type");
  }

  std::string get_value_as_string() const {
    if (type == ValueType::INT)
      return std::to_string(std::get<int>(value));
    else if (type == ValueType::FLOAT)
      return std::to_string(std::get<float>(value));
    else if (type == ValueType::STRING)
      return std::get<std::string>(value);
    else if (type == ValueType::BOOL)
      return std::get<bool>(value) ? "true" : "false";

    throw std::runtime_error("Invalid value type");
  }
};
