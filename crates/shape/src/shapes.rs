use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;
use bevy_prototype_lyon::prelude::tess::path::path::Builder;

use super::shape::Shape;

pub trait DoubleShape<T1: Geometry<Builder>, T2: Geometry<Builder>>: Shape {
    fn get_shape1(&self) -> T1;
    fn get_shape2(&self) -> T2;
    fn _do_create(&self, commands: &mut Commands, entity: Entity) {
        let shape1 = self.get_shape1();
        let shape2 = self.get_shape2();
        let builder = ShapeBuilder::new()
            .add(&shape1)
            .add(&shape2);
        self.insert(commands, entity, builder);
    }
}
