use std::collections::HashMap;

use self::ObjectPropertyValue::*;

/// The different types of values that can be stored inside an 'ObjectProperty'.
#[derive(Clone, Debug, PartialEq)]
pub enum ObjectPropertyValue {
    Integer(u16),
    Float(f64),
}

/// A 'GameObject' property.
#[derive(Clone, Debug, PartialEq)]
pub struct ObjectProperty {
    value: ObjectPropertyValue,
    description: String,
    mutable: bool,
}

/// Each 'GameObject' owns such a register for easier storage and access of
/// multiple 'ObjectProperty' properties.
#[derive(Clone, Debug, Default)]
pub struct ObjectRegister {
    properties: HashMap<String, ObjectProperty>,
}

macro_rules! register_get {
    ($getter_name: ident, $enum_value: ident, $enum_type: ty) => (
        #[allow(dead_code)]
        pub fn $getter_name<S: Into<String>>(&self, key: S) -> Option<&$enum_type> {
            self.properties.get(&key.into()).and_then(|property|
                match property.value {
                    $enum_value(ref v) => Some(v),
                    _ => None,
                })
        }
    )
}

macro_rules! register_get_mut {
    ($getter_name: ident, $enum_value: ident, $enum_type: ty) => (
        #[allow(dead_code)]
        pub fn $getter_name<S: Into<String>>(&mut self, key: S) -> Option<&mut $enum_type> {
            let key_string = key.into();
            if !self.properties.contains_key(&key_string) {
                self.add_property(key_string.clone(),
                                  $enum_value(Default::default()),
                                  "auto-generated on demand by ObjectRegister's mutable getter".into());
            }
            self.properties.get_mut(&key_string).and_then(|property|
                if property.mutable {
                    match property.value {
                        $enum_value(ref mut v) => Some(v),
                        _ => None,
                    }
                } else {
                    None
            })
        }
    )
}

impl ObjectRegister {
    pub fn new() -> ObjectRegister {
        Default::default()
    }

    pub fn add_constant<S: Into<String>>(&mut self, key: S, value: ObjectPropertyValue, desc: S) {
        self.properties.insert(key.into(),
                               ObjectProperty {
                                   value: value,
                                   description: desc.into(),
                                   mutable: false,
                               });
    }

    pub fn add_property<S: Into<String>>(&mut self, key: S, value: ObjectPropertyValue, desc: S) {
        self.properties.insert(key.into(),
                               ObjectProperty {
                                   value: value,
                                   description: desc.into(),
                                   mutable: true,
                               });
    }

    pub fn get_description<S: Into<String>>(&self, key: S) -> Option<&String> {
        self.properties.get(&key.into()).map(|property| &property.description)
    }

    pub fn set_description<S: Into<String>>(&mut self, key: S, desc: S) {
        self.properties.get_mut(&key.into()).map(|property| property.description = desc.into());
    }

    register_get!(get_int, Integer, u16);
    register_get!(get_float, Float, f64);

    register_get_mut!(get_int_mut, Integer, u16);
    register_get_mut!(get_float_mut, Float, f64);
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::ObjectPropertyValue::*;

    #[test]
    fn test_object_register_properties() {
        let mut register: ObjectRegister = Default::default();

        assert_eq!(register.get_int("test_int"), None);
        register.add_property("test_int", Integer(12), "Just a simple test variable.");
        register.get_int_mut("test_int").map(|i| *i += 30);
        assert_eq!(register.get_int("test_int"), Some(&42));

        assert_eq!(register.get_float("test_float"), None);
        register.get_float_mut("test_float").map(|f| *f = 3.14);
        assert_eq!(register.get_float("test_float"), Some(&3.14));
        let description = "Just a simple float variable".to_string();
        register.set_description("test_float".to_string(), description.clone());
        assert_eq!(register.get_description("test_float"), Some(&description));
    }

    #[test]
    fn test_object_register_constants() {
        let mut register = ObjectRegister::new();

        assert_eq!(register.get_int("int_constant"), None);
        register.add_constant("int_constant", Integer(3), "Just a simple test constant.");
        register.get_int_mut("int_constant").map(|i| *i *= 2);
        assert_eq!(register.get_int("int_constant"), Some(&3));
    }
}
