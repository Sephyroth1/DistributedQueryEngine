#include "types.hpp"

#include <nlohmann/json.hpp>
using json = nlohmann::json;

std::unique_ptr<Value>
createValueFromPrimitive(const nlohmann::json &primitive_json_value) {
  if (primitive_json_value.is_number_integer()) {
    return std::make_unique<IntValue>(primitive_json_value.get<int32_t>());
  } else if (primitive_json_value.is_string()) {
    return std::make_unique<StringValue>(
        primitive_json_value.get<std::string>());
  } /*else if (primitive_json_value.is_boolean()) {
    return std::make_unique<BoolValue>(primitive_json_value.get<bool>());
  } else if (primitive_json_value.is_number_float()) {
    return std::make_unique<FloatValue>(primitive_json_value.get<double>());
    }*/
  // Handle null or other primitives if needed
  std::cerr << "Error: Unhandled JSON primitive type: "
            << primitive_json_value.dump() << std::endl;
  return nullptr;
}

std::unique_ptr<Value> parseJson(const std::string &json_str,
                                 const std::string &key_to_extract = "") {
  try {
    nlohmann::json parsed_json = nlohmann::json::parse(json_str);
    nlohmann::json target_value;

    if (parsed_json.is_object()) {
      if (!key_to_extract.empty()) {
        // If a key is specified, try to extract it
        if (parsed_json.contains(key_to_extract)) {
          target_value = parsed_json[key_to_extract];
        } else {
          std::cerr << "Error in parseJson: Object does not contain key '"
                    << key_to_extract << "'. Input: " << json_str << std::endl;
          return nullptr;
        }
      } else if (parsed_json.size() == 1) {
        // If no key is specified, but it's a single-element object, take that
        // value
        target_value = parsed_json.begin().value();
        std::cerr
            << "Warning in parseJson: No key specified for single-element "
               "object. Extracted value from default key. Input: "
            << json_str << std::endl;
      } else {
        // It's an object, but no key specified and it has multiple elements.
        // This might be an error or a case where the caller should provide a
        // key.
        std::cerr << "Error in parseJson: Object with multiple keys provided, "
                     "but no specific key to extract. Input: "
                  << json_str << std::endl;
        return nullptr;
      }
    } else if (parsed_json.is_array()) {
      std::cerr << "Error in parseJson: Cannot directly convert JSON Array to "
                   "a single Value. Input: "
                << json_str << std::endl;
      return nullptr;
    } else {
      // It's a direct primitive value (not an object or array)
      target_value = parsed_json;
    }

    // Now, pass the determined target_value to the helper to create a Value
    // object
    return createValueFromPrimitive(target_value);

  } catch (const nlohmann::json::parse_error &e) {
    std::cerr << "JSON Parse Error in parseJson: " << e.what() << " at byte "
              << e.byte << ". Input: '" << json_str << "'" << std::endl;
    return nullptr;
  } catch (const nlohmann::json::exception &e) {
    std::cerr << "JSON Runtime Error in parseJson (nlohmann::json::exception): "
              << e.what() << ". Input: '" << json_str << "'" << std::endl;
    return nullptr;
  } catch (const std::exception &e) {
    std::cerr << "General Exception in parseJson: " << e.what() << ". Input: '"
              << json_str << "'" << std::endl;
    return nullptr;
  } catch (...) {
    std::cerr << "Unknown Exception in parseJson. Input: '" << json_str << "'"
              << std::endl;
    return nullptr;
  }
}
