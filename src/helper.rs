use svg::node::{Node, Value};

pub fn make_element<T>(mut elemenet: T, values: &Vec<(String, Value)>) -> T
where
  T: Node,
{
  for (key, value) in values.iter() {
    elemenet.assign(key, value.to_owned());
  }
  elemenet
}

pub fn make_pair<K, V>(prop: (K, V)) -> (String, Value)
where
  K: Into<String>,
  V: Into<Value>,
{
  (prop.0.into(), prop.1.into())
}
