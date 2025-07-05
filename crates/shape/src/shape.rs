use bevy::prelude::*;
use bevy_prototype_lyon::prelude::tess::path::builder::NoAttributes;
use bevy_prototype_lyon::prelude::tess::path::BuilderImpl;
use bevy_prototype_lyon::prelude::{*, Shape as LyonShape};
use bevy_prototype_lyon::prelude::tess::path::path::Builder;

pub trait Shape {
    fn get_transform(&self) -> Transform;
    fn get_fill(&self) -> Option<Fill> {
        None
    }
    fn get_stroke(&self) -> Option<Stroke> {
        None
    }
    fn _create(&self, commands: &mut Commands, entity: Entity);

    fn create(&self, commands: &mut Commands, parent: Entity) -> Entity {
        let entity = commands.spawn_empty().id();
        commands.entity(parent).add_children(&[entity]);
        self._create(commands, entity);
        entity
    }
    fn update(&self, commands: &mut Commands, entity: Entity) {
        commands.entity(entity).remove::<LyonShape>();
        self._create(commands, entity);
    }
    fn insert(&self, commands: &mut Commands, entity: Entity, builder: ShapeBuilder<NoAttributes<BuilderImpl>>) {
        let ready_builder =  match (self.get_fill(), self.get_stroke()) {
            (Some(fill), Some(stroke)) => builder.fill(fill).stroke(stroke),
            (Some(fill), None) => builder.fill(fill),
            (None, Some(stroke)) => builder.stroke(stroke),
            (None, None) => return,
        };
        let mut op = commands.entity(entity);
        op.insert(self.get_transform());
        op.insert(ready_builder.build());
    }
}

pub trait SingleShape<T: Geometry<Builder>>: Shape {
    fn get_shape(&self) -> T;
    fn _do_create(&self, commands: &mut Commands, entity: Entity) {
        let shape = self.get_shape();
        let builder = ShapeBuilder::with(&shape);
        self.insert(commands, entity, builder);
    }
}

pub trait ShapeOp<Env, S: Shape>: Clone + Component {
    fn get_shape(&self, env: &Env) -> S;
    fn create(&self, commands: &mut Commands, env: &Env, parent: Entity) -> Entity {
        let shape = self.get_shape(env);
        let shape_entity = shape.create(commands, parent);
        commands.entity(shape_entity).insert(self.clone());
        shape_entity
    }
    fn update(&self, commands: &mut Commands, env: &Env, entity: Entity) {
        let shape = self.get_shape(env);
        shape.update(commands, entity);
    }
}
