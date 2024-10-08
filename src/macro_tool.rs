#[macro_export]
macro_rules! create_serde_string_length_checker {
    ($func_name:ident, $min_length:expr, $max_length:expr) => {
        pub fn $func_name<'de, D>(deserializer: D) -> Result<String, D::Error>
        where
            D: Deserializer<'de>,
        {
            struct StringVisitor {
                max_length: usize,
                min_length: usize,
            }

            impl<'de> de::Visitor<'de> for StringVisitor {
                type Value = String;

                fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                    formatter.write_str(&format!("a string with at most {} - {} characters", self.min_length, self.max_length))
                }

                fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
                where
                    E: de::Error,
                {
                    if 0 != self.min_length {
                        if value.chars().count() < self.min_length {
                           return Err(E::invalid_value(Unexpected::Str(&value), &self));
                        }
                    }

                    if value.chars().count() <= self.max_length {
                        Ok(value.to_owned())
                    } else {
                        Err(E::invalid_value(Unexpected::Str(value), &self))
                    }
                }

                fn visit_string<E>(self, value: String) -> Result<Self::Value, E>
                where
                    E: de::Error,
                {
                    if 0 != self.min_length {
                        if value.chars().count() < self.min_length {
                            return Err(E::invalid_value(Unexpected::Str(&value), &self));
                        }
                    }

                    if value.chars().count() <= self.max_length {
                        Ok(value)
                    } else {
                        Err(E::invalid_value(Unexpected::Str(&value), &self))
                    }
                }
            }

            deserializer.deserialize_string(StringVisitor { max_length: $max_length, min_length: $min_length })
        }
    };
}
