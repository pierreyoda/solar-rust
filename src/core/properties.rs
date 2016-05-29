use std::fmt;
use std::collections::HashMap;

use self::ObjectPropertyValue::*;

/// The different types of values that can be stored inside an 'ObjectProperty'.
#[derive(Clone, Debug, PartialEq)]
pub enum ObjectPropertyValue {
    Integer(u16),
    Float(f64),
    Text(String),
}

impl ObjectPropertyValue {
    pub fn text(text: &str) -> ObjectPropertyValue {
        ObjectPropertyValue::Text(text.to_string())
    }
}

impl fmt::Display for ObjectPropertyValue {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use self::ObjectPropertyValue::*;

        let precision = f.precision().unwrap_or(3);
        match self {
            &Integer(v) => write!(f, "{}", v),
            &Float(v) => write!(f, "{:.*}", precision, v),
            &Text(ref v) => write!(f, "{}", v),
        }
    }
}

/// A 'GameObject' property.
#[derive(Clone, Debug, PartialEq)]
pub struct ObjectProperty {
    value: ObjectPropertyValue,
    description: String,
    mutable: bool,
    /// An optional display name that if specified will be used in-game instead
    /// of the register's associated key.
    display_name: Option<String>,
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
                                   display_name: None,
                               });
    }

    pub fn add_property<S: Into<String>>(&mut self, key: S, value: ObjectPropertyValue, desc: S) {
        self.properties.insert(key.into(),
                               ObjectProperty {
                                   value: value,
                                   description: desc.into(),
                                   mutable: true,
                                   display_name: None,
                               });
    }

    pub fn get_description<S: Into<String>>(&self, key: S) -> Option<&String> {
        self.properties.get(&key.into()).map(|property| &property.description)
    }

    pub fn set_description<S: Into<String>>(&mut self, key: S, desc: S) {
        self.properties.get_mut(&key.into()).map(|property| property.description = desc.into());
    }

    /// Return None if there is no property associated with the given key or if
    /// the property exists but does not specify a display name.
    /// Return Some(property_display_name) otherwise.
    pub fn get_display_name<S: Into<String>>(&self, key: S) -> Option<&String> {
        match self.properties.get(&key.into()) {
            None => None,
            Some(property) => {
                match property.display_name {
                    None => None,
                    Some(ref string) => Some(string),
                }
            }
        }
    }

    pub fn set_display_name<S: Into<String>>(&mut self, key: S, name: S) {
        self.properties
            .get_mut(&key.into())
            .map(|property| property.display_name = Some(name.into()));
    }

    register_get!(get_int, Integer, u16);
    register_get!(get_float, Float, f64);
    register_get!(get_text, Text, String);

    register_get_mut!(get_int_mut, Integer, u16);
    register_get_mut!(get_float_mut, Float, f64);
    register_get_mut!(get_text_mut, Text, String);
}

impl fmt::Display for ObjectRegister {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        try!(write!(f, "Object register :\n"));
        for (key, property) in &self.properties {
            let r = write!(f,
                           "- {name} : {value}\n\tmutable = {mutable}\n\t{description}\n",
                           name = property.display_name.as_ref().unwrap_or(key),
                           value = property.value,
                           mutable = property.mutable,
                           description = property.description);
            try!(r);
        }
        write!(f, "{}", "\n")
    }
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

        assert_eq!(register.get_float("test_text"), None);
        let text = "Hello World".to_string();
        register.get_text_mut("test_text").map(|t| *t = text.clone());
        assert_eq!(register.get_text("test_text"), Some(&text));
        assert_eq!(register.get_int("test_text"), None);
        assert_eq!(register.get_float("test_text"), None);

        assert_eq!(register.get_display_name("property_does_not_exist"), None);
        assert_eq!(register.get_display_name("test_int"), None);
        let display_name = "Test integer".to_string();
        register.set_display_name("test_int", &display_name[..]);
        assert_eq!(register.get_display_name("test_int"), Some(&display_name));
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
