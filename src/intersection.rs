use crate::{color::{BLACK, Color}, ray::Ray, shape::Shape, tuple::Tuple, world::World};

#[derive(Clone)]
pub struct IntersectionContext<'a> {
    pub t: f64,
    pub shape: &'a dyn Shape,
    pub point: Tuple,
    pub eye_vector: Tuple,
    pub normal_vector: Tuple,
    pub reflect_vector: Tuple,
    pub inside: bool,
    pub over_point: Tuple,
    pub under_point: Tuple,
    pub n1: f64,
    pub n2: f64,
}

impl<'a> IntersectionContext<'a> {

  pub fn reflected_color(&self, world: &World, remaining: u8) -> Color {
    if self.shape.material().reflective == 0. || remaining == 0 {
        BLACK
    } else {
        let reflect_ray = Ray::new(self.over_point, self.reflect_vector);
        reflect_ray.color_at(world, remaining - 1) * self.shape.material().reflective
    }
}

  pub fn shade_hit(&self, world: &World, remaining: u8) -> Color {
      assert_eq!(world.lights.len(), 1);
      let in_shadow = world.is_shadowed(self.over_point);
      self.shape.material().lighting(
          &world.lights[0],
          self.shape,
          self.over_point,
          self.eye_vector,
          self.normal_vector,
          in_shadow,
      ) + self.reflected_color(world, remaining)
  }
}

impl<'a> PartialEq for IntersectionContext<'a> {
  fn eq(&self, other: &Self) -> bool {
      self.t == other.t
          && std::ptr::eq(self.shape.as_any(), other.shape.as_any())
          && self.point == other.point
          && self.eye_vector == other.eye_vector
          && self.normal_vector == other.normal_vector
          && self.reflect_vector == other.reflect_vector
          && self.inside == other.inside
          && self.over_point == other.over_point
          && self.under_point == other.under_point
          && self.n1 == other.n1
          && self.n2 == other.n2
  }
}

impl<'a> std::fmt::Debug for IntersectionContext<'a> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
      f.debug_struct("Intersection")
          .field("t", &self.t)
          .field("point", &self.point)
          .field("eye_vector", &self.eye_vector)
          .field("normal_vector", &self.normal_vector)
          .field("reflect_vector", &self.reflect_vector)
          .field("inside", &self.inside)
          .field("over_point", &self.over_point)
          .field("under_point", &self.under_point)
          .field("n1", &self.n1)
          .field("n2", &self.n2)
          .finish()
          .unwrap();
      f.write_fmt(format_args!("object {:?}", std::ptr::addr_of!(*self.shape)))
  }
}

#[cfg(test)]
mod tests {
  use crate::{color::BLACK, light::PointLight, material::Material, matrix::Matrix, plane::Plane, ray::Ray, shape::{Intersection, IntersectionList, MAX_REFLECTIONS}, sphere::Sphere};
  use super::*;

  #[test]
  fn reflection() {
      let m = Material::new();
      assert_eq!(m.reflective, 0.);

      let shape = Plane::new(None);
      let r = Ray::new(
          Tuple::point(0., 1., -1.),
          Tuple::vector(0., 2_f64.sqrt() / -2., 2_f64.sqrt() / 2.),
      );
      let i = r.intersect(&shape);
      assert_eq!(
          i.hit().unwrap().context(&r, None).reflect_vector,
          Tuple::vector(0., 2_f64.sqrt() / 2., 2_f64.sqrt() / 2.)
      );
  }


  #[test]
  fn reflect_color() {
      let mut w = World::default();
      let r = Ray::new(Tuple::point(0., 0., 0.), Tuple::vector(0., 0., 1.));
      w.objects[1].material_mut().ambient = 1.;
      let i = Intersection::new(1., w.objects[1].as_ref());
      assert_eq!(i.context(&r, None).reflected_color(&w, MAX_REFLECTIONS), BLACK);

      let mut w = World::default();
      let mut material = Material::new();
      material.reflective = 0.5;
      let mut shape = Plane::new(Some(material));
      shape.set_transform(&Matrix::translation(0., -1., 0.));
      w.objects.push(Box::new(shape));
      let r = Ray::new(
          Tuple::point(0., 0., -3.),
          Tuple::vector(0., 2_f64.sqrt() / -2., 2_f64.sqrt() / 2.),
      );
      let i = Intersection::new(2_f64.sqrt(), w.objects.last().unwrap());
      assert_eq!(
          i.context(&r, None).reflected_color(&w, MAX_REFLECTIONS),
          Color::new(0.190332, 0.237915, 0.14274)
      );
      assert_eq!(
          i.context(&r, None).shade_hit(&w, MAX_REFLECTIONS),
          Color::new(0.876757, 0.92434, 0.82917)
      );
  }

  #[test]
  fn infinite_reflection() {
      let mut material = Material::new();
      material.reflective = 1.;
      let mut lower = Plane::new(Some(material.clone()));
      lower.set_transform(&Matrix::translation(0., -1., 0.));

      let mut upper = Plane::new(Some(material.clone()));
      upper.set_transform(&Matrix::translation(0., 1., 0.));
      let r = Ray::new(Tuple::point(0., 0., 0.), Tuple::vector(0., 1., 0.));

      let w = World::new(
          vec![Box::new(lower), Box::new(upper)],
          vec![PointLight::new(
              Tuple::point(0., 0., 0.),
              Color::new(1., 1., 1.),
          )],
      );
      r.color_at(&w, MAX_REFLECTIONS);
  }

  #[test]
  fn refractive_indices() {
      let mut a = Sphere::glass_new();
      a.material_mut().refractive_index = 1.5;
      a.set_transform(&Matrix::scaling(2., 2., 2.));

      let mut b = Sphere::glass_new();
      b.material_mut().refractive_index = 2.;
      b.set_transform(&Matrix::translation(0., 0., -0.25));

      let mut c = Sphere::glass_new();
      c.material_mut().refractive_index = 2.5;
      c.set_transform(&Matrix::translation(0., 0., 0.25));

      let r = Ray::new(Tuple::point(0., 0., -4.), Tuple::vector(0., 0., 1.));
      let xs = IntersectionList::new(vec![
          Intersection::new(2., &a),
          Intersection::new(2.75, &b),
          Intersection::new(3.25, &c),
          Intersection::new(4.75, &b),
          Intersection::new(5.25, &c),
          Intersection::new(6., &a),
      ]);

      assert_eq!(xs.intersections[0].context(&r, Some(&xs)).n1, 1.);
      // assert_eq!(xs.intersections[0].n2, 1.5);
      // assert_eq!(xs.intersections[1].n1, 1.5);
      // assert_eq!(xs.intersections[1].n1, 2.);
  }
}
