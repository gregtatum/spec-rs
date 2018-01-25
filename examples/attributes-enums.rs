// We want to be generic over a fixed set of data types. This must expand as we add more types,
// e.g. f64, i32, i64, etc.
enum AttributeData {
    Vec1(Vec<f32>),
    Vec2(Vec<[f32; 2]>),
    Vec3(Vec<[f32; 3]>),
    Vec4(Vec<[f32; 4]>),
    Vec9(Vec<[f32; 9]>),
    Vec16(Vec<[f32; 16]>)
}

// Convert the base types into the above enum. Is there a way to make this more generic where
// f32, f64, etc is <T>? Otherwise enumerate anything we want to support.
trait ToAttributeData { fn to_attribute_data(self) -> AttributeData; }
impl ToAttributeData for Vec<f32> { fn to_attribute_data(self) -> AttributeData { AttributeData::Vec1(self) }}
impl ToAttributeData for Vec<[f32; 2]> { fn to_attribute_data(self) -> AttributeData { AttributeData::Vec2(self) }}
impl ToAttributeData for Vec<[f32; 3]> { fn to_attribute_data(self) -> AttributeData { AttributeData::Vec3(self) }}
impl ToAttributeData for Vec<[f32; 4]> { fn to_attribute_data(self) -> AttributeData { AttributeData::Vec4(self) }}
impl ToAttributeData for Vec<[f32; 9]> { fn to_attribute_data(self) -> AttributeData { AttributeData::Vec9(self) }}
impl ToAttributeData for Vec<[f32; 16]> { fn to_attribute_data(self) -> AttributeData { AttributeData::Vec16(self) }}

// Store a list of attributes for a draw command.
struct Attributes {
    pub list: Vec<(String, AttributeData)>
}

// Add a "name" and "data" tuple for each attribute.
impl Attributes {
    pub fn add<T: ToAttributeData>(&mut self, name: String, data: T) {
        &self.list.push((name, data.to_attribute_data()));
    }
}

fn main() {
    let mut attributes = Attributes { list: Vec::new() };
    attributes.add("points1d".to_string(), vec![0.0, 1.0, 2.0]);
    attributes.add("points2d".to_string(), vec![[0.0, 0.0], [1.0, 1.0], [2.0, 2.0]]);

    for &(ref name, ref data) in attributes.list.iter() {
        println!("\nI want to send this attribute to GL-land: {:?}", name);
        match data {
            &AttributeData::Vec1(ref data) => println!("Found a Vec1, do unsafe things with it {:?}", data),
            &AttributeData::Vec2(ref data) => println!("Found a Vec2, do unsafe things with it {:?}", data),
            _ => println!("Etc. for other types")
        }
    }

    /*
      Ultimately support the syntax for adding attributes:

      regl.draw()
          .attributes("id", vec![
              0.0,
              1.0,
              2.0,
              3.0
          ]);
          .attributes("position", vec![
              [0.0, 0.0, 0.0],
              [0.0, 0.0, 0.0],
              [0.0, 0.0, 0.0],
              [0.0, 0.0, 0.0]
          ]);
          .attributes("color", vec![
              [0.0, 0.0, 0.0, 0.0],
              [0.0, 0.0, 0.0, 0.0],
              [0.0, 0.0, 0.0, 0.0],
              [0.0, 0.0, 0.0, 0.0],
          ]);
    */
}
