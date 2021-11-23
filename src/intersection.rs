use crate::{material::Material, ray::Ray, tuple::Tuple};

pub trait Intersect {
    fn intersect<'a>(&'a self, ray: &Ray) -> IntersectionList<'a>;
    fn as_ref(&self) -> &dyn Intersect;
    fn normal(&self, point: &Tuple) -> Tuple;
    fn material(&self) -> &Material;
}

#[derive(Clone)]
pub struct Intersection<'a> {
    pub t: f64,
    pub object: Box<&'a dyn Intersect>,
}

impl<'a> Intersection<'a> {
    pub fn new(t: f64, object: &'a dyn Intersect) -> Intersection<'a> {
        Intersection {
            t,
            object: Box::new(object),
        }
    }
}

impl<'a> std::fmt::Debug for Intersection<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Intersection")
            .field("t", &self.t)
            .finish()
            .unwrap();
        f.write_fmt(format_args!(
            "object {:?}",
            std::ptr::addr_of!(*self.object)
        ))
    }
}

impl<'a> PartialEq for Intersection<'a> {
    fn eq(&self, other: &Self) -> bool {
        self.t == other.t && std::ptr::eq(*self.object, *other.object)
    }
}

pub struct IntersectionList<'a> {
    pub intersections: Vec<Intersection<'a>>,
}

impl<'a> IntersectionList<'a> {
    pub fn new(intersections: Vec<Intersection<'a>>) -> Self {
        let mut sorted_intersections = intersections;
        sorted_intersections.sort_by(|x, y| x.t.partial_cmp(&y.t).unwrap());
        IntersectionList {
            intersections: sorted_intersections,
        }
    }

    pub fn hit(&self) -> Option<&Intersection> {
        let filtered: Vec<_> = self.intersections.iter().filter(|x| x.t > 0.).collect();
        match filtered.len() {
            0 => None,
            _ => Some(&filtered[0]),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::sphere::Sphere;

    #[test]
    pub fn intersection() {
        let s = Sphere::new();
        let i = Intersection::new(3.5, &s);
        assert_eq!(i.t, 3.5);
        assert!(std::ptr::eq(*i.object as *const _, &s as *const _));
    }

    #[test]
    pub fn intersection_list() {
        let s = Sphere::new();
        let i1 = Intersection::new(1., &s);
        let i2 = Intersection::new(2., &s);
        let i = IntersectionList::new(vec![i1, i2]);
        assert_eq!(i.intersections.len(), 2);
        assert_eq!(i.intersections[0].t, 1.);
        assert_eq!(i.intersections[1].t, 2.);
        assert!(std::ptr::eq(
            *i.intersections[0].object as *const _,
            &s as *const _
        ));
        assert!(std::ptr::eq(
            *i.intersections[1].object as *const _,
            &s as *const _
        ));
    }

    #[test]
    pub fn hit() {
        let s = Sphere::new();
        let i1 = Intersection::new(1., &s);
        let i2 = Intersection::new(2., &s);
        let i = IntersectionList::new(vec![i1, i2]);
        assert_eq!(i.hit(), Some(&i.intersections[0]));

        let i1 = Intersection::new(-1., &s);
        let i2 = Intersection::new(1., &s);
        let i = IntersectionList::new(vec![i1.clone(), i2.clone()]);
        assert_eq!(i.hit(), Some(&i2));

        let i1 = Intersection::new(-2., &s);
        let i2 = Intersection::new(-1., &s);
        let i = IntersectionList::new(vec![i1, i2]);
        assert_eq!(i.hit(), None);

        let i1 = Intersection::new(5., &s);
        let i2 = Intersection::new(7., &s);
        let i3 = Intersection::new(-3., &s);
        let i4 = Intersection::new(2., &s);
        let i = IntersectionList::new(vec![i1.clone(), i2.clone(), i3.clone(), i4.clone()]);
        assert_eq!(i.hit(), Some(&i4));
    }
}
